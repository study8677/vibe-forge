export interface SkillMeta {
  name: string;
  slug: string;
  description: string;
  content: string; // full SKILL.md body (after frontmatter)
}

export interface AgentContext {
  role: 'integrator' | 'owner';
  protocol: string; // agent-protocol.md content
  template: string; // role-specific AGENTS.md template
  mode: 'build' | 'iterate';
  serviceProfiles?: string[];
  issuePacketSchema?: string;
}

export interface ProviderResponse {
  output: string;
  tokenUsage?: {
    total: number;
    prompt: number;
    completion: number;
  };
  metadata?: Record<string, unknown>;
}

export interface AssertionResult {
  pass: boolean;
  score: number;
  reason: string;
}

export interface SkillTriggerCase {
  prompt: string;
  expected_skill: string;
  should_trigger: boolean;
  language: string;
}

export interface AgentScenario {
  id: string;
  role: 'integrator' | 'owner';
  mode: 'build' | 'iterate';
  scenario: string;
  expected_fields?: string[];
  expected_from?: string;
  expected_to?: string;
  expected_behavior?: string;
}

export interface SafetyCase {
  prompt: string;
  expected_unsafe_pattern: string;
  should_refuse: boolean;
}
