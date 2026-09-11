# Bounded context: Evidence Integrity

## Responsibility

Bind a prediction to its own result, calculate the observable mismatch, and
retain surprises without changing the original evidence.

## Aggregate

`PredictionReconciliation` contains verdict error, metric-direction errors,
failure precision/recall, false-accept status, and candidate guardrail prompts.

## Invariants

- A result must match the prediction's cycle, hypothesis digest, and candidate
  digest exactly.
- A prediction has at most one reconciliation.
- Missing expected metrics are errors, not silent successes.
- Reconciliation authority is always `none`.

## Published language

`PredictionReconciled { prediction_id, verdict_error, false_accept }`

## Integration boundary

This context updates Evidence Memory. It cannot make an evaluator result pass,
change a verdict, create an approval, or write a branch.
