import fs from 'node:fs';
import path from 'node:path';
import type { AgentContext } from './types.js';

const DEFAULT_HARNESS_DIR = '/Users/fanjingwen/Projects/teamo/teamo-router/how-to-work-together';

function getHarnessDir(): string {
  return process.env.HARNESS_DIR || DEFAULT_HARNESS_DIR;
}

function safeRead(filePath: string): string {
  if (!fs.existsSync(filePath)) {
    return `[File not found: ${filePath}]`;
  }
  return fs.readFileSync(filePath, 'utf-8');
}

export function loadAgentProtocol(): string {
  const dir = getHarnessDir();
  return safeRead(path.join(dir, 'docs', 'agent-protocol.md'));
}

export function loadIntegratorTemplate(): string {
  const dir = getHarnessDir();
  // Try multiple possible locations
  const candidates = [
    path.join(dir, 'templates', 'AGENTS-integrator.md'),
    path.join(dir, 'templates', 'integration-repo', 'AGENTS.md'),
  ];
  for (const p of candidates) {
    if (fs.existsSync(p)) return fs.readFileSync(p, 'utf-8');
  }
  return safeRead(candidates[0]);
}

export function loadOwnerTemplate(): string {
  const dir = getHarnessDir();
  const candidates = [
    path.join(dir, 'templates', 'service-repo', 'AGENTS.md'),
    path.join(dir, 'templates', 'AGENTS-owner.md'),
  ];
  for (const p of candidates) {
    if (fs.existsSync(p)) return fs.readFileSync(p, 'utf-8');
  }
  return safeRead(candidates[0]);
}

export function loadIssuePacketSchema(): string {
  const dir = getHarnessDir();
  return safeRead(path.join(dir, 'schemas', 'issue-packet.schema.json'));
}

export function loadServiceProfiles(): string[] {
  const dir = path.join(getHarnessDir(), 'coordination', 'service-profiles');
  if (!fs.existsSync(dir)) return [];

  return fs.readdirSync(dir)
    .filter(f => f.endsWith('.md'))
    .map(f => fs.readFileSync(path.join(dir, f), 'utf-8'));
}

export function loadAgentContext(
  role: 'integrator' | 'owner',
  mode: 'build' | 'iterate' = 'iterate',
): AgentContext {
  const protocol = loadAgentProtocol();
  const template = role === 'integrator'
    ? loadIntegratorTemplate()
    : loadOwnerTemplate();
  const issuePacketSchema = loadIssuePacketSchema();
  const serviceProfiles = loadServiceProfiles();

  return {
    role,
    protocol,
    template,
    mode,
    serviceProfiles,
    issuePacketSchema,
  };
}

// Self-test when run directly
if (process.argv[1] && process.argv[1].includes('agent-loader')) {
  const ctx = loadAgentContext('integrator', 'iterate');
  console.log('Agent context loaded:');
  console.log(`  Role: ${ctx.role}`);
  console.log(`  Mode: ${ctx.mode}`);
  console.log(`  Protocol length: ${ctx.protocol.length} chars`);
  console.log(`  Template length: ${ctx.template.length} chars`);
  console.log(`  Service profiles: ${ctx.serviceProfiles?.length || 0}`);
  console.log(`  Issue packet schema: ${ctx.issuePacketSchema ? 'loaded' : 'missing'}`);
}
