# Bounded context: Evidence Memory

## Responsibility

Persist and retrieve predictions plus their later reconciliation under fixed
resource bounds.

## Aggregate

`CounterfactualEvidenceMemory` is the repository of `EvidenceRecord`s keyed by
the SHA-256 prediction ID. `PersistentEvidenceMemory` is the local JSON adapter.

## Invariants

- Maximum record, query, term, and result limits fail closed.
- Duplicate content IDs fail closed.
- Retrieval is exact, deterministic keyword relevance followed by prediction-ID
  tie breaking.
- A snapshot version and every prediction ID are checked on reopen.
- Storage is advisory context only; it has no promotion method.

## Published language

`PredictionStored`, `EvidenceRetrieved`, `SnapshotPersisted`

## Integration boundary

It consumes a prediction from Cycle Planning and a reconciliation from Evidence
Integrity. It must not infer an outcome or rewrite the independent evaluator.
