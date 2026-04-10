import Anthropic from '@anthropic-ai/sdk';
import { config } from 'dotenv';
import { loadSkill, loadAllSkills } from './shared/skill-loader.js';
import {
  buildSkillTriggerContext,
  buildSkillProtocolContext,
  buildSkillSafetyContext,
} from './shared/context-builder.js';
import type { ProviderResponse } from './shared/types.js';

config();

const client = new Anthropic();

interface ProviderOptions {
  config?: {
    mode?: 'trigger' | 'protocol' | 'safety';
    model?: string;
  };
}

interface CallContext {
  vars: Record<string, string>;
}

/**
 * promptfoo custom provider for skill evaluation.
 *
 * Modes:
 * - trigger: Tests whether the model identifies the correct skill to invoke
 * - protocol: Tests whether the model follows the skill's step-by-step protocol
 * - safety: Tests whether the model refuses unsafe operations
 *
 * Config vars:
 * - target_skill: which skill to evaluate (required for protocol/safety modes)
 * - mode: override the provider-level mode
 */
export default class ClaudeSkillProvider {
  private mode: string;
  private model: string;

  constructor(options: ProviderOptions = {}) {
    this.mode = options.config?.mode || 'trigger';
    this.model = options.config?.model || process.env.EVAL_MODEL || 'claude-sonnet-4-20250514';
  }

  id(): string {
    return `claude-skill:${this.mode}`;
  }

  async callApi(prompt: string, context: CallContext): Promise<ProviderResponse> {
    const mode = context.vars.eval_mode || this.mode;
    const allSkills = loadAllSkills();

    let systemPrompt: string;

    switch (mode) {
      case 'protocol': {
        const targetSkill = context.vars.target_skill;
        if (!targetSkill) throw new Error('target_skill is required for protocol mode');
        const skill = loadSkill(targetSkill);
        systemPrompt = buildSkillProtocolContext(skill, allSkills);
        break;
      }
      case 'safety': {
        const targetSkill = context.vars.target_skill;
        if (!targetSkill) throw new Error('target_skill is required for safety mode');
        const skill = loadSkill(targetSkill);
        systemPrompt = buildSkillSafetyContext(skill, allSkills);
        break;
      }
      case 'trigger':
      default: {
        systemPrompt = buildSkillTriggerContext(allSkills);
        break;
      }
    }

    const response = await client.messages.create({
      model: this.model,
      max_tokens: 4096,
      system: systemPrompt,
      messages: [{ role: 'user', content: prompt }],
      temperature: 0,
    });

    const output = response.content
      .filter((c): c is Anthropic.TextBlock => c.type === 'text')
      .map(c => c.text)
      .join('\n');

    return {
      output,
      tokenUsage: {
        total: response.usage.input_tokens + response.usage.output_tokens,
        prompt: response.usage.input_tokens,
        completion: response.usage.output_tokens,
      },
    };
  }
}
