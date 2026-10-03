#!/usr/bin/env node
/** Validate a factory receipt. This checks evidence consistency, not its truth. */
import { createHash } from 'node:crypto';
import { readFileSync, realpathSync } from 'node:fs';
import { resolve, relative, isAbsolute } from 'node:path';
import { fileURLToPath } from 'node:url';

const SHA = /^[a-f0-9]{40}$/;
const HASH = /^[a-f0-9]{64}$/;
const REQUIRED_READS = ['AGENTS.md', 'CLAUDE.md', 'docs/architecture-overview.md'];
const REQUIRED_COMMANDS = ['fmt', 'clippy', 'test', 'security', 'integration', 'benchmark'];

function day(value) {
  return typeof value === 'string' && /^\d{4}-\d{2}-\d{2}$/.test(value)
    && Number.isFinite(Date.parse(value)) && new Date(value).toISOString().slice(0, 10) === value;
}
function url(value, host, pattern) {
  try {
    const u = new URL(value);
    return u.protocol === 'https:' && u.hostname === host && !u.username && !u.password
      && !u.search && !u.hash && pattern.test(u.pathname);
  } catch { return false; }
}

export function validateFactoryReceipt(receipt, { readEvidence = () => null } = {}) {
  const failures = [];
  const check = (ok, reason) => { if (!ok) failures.push(reason); };
  if (!receipt || typeof receipt !== 'object' || Array.isArray(receipt)) {
    return { status: 'PARTIAL_FAILURE', failures: ['receipt-object-required'] };
  }
  check(receipt.schemaVersion === 'dream.factory.v1', 'schema');
  check(day(receipt.date), 'date');
  const memory = receipt.coreMemory ?? {};
  check(SHA.test(memory.commit ?? ''), 'core-memory-commit');
  for (const path of REQUIRED_READS) check(SHA.test(memory.files?.[path] ?? ''), `core-memory-read:${path}`);
  check(Object.keys(memory.files ?? {}).some((p) => p.startsWith('docs/adr/') && SHA.test(memory.files[p])), 'core-memory-adr');
  check(Object.keys(memory.files ?? {}).some((p) => p.startsWith('docs/ddd/') && SHA.test(memory.files[p])), 'core-memory-ddd');
  check(Array.isArray(memory.appliedDecisions) && memory.appliedDecisions.length > 0, 'core-memory-applied-decisions');

  const research = receipt.research ?? {};
  for (const [stream, window] of [['frontier', 60], ['enterprise', 30]]) {
    const items = research[stream];
    check(Array.isArray(items) && items.length >= 3, `${stream}-research-coverage`);
    const organizations = new Set();
    for (const item of Array.isArray(items) ? items : []) {
      organizations.add(item.organization);
      const age = (Date.parse(receipt.date) - Date.parse(item.date)) / 86400000;
      check(day(item.date) && age >= 0 && age <= window, `${stream}-research-date`);
      check(typeof item.organization === 'string' && item.organization.length > 0, `${stream}-organization`);
      check(item.primarySource === true && typeof item.url === 'string' && item.url.startsWith('https://'), `${stream}-primary-source`);
      if (stream === 'enterprise') check(SHA.test(item.commit ?? '') && Boolean(item.license) && item.codeInspected === true, 'enterprise-code-provenance');
    }
    check(organizations.size >= 3, `${stream}-organization-diversity`);
  }
  const project = receipt.project ?? {};
  check(project.kind === 'new-project', 'new-project-required-maintenance-does-not-qualify');
  check(typeof project.thesis === 'string' && project.thesis.length > 20, 'project-thesis');
  check(url(project.repository, 'github.com', /^\/nicholas-ruest\/[a-zA-Z0-9_.-]+$/)
    && project.repository !== 'https://github.com/nicholas-ruest/ruvnet-dream-machine', 'standalone-repository');
  check(SHA.test(project.commit ?? ''), 'project-commit');
  check(project.readBackCommit === project.commit, 'repository-readback');
  const ingredients = project.ingredients ?? [];
  check(Array.isArray(ingredients), 'ingredients-array');
  const integrated = Array.isArray(ingredients) ? ingredients.filter((i) => SHA.test(i.commit ?? '') && i.integrationExecuted === true) : [];
  check(new Set(integrated.filter((i) => i.owner === 'ruvnet').map((i) => i.component)).size >= 2, 'two-real-ruvnet-integrations');
  check(integrated.some((i) => i.owner !== 'ruvnet' && i.enterprise === true && Boolean(i.license)), 'real-enterprise-integration');
  for (const command of REQUIRED_COMMANDS) {
    const entry = receipt.commands?.[command];
    check(entry?.exitCode === 0 && entry?.commit === project.commit && entry?.executed === true, `executed:${command}`);
  }
  for (const evaluator of ['metaharness', 'darwin', 'flywheel']) {
    const entry = receipt.evaluators?.[evaluator];
    check(entry?.executed === true && entry?.exitCode === 0 && SHA.test(entry?.upstreamCommit ?? '') && entry?.mock !== true, `evaluator:${evaluator}`);
  }
  for (const output of ['research', 'build']) {
    const publication = receipt.gists?.[output];
    check(publication?.owner === 'nicholas-ruest'
      && url(publication?.url, 'gist.github.com', /^\/nicholas-ruest\/[a-f0-9]+$/)
      && HASH.test(publication?.contentSha256 ?? '')
      && publication?.readBackSha256 === publication?.contentSha256, `gist:${output}`);
  }
  check(Array.isArray(receipt.evidence) && receipt.evidence.length > 0, 'evidence-required');
  for (const entry of Array.isArray(receipt.evidence) ? receipt.evidence : []) {
    try {
      const bytes = readEvidence(entry.path);
      check(bytes !== null && HASH.test(entry.sha256 ?? '')
        && createHash('sha256').update(bytes).digest('hex') === entry.sha256, `evidence-digest:${entry.path}`);
    } catch { check(false, `evidence-unreadable:${entry.path}`); }
  }
  return { status: failures.length ? 'PARTIAL_FAILURE' : 'RECEIPT_CONSISTENT', failures,
    caveat: 'Receipt consistency is not independent proof of live execution or publication.' };
}

export function evidenceReader(root) {
  const base = realpathSync(resolve(root));
  return (path) => {
    if (typeof path !== 'string' || isAbsolute(path)) throw new Error('relative evidence path required');
    const absolute = realpathSync(resolve(base, path));
    const suffix = relative(base, absolute);
    if (!suffix || suffix === '..' || suffix.startsWith('../') || isAbsolute(suffix)) throw new Error('evidence path escapes root');
    // Symlinks are resolved before the containment check so evidence cannot escape.
    return readFileSync(absolute);
  };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    if (process.argv.length !== 4) throw new Error('usage: node scripts/factory-gate.mjs RECEIPT.json EVIDENCE_ROOT');
    const result = validateFactoryReceipt(JSON.parse(readFileSync(process.argv[2], 'utf8')),
      { readEvidence: evidenceReader(process.argv[3]) });
    console.log(JSON.stringify(result, null, 2));
    process.exitCode = result.failures.length ? 1 : 0;
  } catch (error) {
    console.error(JSON.stringify({ status: 'PARTIAL_FAILURE', failures: [error.message] }));
    process.exitCode = 1;
  }
}
