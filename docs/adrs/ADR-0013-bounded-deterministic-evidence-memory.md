# ADR-0013: Bounded deterministic evidence memory

Status: Accepted and implemented

Date: 2026-09-10

Related: ADR-0003, ADR-0007, ADR-0010, ADR-0011

## Context

The prior memory package proves useful bounded flat-file behaviour, but an
unbounded or opaque new memory subsystem would weaken replay and security
properties. Counterfactual evidence should help find related failures without
becoming an uninspectable decision maker.

## Decision

The Rust core stores at most 10,000 records by default. Queries are bounded to
4,096 bytes, 128 meaningful terms, and 1,000 returned hits. Retrieval is exact
keyword matching over explicit cycle, digest, failure-mode, evidence-reference,
and learned-guardrail fields. Results sort by descending relevance and then the
stable content-derived prediction ID.

The persistent repository serializes a versioned JSON snapshot. Saving writes a
temporary file, syncs it, and renames it into place; readers get a complete old
or new file after a successful rename. It rejects a final symlink and a symlink
parent. This is deliberately a single-control-plane-writer store, not a
hostile-filesystem defence, encrypted vault, or RVF implementation.

## Consequences

### Positive

- Search results, resource use, and tie breaking are reproducible.
- Stable SHA-256 IDs expose tampering or accidental mismatch in serialized
  prediction content.
- Storage can be reopened and tested locally now.

### Negative

- Keyword retrieval does not claim semantic retrieval or RuVector/RVF support.
- Concurrent writers are out of scope until an explicitly reviewed repository
  contract exists.

### Neutral

- A future RVF adapter must satisfy the same ID, ordering, capacity, reopen,
  and authority contracts before it can replace this store.

## Test contract

- Query and record caps reject excess input.
- Snapshot reopen returns recorded evidence.
- Retrieval tie breaking remains deterministic.

## Links

- [Evidence Memory bounded context](../ddd/evidence-memory.md)
- [Persistence implementation](../../crates/dream-machine-core/src/memory.rs)
