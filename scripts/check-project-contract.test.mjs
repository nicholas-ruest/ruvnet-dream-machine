import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { execFileSync } from 'node:child_process';
import { auditProject } from './check-project-contract.mjs';

function fixture(callback) {
  const root = mkdtempSync(join(tmpdir(), 'factory-contract-'));
  const put = (path, body) => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), body);
  };
  execFileSync('git', ['init', '-q', root]);
  try { callback(root, put); } finally { rmSync(root, { recursive: true, force: true }); }
}
test('rejects the observed two-source-file single-crate regression', () => fixture((root, put) => {
  put('Cargo.toml', '[package]\nname="primitive"\nversion="0.1.0"\n');
  put('src/lib.rs', 'pub fn run() {}');
  put('src/main.rs', 'fn main() {}');
  execFileSync('git', ['-C', root, 'add', '.']);
  const result = auditProject(root);
  assert.equal(result.status, 'NONCOMPLIANT_BUILD');
  assert.equal(result.counts.adrs, 0);
  assert.equal(result.counts.ddds, 0);
  assert.ok(result.failures.some(f => f.includes('20 implemented crates')));
}));
test('fails closed outside a Git checkout', () => {
  const root = mkdtempSync(join(tmpdir(), 'factory-no-git-'));
  try { assert.throws(() => auditProject(root)); } finally { rmSync(root, { recursive: true, force: true }); }
});
test('structural pass stays explicitly separate from completion; duplicates fail', () => fixture((root, put) => {
  for (const path of ['docs/specification.md','docs/architecture.md','docs/research.md',
    'docs/implementation-contract.md','docs/adrs/README.md','docs/ddd/README.md',
    'docs/integration-matrix.md','docs/core-memory-guidance-map.md','docs/validation.md','README.md']) {
    put(path, 'Fixture content '.repeat(20));
  }
  for (let i = 0; i < 25; i++) put('docs/adrs/ADR-' + i + '.md',
    '# Context\n' + ('Distinct fixture ' + i + ' ').repeat(40) +
    '\n## Alternatives\n## Decision\n## Consequences\n## Validation\n');
  for (let i = 0; i < 12; i++) put('docs/ddd/DDD-' + i + '.md',
    '# Context\n' + ('Domain fixture ' + i + ' ').repeat(40) + '\n## Language\n## Invariants\n');
  put('Cargo.toml', '[workspace]\nmembers=["crates/*"]');
  put('Cargo.lock', '# fixture');
  for (let i = 0; i < 20; i++) {
    put('crates/c' + i + '/Cargo.toml', '[package]\nname="c' + i + '"\nversion="0.1.0"');
    put('crates/c' + i + '/src/lib.rs', '// Fixture only\n'.repeat(12));
  }
  for (let i = 0; i < 2; i++) put('docs/assets/a' + i + '.svg', '<svg><animate attributeName="opacity"/></svg>');
  put('crates/c0/tests/integration.rs', '// fixture');
  execFileSync('git', ['-C', root, 'add', '.']);
  assert.equal(auditProject(root).status, 'STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW');
  put('docs/adrs/ADR-1.md',
    '# Context\n' + 'Distinct fixture 0 '.repeat(40) +
    '\n## Alternatives\n## Decision\n## Consequences\n## Validation\n');
  assert.ok(auditProject(root).failures.some(f => f.startsWith('Duplicate')));
}));
