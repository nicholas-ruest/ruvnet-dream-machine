use dream_machine_core::{
    Authority, CounterfactualEvidenceMemory, EvaluationOutcome, EvaluationReceipt,
    ExpectedMetricDelta, MemoryError, MemoryLimits, ObservedMetric, PromotionEligibility,
    PromotionGate, Verdict,
};

fn prediction() -> dream_machine_core::CounterfactualPrediction {
    dream_machine_core::CounterfactualPrediction {
        cycle_id: "DM-2026-09-10-001".into(),
        base_commit: "3edd426".into(),
        hypothesis_digest: "sha256:hypothesis".into(),
        candidate_digest: "sha256:candidate".into(),
        predicted_verdict: Verdict::Accept,
        confidence_basis_points: 7_500,
        expected_metric_deltas: vec![ExpectedMetricDelta {
            name: "recall_at_5".into(),
            delta_basis_points: 2_000,
        }],
        predicted_failure_modes: vec!["embedding drift".into()],
        evidence_refs: vec!["NVIDIA_SKILLEVALUATOR".into()],
        authority: Authority::None,
    }
}

fn outcome() -> EvaluationOutcome {
    EvaluationOutcome {
        cycle_id: "DM-2026-09-10-001".into(),
        hypothesis_digest: "sha256:hypothesis".into(),
        candidate_digest: "sha256:candidate".into(),
        actual_verdict: Verdict::Reject,
        metrics: vec![ObservedMetric {
            name: "recall_at_5".into(),
            delta_basis_points: -100,
        }],
        observed_failure_modes: vec!["embedding drift".into(), "corpus mismatch".into()],
        evaluation_receipt: "receipt-1".into(),
    }
}

#[test]
fn reconciliation_retains_errors_but_has_no_authority() {
    let mut memory = CounterfactualEvidenceMemory::default();
    let id = memory.record_prediction(prediction()).unwrap();
    let reconciliation = memory.reconcile(&id, outcome()).unwrap();
    assert!(reconciliation.verdict_error);
    assert_eq!(reconciliation.metric_direction_errors, ["recall_at_5"]);
    assert_eq!(reconciliation.failure_precision_basis_points, 10_000);
    assert_eq!(reconciliation.failure_recall_basis_points, 5_000);
    assert!(reconciliation.false_accept);
    assert_eq!(reconciliation.authority, Authority::None);
    assert_eq!(
        reconciliation.reusable_guardrails,
        ["investigate and bound: corpus mismatch"]
    );
}

#[test]
fn reconciliation_rejects_a_result_for_a_different_candidate() {
    let mut memory = CounterfactualEvidenceMemory::default();
    let id = memory.record_prediction(prediction()).unwrap();
    let mut wrong = outcome();
    wrong.candidate_digest = "sha256:other".into();
    assert!(matches!(
        memory.reconcile(&id, wrong),
        Err(MemoryError::OutcomeDoesNotMatchPrediction(_))
    ));
}

#[test]
fn retrieval_is_deterministic_and_bounded() {
    let mut memory = CounterfactualEvidenceMemory::default();
    memory.record_prediction(prediction()).unwrap();
    let hits = memory.retrieve("NVIDIA embedding", 1).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].relevance_basis_points, 10_000);
    let oversized = (0..129)
        .map(|index| format!("term{index}"))
        .collect::<Vec<_>>()
        .join(" ");
    assert!(matches!(
        memory.retrieve(&oversized, 1),
        Err(MemoryError::Validation(_))
    ));
}

#[test]
fn memory_cap_fails_closed() {
    let mut memory = CounterfactualEvidenceMemory::new(MemoryLimits {
        records: 1,
        ..MemoryLimits::default()
    });
    memory.record_prediction(prediction()).unwrap();
    let mut another = prediction();
    another.cycle_id = "DM-2026-09-10-002".into();
    assert!(matches!(
        memory.record_prediction(another),
        Err(MemoryError::CapacityExceeded)
    ));
}

#[test]
fn only_an_independent_receipt_can_reach_human_review() {
    let receipt = EvaluationReceipt {
        receipt_id: "receipt-1".into(),
        verdict: Verdict::Accept,
        independent_evaluator: true,
        tests_passed: true,
        security_checks_passed: true,
    };
    assert!(matches!(
        PromotionGate::assess(&receipt),
        PromotionEligibility::ReadyForHumanReview { .. }
    ));
    let untrusted = EvaluationReceipt {
        independent_evaluator: false,
        ..receipt
    };
    assert!(matches!(
        PromotionGate::assess(&untrusted),
        PromotionEligibility::Ineligible { .. }
    ));
}

#[test]
fn local_store_reopens_an_atomic_snapshot() {
    let path = std::env::temp_dir().join(format!(
        "dream-machine-memory-{}-{}.json",
        std::process::id(),
        7
    ));
    let _ = std::fs::remove_file(&path);
    let mut store =
        dream_machine_core::PersistentEvidenceMemory::open(&path, MemoryLimits::default()).unwrap();
    let id = store.memory_mut().record_prediction(prediction()).unwrap();
    store.memory_mut().reconcile(&id, outcome()).unwrap();
    store.save().unwrap();
    let reopened =
        dream_machine_core::PersistentEvidenceMemory::open(&path, MemoryLimits::default()).unwrap();
    let hits = reopened.memory().retrieve("corpus mismatch", 1).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(
        hits[0].outcome.as_ref().unwrap().evaluation_receipt,
        "receipt-1"
    );
    std::fs::remove_file(path).unwrap();
}
