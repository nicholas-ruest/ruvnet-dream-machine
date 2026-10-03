import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, writeFileSync, symlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { validateFactoryReceipt, evidenceReader } from './factory-gate.mjs';

const sha = 'a'.repeat(40);
const bytes = Buffer.from('execution log');
const hash = createHash('sha256').update(bytes).digest('hex');
function receipt() {
  return {
    schemaVersion: 'dream.factory.v1', date: '2026-10-02',
    coreMemory: { commit: sha, files: Object.fromEntries(['AGENTS.md', 'CLAUDE.md',
      'docs/architecture-overview.md', 'docs/adr/0002.md', 'docs/ddd/governance.md'].map((p) => [p, sha])),
      appliedDecisions: ['only the adapter imports storage packages'] },
    research: Object.fromEntries(['frontier', 'enterprise'].map((stream) => [stream,
      ['Google', 'Microsoft', 'NVIDIA'].map((organization) => ({ organization, date: '2026-09-21',
        primarySource: true, url: 'https://example.org/paper', commit: sha, license: 'Apache-2.0', codeInspected: true }))])),
    project: { kind: 'new-project', thesis: 'A substantive integrated system with a frozen evaluation.',
      repository: 'https://github.com/nicholas-ruest/example', commit: sha, readBackCommit: sha,
      ingredients: ['darwin', 'flywheel', 'external'].map((component, i) => ({
        component, owner: i < 2 ? 'ruvnet' : 'google-research', enterprise: i === 2,
        commit: sha, license: 'MIT', integrationExecuted: true })) },
    commands: Object.fromEntries(['fmt', 'clippy', 'test', 'security', 'integration', 'benchmark']
      .map((name) => [name, { executed: true, exitCode: 0, commit: sha }])),
    evaluators: Object.fromEntries(['metaharness', 'darwin', 'flywheel']
      .map((name) => [name, { executed: true, exitCode: 0, upstreamCommit: sha, mock: false }])),
    gists: Object.fromEntries(['research', 'build'].map((name) => [name, {
      owner: 'nicholas-ruest', url: 'https://gist.github.com/nicholas-ruest/abc123',
      contentSha256: hash, readBackSha256: hash }])),
    evidence: [{ path: 'logs/test.txt', sha256: hash }],
  };
}
const check = (r) => validateFactoryReceipt(r, { readEvidence: () => bytes });
test('complete synthetic receipt is consistent, not proof of execution', () => {
  assert.equal(check(receipt()).status, 'RECEIPT_CONSISTENT');
  assert.match(check(receipt()).caveat, /not independent proof/);
});
test('maintenance cannot substitute for a new project', () => {
  const r = receipt(); r.project.kind = 'maintenance';
  assert(check(r).failures.includes('new-project-required-maintenance-does-not-qualify'));
});
test('missing core-memory read fails', () => {
  const r = receipt(); delete r.coreMemory.files['AGENTS.md'];
  assert(check(r).failures.includes('core-memory-read:AGENTS.md'));
});
test('stale enterprise and future frontier dates fail', () => {
  const r = receipt(); r.research.enterprise[0].date = '2026-08-01';
  r.research.frontier[0].date = '2026-10-03';
  assert(check(r).failures.includes('enterprise-research-date'));
  assert(check(r).failures.includes('frontier-research-date'));
});
test('tests must pass on exact published commit', () => {
  const r = receipt(); r.commands.test.exitCode = 1; r.commands.clippy.commit = 'b'.repeat(40);
  assert(check(r).failures.includes('executed:test'));
  assert(check(r).failures.includes('executed:clippy'));
});
test('mock evaluator and decorative integration fail', () => {
  const r = receipt(); r.evaluators.darwin.mock = true; r.project.ingredients[0].integrationExecuted = false;
  assert(check(r).failures.includes('evaluator:darwin'));
  assert(check(r).failures.includes('two-real-ruvnet-integrations'));
});
test('Gist owner, URL host and readback are checked', () => {
  const r = receipt(); r.gists.research.url = 'https://gist.github.com.evil.test/nicholas-ruest/abc123';
  r.gists.build.readBackSha256 = 'b'.repeat(64);
  assert(check(r).failures.includes('gist:research')); assert(check(r).failures.includes('gist:build'));
});
test('tampered execution evidence fails', () => {
  const r = receipt(); r.evidence[0].sha256 = 'b'.repeat(64);
  assert(check(r).failures.includes('evidence-digest:logs/test.txt'));
});
test('evidence reader rejects traversal and escaping symlinks', () => {
  const parent = mkdtempSync(join(tmpdir(), 'factory-gate-'));
  const root = mkdtempSync(join(parent, 'evidence-'));
  writeFileSync(join(parent, 'outside'), 'secret');
  symlinkSync(join(parent, 'outside'), join(root, 'link'));
  assert.throws(() => evidenceReader(root)('../outside'));
  assert.throws(() => evidenceReader(root)('link'));
});
test('empty receipt cannot pass', () => {
  assert.equal(check({}).status, 'PARTIAL_FAILURE');
  assert.equal(check(null).status, 'PARTIAL_FAILURE');
});
