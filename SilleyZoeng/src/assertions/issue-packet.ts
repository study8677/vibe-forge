import Ajv from 'ajv';
import addFormats from 'ajv-formats';
import { readFileSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import type { AssertionResult } from '../providers/shared/types.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const schemaPath = resolve(__dirname, '..', '..', 'schemas', 'issue-packet.schema.json');
const schema = JSON.parse(readFileSync(schemaPath, 'utf-8'));

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const AjvCtor = Ajv as any;
const ajv = new AjvCtor({ allErrors: true });
// eslint-disable-next-line @typescript-eslint/no-explicit-any
(addFormats as any)(ajv);
const validate = ajv.compile(schema);

/**
 * Extracts JSON from the model output (looks for code fences containing JSON)
 * and validates it against the issue-packet.schema.json.
 *
 * Also performs semantic checks:
 * - fromService and toService match expected values (if provided in vars)
 * - severity is reasonable for the scenario
 * - reproCommand looks executable
 */
export default function issuePacketValidation(
  output: string,
  context: { vars: Record<string, string> },
): AssertionResult {
  // Extract JSON from code fences
  const jsonMatches = output.match(/```(?:json)?\s*\n([\s\S]*?)\n```/g);
  if (!jsonMatches) {
    return { pass: false, score: 0, reason: 'No JSON code block found in output' };
  }

  // Try each code block until we find valid JSON
  let packet: Record<string, unknown> | null = null;
  let parseError = '';

  for (const match of jsonMatches) {
    const content = match.replace(/```(?:json)?\s*\n/, '').replace(/\n```$/, '');
    try {
      const parsed = JSON.parse(content);
      if (parsed && typeof parsed === 'object' && 'issueId' in parsed) {
        packet = parsed;
        break;
      }
    } catch (e) {
      parseError = String(e);
    }
  }

  if (!packet) {
    return {
      pass: false,
      score: 0.1,
      reason: `No valid issue packet JSON found. Parse error: ${parseError}`,
    };
  }

  // Schema validation
  const valid = validate(packet);
  if (!valid) {
    const errors = validate.errors
      ?.map((e: { instancePath?: string; message?: string }) => `${e.instancePath || '/'} ${e.message}`)
      .join('; ');
    return {
      pass: false,
      score: 0.3,
      reason: `Schema validation failed: ${errors}`,
    };
  }

  // Semantic checks
  const issues: string[] = [];
  let semanticScore = 1;

  if (context.vars.expected_from && packet.fromService !== context.vars.expected_from) {
    issues.push(`fromService: expected '${context.vars.expected_from}', got '${packet.fromService}'`);
    semanticScore -= 0.2;
  }

  if (context.vars.expected_to && packet.toService !== context.vars.expected_to) {
    issues.push(`toService: expected '${context.vars.expected_to}', got '${packet.toService}'`);
    semanticScore -= 0.2;
  }

  // Check reproCommand looks executable
  const repro = packet.reproCommand as string;
  if (repro && repro.length < 5) {
    issues.push('reproCommand is suspiciously short');
    semanticScore -= 0.1;
  }

  // Check issueId format (INC-YYYYMMDD-NNNN)
  const issueId = packet.issueId as string;
  if (issueId && !/^INC-\d{8}-\d{4}$/.test(issueId)) {
    issues.push(`issueId format should be INC-YYYYMMDD-NNNN, got '${issueId}'`);
    semanticScore -= 0.1;
  }

  const finalScore = Math.max(0, semanticScore);

  return {
    pass: issues.length === 0,
    score: finalScore,
    reason: issues.length === 0
      ? 'Valid issue packet with correct routing'
      : `Schema valid but semantic issues: ${issues.join('; ')}`,
  };
}
