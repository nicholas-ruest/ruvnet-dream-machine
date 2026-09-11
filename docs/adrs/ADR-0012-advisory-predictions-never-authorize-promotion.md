# ADR-0012: Advisory predictions never authorize promotion

Status: Accepted and implemented

Date: 2026-09-10

Related: ADR-0001, ADR-0004, ADR-0007, ADR-0010, ADR-0011

## Context

A forecast that a candidate will pass, or a history of apparently accurate
forecasts, is not an evaluation. If an optimizing system could use its own
prediction as proof, it could improve the appearance of success without
improving the repository. This is the precise Goodhart path the Dream Machine
must exclude.

## Decision

Every `CounterfactualPrediction` and `PredictionReconciliation` carries the
only permitted authority value: `none`. The Rust type does not offer another
variant. Reconciliation requires the original cycle ID, hypothesis digest, and
candidate digest to equal the independent outcome, and it may be written once.

`PromotionGate` does not accept predictions or reconciliations. It considers
only an `EvaluationReceipt` with all of these conditions:

1. a nonempty receipt identifier;
2. an independent evaluator;
3. passing tests;
4. passing security checks; and
5. an `ACCEPT` verdict.

Even then it returns `ReadyForHumanReview`, not a merge command or a permission
to modify a repository.

## Consequences

### Positive

- Forecasts can guide bounded search and preserve failure knowledge without
  crossing the evaluation or promotion boundary.
- A changed candidate, hypothesis, or cycle cannot borrow an older outcome.

### Negative

- More input must be recorded before a candidate reaches human review.
- The initial scorer remains intentionally simple and advisory.

### Neutral

- A real evaluator and a human still decide whether any change is promoted.

## Test contract

- A mismatched candidate outcome is rejected.
- A false `ACCEPT` forecast is retained as a `false_accept` error.
- An evaluator that is not independent is ineligible for human review.

## Links

- [Promotion bounded context](../ddd/promotion.md)
- [Core policy implementation](../../crates/dream-machine-core/src/promotion.rs)
