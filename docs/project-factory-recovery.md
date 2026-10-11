# Factory failure recovery — October 11, 2026

## Verified failure
The canonical ChatGPT Project Factory automation already required multi-crate systems,
approximately 25 ADRs and 12 DDDs before this repair.
Current public repository trees nevertheless show:
- proposal-sieve@04114b526ebe511c2145c39fd48a0bea2ffcd5e1: one Cargo package, src/lib.rs and src/main.rs, zero ADRs and zero DDDs.
- epoch-envelope@f31f52a11adff4001f2019bfed508340ca4f7045: the same structural failure.

Instruction changes alone did not establish execution enforcement. The exact scheduled
executor transcript is not exposed here; do not claim a particular executor ignored a rule
without that evidence. The repo's dream-nightly GitHub workflow is explicitly research-only
and is not the standalone Project Factory implementation.

Live Core Memory task_start in namespace dream-machine returned no playbook entries or
anti-patterns and unscreened=true. That is not evidence that Nick's build preferences were
retrieved. Repository AGENTS.md was separately read.

## Runnable checkpoint
Run against the intended generated project checkout before public repo creation or push:

```bash
node /path/to/ruvnet-dream-machine/scripts/check-project-contract.mjs /path/to/candidate
```

Exit 1 means NONCOMPLIANT_BUILD; exit 2 means PRECHECK_BLOCKED.
Exit 0 means STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW, never BUILT.

The gate checks 25 ADR documents, 12 DDD documents, basic document sections and
duplicate bodies, required architecture/research/contract/guidance/integration/validation
documents, twenty implemented crate directories, workspace declaration, lockfile, integration
test sources, and two animated SVG assets. Counts are minimums for this recovery contract;
substantive review remains mandatory. Do not add filler to satisfy counts.

## Updated authority — October 11 follow-up
Nick explicitly requires at least 20 meaningful implemented workspace crates in every generated project. The canonical positive build instructions are in core-memory/docs/guidance/dream-machine-build-standard.md and surfaced by core-memory/AGENTS.md. This supersedes the initial four-crate floor.

## Execution order
1. Select a substantive system and pin upstream ingredients.
2. Commit specification, architecture, research, ADRs, DDDs, integration matrix and
   implementation contract on a private working branch before implementation.
3. Implement across bounded contexts with real executed integrations.
4. Run this gate and all required build/integration/security/benchmark/ablation checks.
5. Inspect exact source and raw execution evidence independently.
6. Publish only within existing authorization after both structural and evidence checks.

If time expires, persist the branch/checkpoint and resume it next run. Do not create another
utility repository. If the checker cannot be loaded or executed, publication remains blocked.

## Limits and activation
This is structural defense in depth. It cannot prove semantic quality, workspace member
linkage, upstream runtime execution, benchmark validity, or protected publication decisions.
A ChatGPT instruction to invoke it is still instruction-based orchestration; this patch does
not claim server-side interception of every GitHub mutation.
The regression workflow tests the checker itself; it is not a generated-project quality pass.
The automation must use the exact reviewed source commit containing the checker until
the change is merged, then follow the verified merged version.
