import Anthropic from '@anthropic-ai/sdk';
import { config } from 'dotenv';
import { loadAgentContext } from './shared/agent-loader.js';
import { buildAgentContext } from './shared/context-builder.js';
import type { ProviderResponse } from './shared/types.js';

config();

const client = new Anthropic();

interface ProviderOptions {
  config?: {
    role?: 'integrator' | 'owner';
    mode?: 'build' | 'iterate';
    model?: string;
  };
}

interface CallContext {
  vars: Record<string, string>;
}

/**
 * promptfoo custom provider for agent role evaluation.
 *
 * Injects the agent protocol, role template, issue packet schema,
 * and service profiles into the system prompt. Tests whether the
 * model follows the multi-agent collaboration protocol correctly.
 *
 * Config vars:
 * - agent_role: 'integrator' | 'owner' (overrides provider-level config)
 * - agent_mode: 'build' | 'iterate' (overrides provider-level config)
 */
export default class ClaudeAgentProvider {
  private defaultRole: 'integrator' | 'owner';
  private defaultMode: 'build' | 'iterate';
  private model: string;

  constructor(options: ProviderOptions = {}) {
    this.defaultRole = options.config?.role || 'integrator';
    this.defaultMode = options.config?.mode || 'iterate';
    this.model = options.config?.model || process.env.EVAL_MODEL || 'claude-sonnet-4-20250514';
  }

  id(): string {
    return `claude-agent:${this.defaultRole}:${this.defaultMode}`;
  }

  async callApi(prompt: string, context: CallContext): Promise<ProviderResponse> {
    const role = (context.vars.agent_role as 'integrator' | 'owner') || this.defaultRole;
    const mode = (context.vars.agent_mode as 'build' | 'iterate') || this.defaultMode;

    const agentCtx = loadAgentContext(role, mode);
    const systemPrompt = buildAgentContext(agentCtx);

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
