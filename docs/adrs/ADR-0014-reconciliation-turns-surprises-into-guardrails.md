# ADR-0014: Reconciliation turns surprises into bounded guardrails

Status: Accepted and implemented

Date: 2026-09-10

Related: ADR-0003, ADR-0005, ADR-0011, ADR-0012, ADR-0013

## Context

Recording an evaluation verdict alone loses whether the system expected the
result, misunderstood metric direction, or failed to anticipate a failure mode.
The next cycle needs those differences in a form it can inspect, challenge, and
use only as advisory context.

## Decision

One matching evaluation outcome may reconcile one prediction. The store retains
both immutable input records. The reconciliation records verdict error,
directional metric errors, precision and recall over failure modes, false-accept
status, and new observed failure modes rendered as `investigate and bound`
guardrails. It does not rewrite the original prediction or outcome and it has
`authority: none`.

An absent expected metric counts as a directional error; a zero expected change
only agrees with a zero observed change. Empty predicted or observed failure
sets have zero precision or recall rather than an invented perfect score.

## Consequences

### Positive

- The system accumulates explicit, reviewable error signals.
- A missing measurement cannot masquerade as success.

### Negative

- Failure-mode vocabulary must be maintained with enough precision to be
  useful; generic labels reduce signal quality.

### Neutral

- Guardrails are proposals for the next frozen hypothesis, not policy rules or
  automatic candidate rejection.

## Test contract

- A changed hypothesis, candidate, or cycle cannot reconcile.
- False accepts and unexpected failure modes are retained.
- Reconciliations are one-way and cannot grant authority.

## Links

- [Evaluation bounded context](../ddd/evaluation.md)
- [Evidence Integrity bounded context](../ddd/evidence-integrity.md)
