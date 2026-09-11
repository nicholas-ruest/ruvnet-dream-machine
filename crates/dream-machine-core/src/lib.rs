//! A bounded Rust implementation of Dream Machine's counterfactual evidence
//! memory. It records predictions before an evaluation, reconciles them only
//! with an independent result, and deliberately has no merge or promotion API.

mod domain;
mod error;
mod memory;
mod promotion;

pub use domain::{
    Authority, CounterfactualPrediction, EvaluationOutcome, EvaluationReceipt, ExpectedMetricDelta,
    ObservedMetric, PredictionReconciliation, Verdict,
};
pub use error::MemoryError;
pub use memory::{
    CounterfactualEvidenceMemory, EvidenceHit, MemoryLimits, PersistentEvidenceMemory,
};
pub use promotion::{PromotionEligibility, PromotionGate};
