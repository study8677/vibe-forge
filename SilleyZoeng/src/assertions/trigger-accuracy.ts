import type { AssertionResult } from '../providers/shared/types.js';

const ALL_SKILLS = [
  'deploy', 'ops-report', 'ssh-prod',
  'log-patrol', 'integrator-patrol', 'self-improving',
];

/**
 * Checks whether the model correctly identified the right skill to invoke.
 *
 * For positive cases (should_trigger=true): checks that the output contains
 * "SKILL: <expected_skill>" or mentions the expected skill name prominently.
 *
 * For negative cases (should_trigger=false): checks that the output contains
 * "SKILL: none" and doesn't invoke any of the 6 skills.
 */
export default function triggerAccuracy(
  output: string,
  context: { vars: Record<string, string> },
): AssertionResult {
  const expectedSkill = context.vars.expected_skill;
  const shouldTrigger = context.vars.should_trigger === 'true';
  const outputLower = output.toLowerCase();

  // Check for explicit SKILL: tag in output
  const skillTagMatch = output.match(/SKILL:\s*(\S+)/i);
  const invokedSkill = skillTagMatch ? skillTagMatch[1].toLowerCase() : null;

  if (shouldTrigger) {
    // Positive case: should trigger the expected skill
    const expectedLower = expectedSkill.toLowerCase();

    // Primary check: explicit SKILL: tag matches
    if (invokedSkill === expectedLower) {
      return { pass: true, score: 1, reason: `Correctly identified skill: ${expectedSkill}` };
    }

    // Secondary check: skill name mentioned prominently in the response
    const mentionPatterns = [
      new RegExp(`\\b${expectedLower}\\b`, 'i'),
      new RegExp(`skill.*${expectedLower}`, 'i'),
      new RegExp(`invoke.*${expectedLower}`, 'i'),
      new RegExp(`trigger.*${expectedLower}`, 'i'),
      new RegExp(`use.*${expectedLower}`, 'i'),
    ];
    const mentioned = mentionPatterns.some(p => p.test(output));

    if (mentioned) {
      // Check it's not SKILL: none with a passing mention
      if (invokedSkill === 'none') {
        return {
          pass: false,
          score: 0.2,
          reason: `Said SKILL: none but mentioned ${expectedSkill}. Expected to trigger ${expectedSkill}.`,
        };
      }
      return { pass: true, score: 0.8, reason: `Mentioned skill ${expectedSkill} (no explicit SKILL: tag)` };
    }

    return {
      pass: false,
      score: 0,
      reason: `Failed to trigger skill '${expectedSkill}'. Got: ${invokedSkill || 'no skill identified'}`,
    };
  } else {
    // Negative case: should NOT trigger any skill
    if (invokedSkill === 'none' || invokedSkill === null) {
      // Check no skill names appear in an "invoke" context
      const falsePositive = ALL_SKILLS.find(s =>
        new RegExp(`(invoke|trigger|use|SKILL:)\\s*${s}`, 'i').test(output),
      );
      if (falsePositive) {
        return {
          pass: false,
          score: 0.3,
          reason: `Said SKILL: none but appears to invoke '${falsePositive}'`,
        };
      }
      return { pass: true, score: 1, reason: 'Correctly avoided triggering any skill' };
    }

    return {
      pass: false,
      score: 0,
      reason: `Incorrectly triggered skill '${invokedSkill}' when none should have been triggered`,
    };
  }
}
