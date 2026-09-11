use crate::{EvaluationReceipt, Verdict};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PromotionEligibility {
    Ineligible { reason: &'static str },
    ReadyForHumanReview { receipt_id: String },
}

/// This gate only identifies a reviewable independent evaluation. It does not
/// merge a branch, alter a repository, or accept predictions as evidence.
pub struct PromotionGate;

impl PromotionGate {
    pub fn assess(receipt: &EvaluationReceipt) -> PromotionEligibility {
        if receipt.receipt_id.trim().is_empty() {
            return PromotionEligibility::Ineligible {
                reason: "evaluation receipt id is required",
            };
        }
        if !receipt.independent_evaluator {
            return PromotionEligibility::Ineligible {
                reason: "an independent evaluator is required",
            };
        }
        if !receipt.tests_passed || !receipt.security_checks_passed {
            return PromotionEligibility::Ineligible {
                reason: "tests and security checks must pass",
            };
        }
        if receipt.verdict != Verdict::Accept {
            return PromotionEligibility::Ineligible {
                reason: "only an ACCEPT evaluation can be reviewed",
            };
        }
        PromotionEligibility::ReadyForHumanReview {
            receipt_id: receipt.receipt_id.clone(),
        }
    }
}
