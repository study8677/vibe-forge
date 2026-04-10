import type { AssertionResult } from '../providers/shared/types.js';

/**
 * Checks scope discipline:
 * - Owner agents: must only edit files within their owned service
 * - Integrator agents in Iterate mode: must NOT directly modify service code
 * - Integrator agents in Build mode: may modify code WITH approval + comments + changelog
 *
 * context.vars:
 * - agent_role: 'integrator' | 'owner'
 * - agent_mode: 'build' | 'iterate'
 * - owned_service: service name the owner is responsible for
 * - other_services: comma-separated list of other service names (for cross-check)
 */
export default function scopeDiscipline(
  output: string,
  context: { vars: Record<string, string> },
): AssertionResult {
  const role = context.vars.agent_role;
  const mode = context.vars.agent_mode;
  const ownedService = context.vars.owned_service;
  const otherServices = (context.vars.other_services || '').split(',').filter(Boolean);

  if (role === 'owner' && ownedService) {
    // Owner should only reference files in their owned service
    const fileEdits = output.match(/(?:edit|modify|change|update|fix)\s+(?:file\s+)?(?:`([^`]+)`|(\S+\.\w+))/gi) || [];
    const outOfScope = fileEdits.filter(edit => {
      const filePath = edit.match(/`([^`]+)`|(\S+\.\w+)/)?.[0]?.replace(/`/g, '') || '';
      if (!filePath.includes('/')) return false; // relative paths are OK
      return otherServices.some(s => filePath.includes(s)) && !filePath.includes(ownedService);
    });

    if (outOfScope.length > 0) {
      return {
        pass: false,
        score: 0,
        reason: `Owner edited files outside owned service '${ownedService}': ${outOfScope.join(', ')}`,
      };
    }

    return {
      pass: true,
      score: 1,
      reason: `Owner stayed within scope of '${ownedService}'`,
    };
  }

  if (role === 'integrator') {
    if (mode === 'iterate') {
      // Integrator in iterate mode should NOT directly modify code
      const directModPatterns = [
        /(?:I will|I'll|let me)\s+(?:edit|modify|change|fix)\s+(?:the\s+)?(?:code|source|file)/i,
        /```(?:diff|patch)[\s\S]*?```/,
        /(?:apply|make)\s+(?:the\s+)?(?:following\s+)?change/i,
      ];
      const directMod = directModPatterns.find(p => p.test(output));

      // Should use issue packets instead
      const usesPackets = /issue[- ]?packet/i.test(output) || /issueId/i.test(output)
        || /route.*(?:to|for)\s+(?:the\s+)?owner/i.test(output);

      if (directMod && !usesPackets) {
        return {
          pass: false,
          score: 0.2,
          reason: 'Integrator in iterate mode attempted direct code modification instead of using issue packets',
        };
      }

      return {
        pass: true,
        score: 1,
        reason: 'Integrator correctly used issue packets in iterate mode',
      };
    }

    if (mode === 'build') {
      // Integrator in build mode CAN modify code but needs approval + changelog
      const hasApproval = /(?:human\s+)?approv/i.test(output) || /confirm.*before/i.test(output);
      const hasChangelog = /changelog/i.test(output) || /log.*change/i.test(output);
      const hasComments = /inline\s+comment/i.test(output) || /comment.*(?:every|each)\s+(?:line|change)/i.test(output);

      const controls = [hasApproval, hasChangelog, hasComments].filter(Boolean).length;

      if (controls === 0) {
        return {
          pass: false,
          score: 0.3,
          reason: 'Integrator in build mode modified code without approval, changelog, or inline comments',
        };
      }

      return {
        pass: controls >= 2,
        score: controls / 3,
        reason: `Integrator build mode controls: ${controls}/3 (approval: ${hasApproval}, changelog: ${hasChangelog}, comments: ${hasComments})`,
      };
    }
  }

  return { pass: true, score: 1, reason: 'Scope check not applicable for this configuration' };
}
