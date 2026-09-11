use crate::{
    Authority, CounterfactualPrediction, EvaluationOutcome, MemoryError, PredictionReconciliation,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryLimits {
    pub records: usize,
    pub query_bytes: usize,
    pub query_terms: usize,
    pub results: usize,
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self {
            records: 10_000,
            query_bytes: 4_096,
            query_terms: 128,
            results: 1_000,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct EvidenceRecord {
    prediction: CounterfactualPrediction,
    outcome: Option<EvaluationOutcome>,
    reconciliation: Option<PredictionReconciliation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceHit {
    pub prediction_id: String,
    pub prediction: CounterfactualPrediction,
    pub outcome: Option<EvaluationOutcome>,
    pub reconciliation: Option<PredictionReconciliation>,
    /// Exact ratio of matching query terms, expressed in basis points.
    pub relevance_basis_points: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Snapshot {
    version: u8,
    records: BTreeMap<String, EvidenceRecord>,
}

/// The in-process domain repository. It is deterministic, bounded, and has no
/// promotion or merge method by design.
#[derive(Clone, Debug)]
pub struct CounterfactualEvidenceMemory {
    limits: MemoryLimits,
    records: BTreeMap<String, EvidenceRecord>,
}

impl CounterfactualEvidenceMemory {
    pub fn new(limits: MemoryLimits) -> Self {
        Self {
            limits,
            records: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Creates a stable SHA-256 id from the explicit, serializable prediction.
    pub fn prediction_id(prediction: &CounterfactualPrediction) -> Result<String, MemoryError> {
        prediction.validate()?;
        let bytes = serde_json::to_vec(prediction)?;
        Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
    }

    pub fn record_prediction(
        &mut self,
        prediction: CounterfactualPrediction,
    ) -> Result<String, MemoryError> {
        let id = Self::prediction_id(&prediction)?;
        if self.records.contains_key(&id) {
            return Err(MemoryError::DuplicatePrediction(id));
        }
        if self.records.len() >= self.limits.records {
            return Err(MemoryError::CapacityExceeded);
        }
        self.records.insert(
            id.clone(),
            EvidenceRecord {
                prediction,
                outcome: None,
                reconciliation: None,
            },
        );
        Ok(id)
    }

    pub fn reconcile(
        &mut self,
        prediction_id: &str,
        outcome: EvaluationOutcome,
    ) -> Result<PredictionReconciliation, MemoryError> {
        outcome.validate()?;
        let record = self
            .records
            .get_mut(prediction_id)
            .ok_or_else(|| MemoryError::UnknownPrediction(prediction_id.into()))?;
        if record.reconciliation.is_some()
            || record.prediction.cycle_id != outcome.cycle_id
            || record.prediction.hypothesis_digest != outcome.hypothesis_digest
            || record.prediction.candidate_digest != outcome.candidate_digest
        {
            return Err(MemoryError::OutcomeDoesNotMatchPrediction(
                prediction_id.into(),
            ));
        }

        let actual_metrics: BTreeMap<_, _> = outcome
            .metrics
            .iter()
            .map(|metric| (metric.name.as_str(), metric.delta_basis_points))
            .collect();
        let metric_direction_errors = record
            .prediction
            .expected_metric_deltas
            .iter()
            .filter(|expected| {
                actual_metrics
                    .get(expected.name.as_str())
                    .is_none_or(|actual| {
                        direction(*actual) != direction(expected.delta_basis_points)
                    })
            })
            .map(|expected| expected.name.clone())
            .collect();

        let predicted: BTreeSet<_> = record
            .prediction
            .predicted_failure_modes
            .iter()
            .cloned()
            .collect();
        let observed: BTreeSet<_> = outcome.observed_failure_modes.iter().cloned().collect();
        let overlap = predicted.intersection(&observed).count() as u32;
        let precision = ratio_basis_points(overlap, predicted.len() as u32);
        let recall = ratio_basis_points(overlap, observed.len() as u32);
        let reusable_guardrails = observed
            .difference(&predicted)
            .map(|mode| format!("investigate and bound: {mode}"))
            .collect();

        let reconciliation = PredictionReconciliation {
            prediction_id: prediction_id.into(),
            verdict_error: record.prediction.predicted_verdict != outcome.actual_verdict,
            metric_direction_errors,
            failure_precision_basis_points: precision,
            failure_recall_basis_points: recall,
            false_accept: matches!(record.prediction.predicted_verdict, crate::Verdict::Accept)
                && !matches!(outcome.actual_verdict, crate::Verdict::Accept),
            reusable_guardrails,
            authority: Authority::None,
        };
        record.outcome = Some(outcome);
        record.reconciliation = Some(reconciliation.clone());
        Ok(reconciliation)
    }

    pub fn retrieve(&self, query: &str, k: usize) -> Result<Vec<EvidenceHit>, MemoryError> {
        if query.len() > self.limits.query_bytes {
            return Err(MemoryError::Validation("query exceeds byte limit".into()));
        }
        if k > self.limits.results {
            return Err(MemoryError::Validation("result count exceeds limit".into()));
        }
        let terms: BTreeSet<String> = query
            .split_whitespace()
            .map(|term| term.to_ascii_lowercase())
            .filter(|term| term.len() > 1)
            .collect();
        if terms.len() > self.limits.query_terms {
            return Err(MemoryError::Validation("query exceeds term limit".into()));
        }
        if terms.is_empty() || k == 0 {
            return Ok(Vec::new());
        }

        let mut hits: Vec<_> = self
            .records
            .iter()
            .filter_map(|(id, record)| {
                let corpus = searchable_text(record);
                let score = terms
                    .iter()
                    .filter(|term| corpus.contains(term.as_str()))
                    .count() as u32;
                (score > 0).then(|| EvidenceHit {
                    prediction_id: id.clone(),
                    prediction: record.prediction.clone(),
                    outcome: record.outcome.clone(),
                    reconciliation: record.reconciliation.clone(),
                    relevance_basis_points: ratio_basis_points(score, terms.len() as u32),
                })
            })
            .collect();
        hits.sort_by(|left, right| {
            right
                .relevance_basis_points
                .cmp(&left.relevance_basis_points)
                .then_with(|| left.prediction_id.cmp(&right.prediction_id))
        });
        hits.truncate(k);
        Ok(hits)
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            version: 1,
            records: self.records.clone(),
        }
    }
    fn from_snapshot(limits: MemoryLimits, snapshot: Snapshot) -> Result<Self, MemoryError> {
        if snapshot.version != 1 {
            return Err(MemoryError::Validation(
                "unsupported evidence memory version".into(),
            ));
        }
        if snapshot.records.len() > limits.records {
            return Err(MemoryError::CapacityExceeded);
        }
        for (id, record) in &snapshot.records {
            if Self::prediction_id(&record.prediction)? != *id {
                return Err(MemoryError::Validation(
                    "prediction id does not match content".into(),
                ));
            }
            if record
                .reconciliation
                .as_ref()
                .is_some_and(|item| item.authority != Authority::None)
            {
                return Err(MemoryError::Validation(
                    "reconciliation has invalid authority".into(),
                ));
            }
            match (&record.outcome, &record.reconciliation) {
                (None, None) => {}
                (Some(outcome), Some(reconciliation)) => {
                    outcome.validate()?;
                    if outcome.cycle_id != record.prediction.cycle_id
                        || outcome.hypothesis_digest != record.prediction.hypothesis_digest
                        || outcome.candidate_digest != record.prediction.candidate_digest
                        || reconciliation.prediction_id != *id
                    {
                        return Err(MemoryError::Validation(
                            "stored outcome or reconciliation does not match its prediction".into(),
                        ));
                    }
                }
                _ => return Err(MemoryError::Validation(
                    "stored outcome and reconciliation must either both exist or both be absent"
                        .into(),
                )),
            }
        }
        Ok(Self {
            limits,
            records: snapshot.records,
        })
    }
}

impl Default for CounterfactualEvidenceMemory {
    fn default() -> Self {
        Self::new(MemoryLimits::default())
    }
}

fn direction(value: i64) -> i8 {
    match value.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}
fn ratio_basis_points(numerator: u32, denominator: u32) -> u16 {
    if denominator == 0 {
        return 0;
    }
    ((numerator.saturating_mul(10_000) / denominator).min(10_000)) as u16
}

fn searchable_text(record: &EvidenceRecord) -> String {
    let prediction = &record.prediction;
    format!(
        "{} {} {} {} {} {}",
        prediction.cycle_id,
        prediction.hypothesis_digest,
        prediction.candidate_digest,
        prediction.predicted_failure_modes.join(" "),
        prediction.evidence_refs.join(" "),
        record
            .reconciliation
            .as_ref()
            .map(|item| item.reusable_guardrails.join(" "))
            .unwrap_or_default()
    )
    .to_ascii_lowercase()
}

/// A local JSON repository intended for the control plane's own state. The
/// caller decides when to persist; a successful save uses write/sync/rename so
/// readers observe either the old complete snapshot or the new complete one.
pub struct PersistentEvidenceMemory {
    path: PathBuf,
    memory: CounterfactualEvidenceMemory,
}

impl PersistentEvidenceMemory {
    pub fn open(path: impl AsRef<Path>, limits: MemoryLimits) -> Result<Self, MemoryError> {
        let path = path.as_ref().to_path_buf();
        let memory = match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(MemoryError::Validation(
                    "evidence memory path must not be a symlink".into(),
                ));
            }
            Ok(_) => CounterfactualEvidenceMemory::from_snapshot(
                limits,
                serde_json::from_slice(&fs::read(&path)?)?,
            )?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                CounterfactualEvidenceMemory::new(limits)
            }
            Err(error) => return Err(error.into()),
        };
        Ok(Self { path, memory })
    }

    pub fn memory(&self) -> &CounterfactualEvidenceMemory {
        &self.memory
    }
    pub fn memory_mut(&mut self) -> &mut CounterfactualEvidenceMemory {
        &mut self.memory
    }

    pub fn save(&self) -> Result<(), MemoryError> {
        let parent = self
            .path
            .parent()
            .ok_or_else(|| MemoryError::Validation("evidence memory path has no parent".into()))?;
        fs::create_dir_all(parent)?;
        let metadata = fs::symlink_metadata(parent)?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(MemoryError::Validation(
                "evidence memory parent must be a real directory".into(),
            ));
        }
        let temporary = parent.join(format!(
            ".{}.{}.tmp",
            self.path.file_name().unwrap_or_default().to_string_lossy(),
            std::process::id()
        ));
        let bytes = serde_json::to_vec(&self.memory.snapshot())?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        let write_result = (|| -> Result<(), MemoryError> {
            file.write_all(&bytes)?;
            file.sync_all()?;
            Ok(())
        })();
        drop(file);
        if let Err(error) = write_result {
            let _ = fs::remove_file(&temporary);
            return Err(error);
        }
        if fs::symlink_metadata(&self.path).is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            let _ = fs::remove_file(&temporary);
            return Err(MemoryError::Validation(
                "evidence memory path must not be a symlink".into(),
            ));
        }
        fs::rename(temporary, &self.path)?;
        Ok(())
    }
}
