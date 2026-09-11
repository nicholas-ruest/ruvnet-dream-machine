use crate::MemoryError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Verdict {
    Accept,
    Reject,
    Inconclusive,
}

/// A value that documents that a record can advise but cannot authorize action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Authority {
    #[serde(rename = "none")]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExpectedMetricDelta {
    pub name: String,
    pub delta_basis_points: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservedMetric {
    pub name: String,
    pub delta_basis_points: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CounterfactualPrediction {
    pub cycle_id: String,
    pub base_commit: String,
    pub hypothesis_digest: String,
    pub candidate_digest: String,
    pub predicted_verdict: Verdict,
    pub confidence_basis_points: u16,
    pub expected_metric_deltas: Vec<ExpectedMetricDelta>,
    pub predicted_failure_modes: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub authority: Authority,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluationOutcome {
    pub cycle_id: String,
    pub hypothesis_digest: String,
    pub candidate_digest: String,
    pub actual_verdict: Verdict,
    pub metrics: Vec<ObservedMetric>,
    pub observed_failure_modes: Vec<String>,
    pub evaluation_receipt: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PredictionReconciliation {
    pub prediction_id: String,
    pub verdict_error: bool,
    pub metric_direction_errors: Vec<String>,
    pub failure_precision_basis_points: u16,
    pub failure_recall_basis_points: u16,
    pub false_accept: bool,
    pub reusable_guardrails: Vec<String>,
    pub authority: Authority,
}

/// Evidence that may be considered by a human review gate. A prediction or
/// reconciliation intentionally cannot be converted into this type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvaluationReceipt {
    pub receipt_id: String,
    pub verdict: Verdict,
    pub independent_evaluator: bool,
    pub tests_passed: bool,
    pub security_checks_passed: bool,
}

fn validate_nonempty(name: &str, value: &str) -> Result<(), MemoryError> {
    if value.trim().is_empty() {
        return Err(MemoryError::Validation(format!("{name} must not be empty")));
    }
    Ok(())
}

impl CounterfactualPrediction {
    pub fn validate(&self) -> Result<(), MemoryError> {
        for (name, value) in [
            ("cycle_id", &self.cycle_id),
            ("base_commit", &self.base_commit),
            ("hypothesis_digest", &self.hypothesis_digest),
            ("candidate_digest", &self.candidate_digest),
        ] {
            validate_nonempty(name, value)?;
        }
        if self.confidence_basis_points > 10_000 {
            return Err(MemoryError::Validation(
                "confidence_basis_points must be <= 10000".into(),
            ));
        }
        for metric in &self.expected_metric_deltas {
            validate_nonempty("expected metric name", &metric.name)?;
        }
        for mode in &self.predicted_failure_modes {
            validate_nonempty("predicted failure mode", mode)?;
        }
        for reference in &self.evidence_refs {
            validate_nonempty("evidence reference", reference)?;
        }
        if self.authority != Authority::None {
            return Err(MemoryError::Validation(
                "predictions must have authority=none".into(),
            ));
        }
        Ok(())
    }
}

impl EvaluationOutcome {
    pub fn validate(&self) -> Result<(), MemoryError> {
        for (name, value) in [
            ("cycle_id", &self.cycle_id),
            ("hypothesis_digest", &self.hypothesis_digest),
            ("candidate_digest", &self.candidate_digest),
            ("evaluation_receipt", &self.evaluation_receipt),
        ] {
            validate_nonempty(name, value)?;
        }
        for metric in &self.metrics {
            validate_nonempty("observed metric name", &metric.name)?;
        }
        for mode in &self.observed_failure_modes {
            validate_nonempty("observed failure mode", mode)?;
        }
        Ok(())
    }
}
