# Bounded context: Evaluation

## Responsibility

Provide an independently measured `EvaluationOutcome`: the actual verdict,
observed metric deltas, observed failure modes, and an evaluator receipt
reference.

## Aggregate

`EvaluationOutcome` is an input value. The current Rust tranche validates and
matches it but deliberately does not run benchmarks: evaluators remain external
to the advisory memory core.

## Invariants

- Cycle, hypothesis, candidate, and receipt identifiers are nonempty.
- Metrics and failure modes are named explicitly.
- Evaluation never mutates a frozen prediction.
- The outcome is not sufficient for promotion by itself.

## Published language

`EvaluationCompleted { cycle_id, candidate_digest, verdict, receipt }`

## Integration boundary

Evidence Integrity consumes outcomes only when their three immutable identities
match the prediction. Promotion consumes a separately typed receipt.
