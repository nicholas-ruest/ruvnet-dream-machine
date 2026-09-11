# Counterfactual Evidence Memory — DDD map

This is the bounded domain model implemented by
[`dream-machine-core`](../../crates/dream-machine-core/). It deliberately does
not model GitHub mutation, merge, signing, model invocation, or hardware action.

| Bounded context | Owns | Cannot authorize |
| --- | --- | --- |
| [Cycle Planning](./cycle-planning.md) | frozen hypothesis, candidate identity, prediction | evaluation, promotion, merge |
| [Evidence Memory](./evidence-memory.md) | bounded records, retrieval, persistence | semantic/RVF claims, promotion |
| [Evaluation](./evaluation.md) | independently produced outcome and receipt reference | changing the frozen prediction |
| [Evidence Integrity](./evidence-integrity.md) | content IDs, match checks, reconciliation | accepting a candidate |
| [Promotion](./promotion.md) | human-review readiness from a real receipt | merge, auto-promotion, repository mutation |

## Context flow

```text
Cycle Planning --prediction--> Evidence Memory
Evaluation --outcome/receipt--> Evidence Integrity --reconciliation--> Evidence Memory
Evaluation receipt -----------------------------------------------> Promotion --review only--> Human
```

All cross-context records are plain, serializable values. `authority: none` is
an invariant of predictions and reconciliations, not a convention.
