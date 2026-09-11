# Dream Cycle — 2026-09-10: Counterfactual Evidence Memory

## Verdict

**INCONCLUSIVE for autonomous selection; implementation complete for human
review.** The requested Rust candidate is implemented and all repository and
Rust checks pass. The configured Darwin mock run completed, but its generic
harness leaderboard did not evaluate comparable Counterfactual Evidence Memory
implementations. It therefore cannot truthfully select a production winner.

This draft is deliberately not a merge recommendation.

## Publication

- Draft PR: https://github.com/nicholas-ruest/ruvnet-dream-machine/pull/1
- Public Dream Brief: https://gist.github.com/nicholas-ruest/670c6a922f381d7167d722b3b6bb133f
- Issue tracker: disabled for this repository, so no dream-cycle issue could be created.

## Original context

Dream Machine can preserve prior-night material through a bounded TypeScript
keyword store, but it could not record a pre-evaluation forecast, bind it to
the exact evaluated candidate, calculate the prediction error, or retain a
failure boundary as reusable advisory context. The missed 9:30 PM cycle
proposed this direction but left it as documentation only.

## Frozen hypothesis

If Dream Machine records an immutable, `authority: none` prediction before an
independent evaluation and reconciles it only against the same cycle,
hypothesis, and candidate digests, it can retain reviewable forecast errors and
failure guardrails without allowing a model forecast to satisfy an evaluation
or promotion gate.

## Shared implementation contract

All candidates must provide the following invariants:

1. Record a validated prediction with a content-derived stable identifier.
2. Reconcile at most one matching independent outcome.
3. Retain verdict error, metric-direction errors, failure precision/recall,
   false-accept state, and new guardrail prompts.
4. Keep prediction and reconciliation authority at `none`.
5. Bound records, queries, and result counts; fail closed at the limits.
6. Never offer merge, write-to-GitHub, or promotion authority.

## Candidate decision

| Candidate | Result | Rationale |
| --- | --- | --- |
| A — Rust bounded core with versioned JSON snapshots | **Implemented** | Typed domain model, stable SHA-256 IDs, deterministic retrieval, atomic local replacement, and explicit human-review gate. |
| B — Extend the published TypeScript memory package | Not selected | Does not satisfy the explicit Rust implementation requirement and would conflate the new authority domain with the existing public API. |
| C — Direct RVF/RuVector adapter | Not selected | The adapter persistence/query contract is not proven in this environment; claiming it would violate the existing optional-backend boundary. |

The implementation is Candidate A in `crates/dream-machine-core`. Its decision
records are ADR-0011 through ADR-0014 and its bounded contexts are Cycle
Planning, Evidence Memory, Evaluation, Evidence Integrity, and Promotion.

## Evidence

| Gate | Observed result |
| --- | --- |
| Rust format, test, clippy | PASS — six integration tests; warnings denied |
| Existing repository check | PASS — TypeScript typecheck/build/lint, 616 unit tests, 81 governance tests, Edge v1 validation, development-policy check |
| Fresh-checkout doctor regression | PASS — fixed macOS `/tmp` versus `/private/tmp` ESM entrypoint detection; the test now also correctly handles the intentional `INCONCLUSIVE` exit code on an unsupported Node runtime |
| Darwin mock | COMPLETED — generic mock winner `g2_v5`, +0.110 over its baseline; **not comparable to this candidate set** |
| Independent MetaHarness candidate comparison | NOT AVAILABLE — no comparable multi-candidate measurement, so no autonomous winner claim |

## Scope and guardrails

- No existing TypeScript public API is replaced.
- The JSON store is a bounded local control-plane store, not RVF, an encrypted
  vault, a hostile-filesystem defence, or a multi-writer database.
- Only an independent successful evaluation may be *ready for human review*;
  nothing in this change merges or promotes code.
- The CI workflow now validates the Rust core on the Node 22 matrix lane.

## Next measured step

Implement an independently comparable second Rust storage adapter behind the
same contract and evaluate both adapters on a frozen corpus plus an actual
MetaHarness-compatible metric suite. Only then may the nightly process call one
the selected winner.
