# dream-machine-core

The Rust advisory core for Counterfactual Evidence Memory. It preserves a
simple authority boundary:

```text
freeze prediction -> independent evaluation -> reconcile error -> human review gate
```

A prediction records the expected verdict, metric directions, likely failure
modes, and source evidence **before** measurement. A reconciliation may be
created only once and only for the same cycle, hypothesis digest, and candidate
digest. The resulting error information is searchable context for the next
cycle; it cannot authorize promotion.

## Run

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Explicit non-goals

- no GitHub client, merge command, network listener, signing key, or actuator;
- no semantic/RVF/RuVector implementation claim;
- no multi-writer database or hostile-filesystem guarantee; and
- no conversion from a prediction or reconciliation into promotion evidence.

See [ADR-0011 through ADR-0014](../../docs/adrs/INDEX.md) and the
[DDD map](../../docs/ddd/README.md).
