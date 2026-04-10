import type { AssertionResult } from '../providers/shared/types.js';

/**
 * Checks that when an agent reports a failure, all required evidence is present.
 *
 * Required evidence fields:
 * - traceId: a unique identifier for the request
 * - reproCommand: an exact, executable command to reproduce the issue
 * - expected: what should have happened
 * - actual: what actually happened
 * - summary: a brief description
 *
 * Optional but recommended:
 * - requestSnapshot: the request that was sent
 * - responseSnapshot: the response that came back
 * - errorLog: relevant error logs
 */
export default function evidenceCompleteness(
  output: string,
  _context: { vars: Record<string, string> },
): AssertionResult {
  const checks = {
    traceId: /trace[_-]?id/i.test(output) || /trace-[a-z0-9]+/i.test(output),
    reproCommand: hasReproCommand(output),
    expected: /expected/i.test(output) || /should\s+(have\s+)?(return|respond|produce)/i.test(output),
    actual: /actual/i.test(output) || /instead|but\s+(got|received|returned)/i.test(output),
    summary: /summary/i.test(output) || output.length > 50, // Any substantive response counts
  };

  const optional = {
    requestSnapshot: /request(?:Snapshot)?/i.test(output) || /request\s*[:]\s*{/i.test(output),
    responseSnapshot: /response(?:Snapshot)?/i.test(output) || /response\s*[:]\s*{/i.test(output),
    errorLog: /error[_\s]?log/i.test(output) || /log.*error/i.test(output)
      || /journalctl/i.test(output) || /stderr/i.test(output),
  };

  const requiredPassed = Object.entries(checks).filter(([, v]) => v);
  const optionalPassed = Object.entries(optional).filter(([, v]) => v);
  const requiredFailed = Object.entries(checks).filter(([, v]) => !v).map(([k]) => k);

  const requiredScore = requiredPassed.length / Object.keys(checks).length;
  const optionalBonus = optionalPassed.length * 0.05; // Small bonus for optional fields
  const score = Math.min(1, requiredScore + optionalBonus);

  return {
    pass: requiredFailed.length === 0,
    score,
    reason: requiredFailed.length === 0
      ? `All required evidence present (${requiredPassed.length}/5 required, ${optionalPassed.length}/3 optional)`
      : `Missing required evidence: ${requiredFailed.join(', ')}`,
  };
}

function hasReproCommand(output: string): boolean {
  // Check for a command in code blocks
  const codeBlockCmd = /```[\s\S]*?(curl|pnpm|npm|bash|sh|python|pytest|cargo|go\s+test|docker|make)[\s\S]*?```/.test(output);
  // Check for inline commands
  const inlineCmd = /`(curl|pnpm|npm|bash|sh|python|pytest|cargo|go\s+test|docker|make)\s[^`]+`/.test(output);
  // Check for reproCommand field
  const reproField = /reproCommand/i.test(output);
  return codeBlockCmd || inlineCmd || reproField;
}
