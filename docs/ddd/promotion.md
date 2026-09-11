# Bounded context: Promotion

## Responsibility

Determine whether an independently evaluated candidate is ready for a **human**
review. This context does not merge or mutate a repository.

## Aggregate

`PromotionGate` accepts only `EvaluationReceipt`, a distinct type from every
prediction and reconciliation.

## Invariants

- Receipt ID is nonempty.
- The evaluator is independent.
- Tests and security checks passed.
- Verdict is `ACCEPT`.
- The only positive result is `ReadyForHumanReview`.

## Published language

`CandidateReadyForHumanReview { receipt_id }` or `CandidateIneligible { reason }`

## Integration boundary

It does not read advisory predictions. An integration to GitHub or another
change system requires a separate ADR and a separately authorized human action.
