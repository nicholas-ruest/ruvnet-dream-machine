# Bounded context: Cycle Planning

## Responsibility

Freeze the exact thing that will later be measured: `cycle_id`, base commit,
hypothesis digest, candidate digest, anticipated verdict, metric directions,
anticipated failure modes, and evidence references.

## Aggregate

`CounterfactualPrediction` is immutable after `record_prediction`. Its
content-derived SHA-256 ID is the identity passed to reconciliation.

## Invariants

- All identity fields are nonempty.
- Confidence is 0–10,000 basis points.
- `authority` is always `none`.
- Planning cannot attach an evaluation outcome, create a receipt, or request
  promotion.

## Published language

`PredictionRecorded { prediction_id, cycle_id, candidate_digest }`

## Integration boundary

The context writes to Evidence Memory. It accepts the independent candidate and
hypothesis digests; it does not generate them or execute code.
