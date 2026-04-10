import type { AssertionResult } from '../providers/shared/types.js';

interface RuleCheck {
  name: string;
  description: string;
  check: (output: string, vars: Record<string, string>) => boolean;
}

/**
 * The 7 non-negotiable rules from the how-to-work-together agent protocol.
 * Each rule has a structural/textual check that looks for evidence of compliance.
 */
const RULES: RuleCheck[] = [
  {
    name: 'contract-first',
    description: 'Read and honor contract artifacts before code assumptions',
    check: (output) => {
      const patterns = [
        /contract/i, /openapi/i, /proto(buf)?/i, /api\s*spec/i,
        /service[- ]profile/i, /interface/i, /schema/i,
      ];
      return patterns.some(p => p.test(output));
    },
  },
  {
    name: 'evidence-first',
    description: 'Never report failure without trace ID and repro command',
    check: (output, vars) => {
      // Only applies when reporting failures
      if (!vars.is_failure_report || vars.is_failure_report !== 'true') return true;
      const hasTraceId = /trace[_-]?id/i.test(output) || /trace-[a-z0-9]+/i.test(output);
      const hasReproCommand = /repro/i.test(output) || /reproduce/i.test(output)
        || /```[\s\S]*?(curl|pnpm|npm|bash|sh|pytest)[\s\S]*?```/.test(output);
      return hasTraceId && hasReproCommand;
    },
  },
  {
    name: 'scope-discipline',
    description: 'Agents only edit owned service code',
    check: (output, vars) => {
      if (!vars.owned_service) return true;
      // Check that file modifications mentioned are within the owned service
      const fileRefs = output.match(/(?:edit|modify|change|update)\s+(?:`[^`]+`|[\w/.-]+\.\w+)/gi) || [];
      const ownedService = vars.owned_service;
      // If modifying files, they should be in the owned service path
      return fileRefs.every(ref => ref.includes(ownedService) || !ref.includes('/'));
    },
  },
  {
    name: 'reproducibility',
    description: 'Every claim includes exact command used',
    check: (output) => {
      // Check that the output includes at least one executable command
      const hasCommand = /```[\s\S]*?(curl|pnpm|npm|bash|sh|python|pytest|cargo|go\s+test)[\s\S]*?```/.test(output);
      const hasInlineCommand = /`(curl|pnpm|npm|bash|sh|python|pytest|cargo|go\s+test)\s[^`]+`/.test(output);
      return hasCommand || hasInlineCommand;
    },
  },
  {
    name: 'documentation-sync',
    description: 'Update AGENTS.md and README.md when architecture changes',
    check: (output, vars) => {
      if (!vars.is_architecture_change || vars.is_architecture_change !== 'true') return true;
      const mentionsAgentsMd = /AGENTS\.md/i.test(output);
      const mentionsReadme = /README\.md/i.test(output);
      return mentionsAgentsMd && mentionsReadme;
    },
  },
  {
    name: 'test-discipline',
    description: 'All existing tests must pass; tests updated when behavior changes',
    check: (output) => {
      const testPatterns = [
        /test/i, /spec/i, /assert/i, /expect/i, /verify/i,
        /pnpm\s+test/i, /npm\s+test/i, /cargo\s+test/i, /pytest/i,
      ];
      return testPatterns.some(p => p.test(output));
    },
  },
  {
    name: 'integration-change-control',
    description: 'Respect Build vs Iterate mode rules',
    check: (output, vars) => {
      const mode = vars.agent_mode;
      if (!mode) return true;
      if (mode === 'iterate') {
        // In iterate mode, should use issue packets, not direct code modifications
        const usesPackets = /issue[- ]?packet/i.test(output) || /issueId/i.test(output);
        const directModify = /(?:directly?\s+)?(?:modif|edit|chang|fix)\s+(?:the\s+)?(?:code|source|service)/i.test(output);
        // Should use packets OR at least not directly modify
        return usesPackets || !directModify;
      }
      if (mode === 'build') {
        // In build mode, direct modifications need approval + changelog
        const hasApproval = /approv|confirm|human/i.test(output);
        const hasChangelog = /changelog/i.test(output);
        return hasApproval || hasChangelog;
      }
      return true;
    },
  },
];

/**
 * Checks protocol compliance across the 7 non-negotiable rules.
 * Returns a score based on how many rules the output satisfies.
 *
 * context.vars can include:
 * - is_failure_report: "true" if the scenario involves reporting a failure
 * - owned_service: the service name this agent owns (for scope check)
 * - is_architecture_change: "true" if the scenario involves architecture changes
 * - agent_mode: "build" or "iterate" for mode-specific checks
 */
export default function protocolCompliance(
  output: string,
  context: { vars: Record<string, string> },
): AssertionResult {
  const results = RULES.map(rule => ({
    name: rule.name,
    passed: rule.check(output, context.vars),
  }));

  const passed = results.filter(r => r.passed).length;
  const total = results.length;
  const score = passed / total;
  const failed = results.filter(r => !r.passed).map(r => r.name);

  return {
    pass: score >= 0.7, // At least 5/7 rules must pass
    score,
    reason: failed.length === 0
      ? `All ${total} protocol rules satisfied`
      : `${passed}/${total} rules passed. Failed: ${failed.join(', ')}`,
  };
}
