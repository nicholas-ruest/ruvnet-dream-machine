# ADR-0011: Rust counterfactual evidence core

Status: Accepted and implemented

Date: 2026-09-10

Related: ADR-0001, ADR-0003, ADR-0004, ADR-0007, ADR-0010

## Context

The scheduled Dream Machine cycle proposed Counterfactual Evidence Memory but
delivered no runtime. The existing workspace is TypeScript and has a bounded
flat-file keyword memory package, but it has no executable representation for a
prediction made before evaluation, a matched independent outcome, or the error
between the two. A design-only record would repeat the same failure.

## Decision

Add `crates/dream-machine-core`, an isolated Rust 2024 library. It implements
the advisory evidence core and local persistence without changing the published
TypeScript CLI or granting any new GitHub, merge, actuator, signing, or network
authority.

The core exposes only:

- validated `CounterfactualPrediction` recording;
- stable SHA-256 prediction identifiers;
- outcome matching and deterministic reconciliation;
- bounded deterministic keyword retrieval over evidence metadata;
- local JSON snapshot reopen/save; and
- a review-readiness policy that cannot merge anything.

`cargo test --workspace` is the required executable contract for this Rust
tranche. The crate has no runtime network dependency and uses only `serde`,
`serde_json`, and `sha2`.

## Consequences

### Positive

- The proposal now has a real, type-checked, tested implementation.
- Rust separates the authority-sensitive evidence domain from the existing
  Node-facing orchestration surface.
- The implementation can be integrated with the TypeScript CLI through a
  future explicit adapter, instead of silently rewriting the public API.

### Negative

- This is a parallel core, not a completed migration of every TypeScript
  package. Any adapter must preserve the data and authority invariants below.
- Local JSON persistence is a control-plane store, not a multi-writer database,
  authenticated log, RVF container, or encrypted archive.

### Neutral

- Existing TypeScript packages remain the published interface and continue to
  own current schedule/CLI behaviour.

## Test contract

- Duplicate content-derived prediction IDs fail closed.
- Memory capacity and query/result limits fail closed.
- A reopened snapshot preserves reconciled evidence.
- A prediction or reconciliation never becomes promotion evidence.

## Links

- [Rust core](../../crates/dream-machine-core/)
- [DDD context map](../ddd/README.md)
