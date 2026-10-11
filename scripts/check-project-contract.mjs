#!/usr/bin/env node
// Structural acceptance only. This cannot prove integration execution or design quality.
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve, relative } from 'node:path';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';

export function auditProject(root) {
  const files = execFileSync('git', ['-C', root, 'ls-files', '-z'], { encoding: 'utf8' })
    .split('\0').filter(Boolean);
  const failures = [];
  const text = (path) => {
    const target = resolve(root, path);
    if (relative(resolve(root), target).startsWith('..')) throw new Error('Unsafe path');
    return readFileSync(target, 'utf8');
  };
  const required = [
    'docs/specification.md', 'docs/architecture.md', 'docs/research.md',
    'docs/implementation-contract.md', 'docs/adrs/README.md', 'docs/ddd/README.md',
    'docs/integration-matrix.md', 'docs/core-memory-guidance-map.md',
    'docs/validation.md', 'README.md',
  ];
  for (const path of required) {
    if (!files.includes(path) || text(path).trim().length < 100) failures.push('Missing or empty: ' + path);
  }
  function documents(prefix, minimum, sections) {
    const paths = files.filter(p => p.startsWith(prefix) && p.endsWith('.md') && !/\/(README|index)\.md$/i.test(p));
    const hashes = new Set();
    for (const path of paths) {
      const body = text(path);
      const normalized = body.replace(/^#.*$/gm, '').replace(/\s+/g, ' ').trim();
      const digest = createHash('sha256').update(normalized).digest('hex');
      if (normalized.length < 500) failures.push('Insufficient document content: ' + path);
      if (hashes.has(digest)) failures.push('Duplicate document body: ' + path);
      hashes.add(digest);
      for (const section of sections) {
        if (!new RegExp('^#{1,6}\\s+.*' + section, 'im').test(body)) failures.push('Missing section ' + section + ': ' + path);
      }
    }
    if (paths.length < minimum) failures.push(prefix + ' requires at least ' + minimum + ' documents; found ' + paths.length);
    return paths.length;
  }
  const adrs = documents('docs/adrs/', 25, ['context', 'alternatives', 'decision', 'consequences', 'validation']);
  const ddds = documents('docs/ddd/', 12, ['context', 'language', 'invariants']);
  const manifests = files.filter(p => p.endsWith('/Cargo.toml'));
  const crates = manifests.filter(p => {
    const dir = p.slice(0, -'Cargo.toml'.length);
    return /^\[package\]/m.test(text(p)) && files.some(f => f.startsWith(dir + 'src/') && f.endsWith('.rs') && text(f).trim().length > 100);
  });
  if (crates.length < 20) failures.push('Requires at least 20 implemented crates; found ' + crates.length);
  if (!files.includes('Cargo.toml') || !/^\[workspace\]/m.test(text('Cargo.toml'))) failures.push('Missing Cargo workspace');
  const svgs = files.filter(p => p.startsWith('docs/assets/') && p.endsWith('.svg') && /<animate\b|@keyframes/.test(text(p)));
  if (svgs.length < 2) failures.push('Requires 2 animated SVG assets; found ' + svgs.length);
  const integrationTests = files.filter(p => /(^|\/)tests\/.*\.rs$/.test(p));
  if (!integrationTests.length) failures.push('Missing integration test sources');
  if (!files.includes('Cargo.lock')) failures.push('Missing Cargo.lock');
  return {
    status: failures.length ? 'NONCOMPLIANT_BUILD' : 'STRUCTURAL_PASS_REQUIRES_EVIDENCE_REVIEW',
    counts: { adrs, ddds, crates: crates.length, animatedSvgs: svgs.length },
    failures,
    limitations: 'Structural checks do not prove meaningful architecture, linked workspace membership, executed upstream integrations, benchmark validity, CI success, or publication authorization.',
  };
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  try {
    const result = auditProject(resolve(process.argv[2] || '.'));
    console.log(JSON.stringify(result, null, 2));
    process.exitCode = result.failures.length ? 1 : 0;
  } catch (error) {
    console.log(JSON.stringify({ status: 'PRECHECK_BLOCKED', error: error.message }));
    process.exitCode = 2;
  }
}
