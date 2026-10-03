# Dream Machine factory recovery

Status: executor repaired; complete research-to-new-project publication remains unverified.
This is infrastructure recovery, not a nightly project deliverable.

## Verified observations — October 3, 2026

- The connected GitHub identity is `nicholas-ruest`, with admin access to this control-plane repository.
- Current core-memory commit read: `515c6445e28e75d33ebe75f34ea3e8f3a31a2361`.
- Read AGENTS.md (`4afe6f9008076140486d2a1e5424ddc6e40a0671`), CLAUDE.md
  (`0178167276c981ff0ba503e2e564eee8d347e3f7`), architecture overview, storage-adapter ADR and governance DDD.
- Applied decisions: source-bound evidence; separate coordination from execution;
  no self-promotion; existing branch/approval rules; isolated source scopes;
  fail-loud adapter boundaries and no exposure of private repository contents.
- RuOS machine `8ee959f7363638` returned fresh heartbeats and executor-owned completion receipts.
- Rust was installed outside PATH: `/home/ruv/.cargo/bin`. Cargo/rustc 1.99.0,
  Clippy 0.1.99 and rustfmt 1.10.0 were executed successfully.
- A temporary Rust runtime probe completed formatting, strict Clippy and a locked test.
  Its one generated unit test establishes toolchain execution only, not project quality.
- Node 24.21.0 was installed in a user-scoped directory after checking the official
  archive SHA-256 `fd8e59d5a511510f6a298afb548f18c7d2b1be404d8b4a27d94fbe49f56cb2d6`.
- Pinned Darwin 0.10.3 and Flywheel 0.1.12 npm artifacts are installed under
  `/home/ruv/.local/share/dream-machine/evaluators/node_modules`. Darwin CLI help
  executed and Flywheel's run/replay/signer exports loaded on RuOS. Lockfile integrity
  records bind registry artifacts; npm did not expose gitHead, so equivalence to the
  separately inspected source revision is not claimed.
- Private source access works through the connector; shell Git does not inherit its credentials.
- The existing Chrome profile reached GitHub's sign-in page. Repository and Gist
  creation via that browser cannot proceed until the owner authenticates there.
- The GitHub connector exposes existing-repository writes but no repository-creation or Gist operations.

## Executor setup

Run `bash scripts/bootstrap-ruos.sh` on the Linux x86_64 RuOS desktop. It downloads
only the pinned Node archive, verifies its checksum and executes version probes.
It does not sign in, start background agents, provision machines, publish or deploy.
Set the printed PATH explicitly for **each** governed command; exports do not survive
between separate desktop_exec calls. Inspect completionVerified and exitCode.

Prefer the GitHub connector for authorized private source. Never copy browser cookies,
access tokens or connector credentials into a shell. Authenticate publishing through
the owner's supported GitHub browser/CLI flow and verify the account before writing.

## Required workflow

Read current core-memory, then complete frontier and enterprise OSS research even
when execution/publishing has a blocker. Search independent primary sources; record
actual publication/material-change dates, licenses, inspected code and exact revisions.
Prepare a ranked composition matrix. Select a substantive new standalone project with
two complementary Ruvnet integrations and an enterprise OSS integration.
Freeze specification, DDD, ADRs, implementation contract and evaluation criteria before
implementation. Execute integrations, Rust quality gates and bounded real evaluators.
Publish repository and research/build Gists through authorized paths, then read them back.
Maintenance work never substitutes for this deliverable. Each unfinished stage remains explicit.

## Receipt consistency gate

Run `node scripts/factory-gate.mjs RECEIPT.json EVIDENCE_ROOT`. Exit 1 means incomplete
or inconsistent evidence. `RECEIPT_CONSISTENT` is deliberately not `BUILT`: the gate
cannot independently establish whether a supplied command log or publishing receipt is true.
The operator still must inspect actual executor/connector results and public readbacks.
The gate has no network, credential or promotion authority.

Receipt schema: `schemaVersion: dream.factory.v1`, local `date: YYYY-MM-DD`;
`coreMemory: {commit, files: {path: blobSha}, appliedDecisions}`;
`research: {frontier, enterprise}` arrays of `{organization, date, primarySource, url}`
(enterprise additionally needs `{commit, license, codeInspected}`);
`project: {kind: new-project, thesis, repository, commit, readBackCommit, ingredients}`;
ingredients carry `{component, owner, commit, license, enterprise, integrationExecuted}`;
`commands` maps fmt/clippy/test/security/integration/benchmark to
`{executed, exitCode, commit}`; `evaluators` maps metaharness/darwin/flywheel to
`{executed, exitCode, upstreamCommit, mock}`; `gists` maps research/build to
`{owner, url, contentSha256, readBackSha256}`; `evidence` lists `{path, sha256}`.
Evidence paths must resolve inside EVIDENCE_ROOT, including symlink targets.

Test fixtures are synthetic and intentionally cannot prove live project execution.
Run `node --test scripts/factory-gate.test.mjs` to check the known failure modes.

## Inspected upstreams and recovery research

- Google RRSI: `be50316e1db05914068a973f322770ef08ed7ba1`, Apache-2.0;
  September 21, 2026 paper, September 23 latest inspected commit. Inspected schedule,
  critic, selection and domain adapter interfaces; ran eight upstream core checks.
  Sources: https://github.com/google-research/rrsi and https://arxiv.org/abs/2609.24972.
  Proposed integration: Rust coordinator invokes the pinned Python library through a
  bounded process protocol; live proposer/critic roles require authorized Vertex credentials.
  This is an integration proposal, not a completed integration or replication of paper metrics.
- MetaHarness: `9ce8b8dd89045c3b9a1f809ae58f3589029db4a4`, MIT;
  actual packages are `packages/darwin-mode` (@metaharness/darwin 0.10.3) and
  `packages/flywheel` (@metaharness/flywheel 0.1.12). Both were compiled locally.
  Flywheel's 143 tests passed with a package-scoped, non-serving test configuration.
  Darwin's package tests: 665 passed, 14 skipped (74 files). The control-plane
  workspace: 616 unit tests and 91 governance/script tests passed, along with
  typecheck, build, lint, edge-contract validation and development-policy checks.
  The first governance run required fetching full Git history for its pinned
  historical codec oracle; no tests or oracle revisions were changed.
  GitHub's first repair CI passed build/test, CodeQL and software evidence; its audit
  discovered existing brace-expansion 5.0.9 and fast-uri 3.1.7 vulnerabilities.
  Compatible lockfile updates to 5.0.12 and 3.1.8 removed both; local npm audit reported
  zero vulnerabilities. The remote rerun must be checked on the updated commit.
  These package tests are executor/dependency checks, not candidate improvement evidence.
- Microsoft run-assert-eval, September 24, 2026:
  https://commandline.microsoft.com/run-assert-eval-responsible-ai-agent-risk-discovery-at-runtime/.
  Primary report describes risk discovery, runtime-policy construction and frozen before/after
  evaluation. Source/license/revision integration review is not complete; not yet an accepted dependency.
- Google Retrieve-for-Train, September 15, 2026:
  https://research.google/blog/bypassing-inference-bottlenecks-accelerating-complex-ai-search-with-retrieve-for-train/.
  An inspected primary research lead, not a completed build ingredient.

This partial recovery list is not the complete required research slate. The immediate
Frontier Scout and Enterprise Radar reruns were requested separately; asynchronous run
requests do not establish completed reports, published Gists or successful factory builds.

## Rollback

Repository changes stay on a review branch until authorized merge. Restore previous
task prompts from the pre-repair task snapshot if needed; schedules were preserved.
The runtime is user-scoped: selecting the prior PATH leaves the installed Node unused.
Do not weaken checks or credential boundaries to remove a blocker.
