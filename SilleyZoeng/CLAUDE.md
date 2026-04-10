# Skill & Agent Evaluation Platform

Harbor + promptfoo evaluation platform for Claude Code skills and multi-agent harness.

## Tech Stack
- **promptfoo** - evaluation engine (YAML configs, custom providers/assertions)
- **Harbor / Docker Compose** - infrastructure (Ollama local judge + promptfoo UI)
- **TypeScript** - custom providers and assertions
- **@anthropic-ai/sdk** - Claude API calls

## Project Structure
- `src/providers/` - Custom promptfoo providers (claude-skill, claude-agent)
- `src/assertions/` - Custom assertions (trigger, safety, protocol, issue-packet)
- `evals/` - promptfoo YAML configs and JSON datasets
- `schemas/` - JSON schemas (issue-packet from how-to-work-together)
- `scripts/` - Setup and runner scripts

## Commands
- `make eval-all` - Run full evaluation suite
- `make eval-skills` - Skills only
- `make eval-agents` - Agents only
- `make dashboard` - Open promptfoo web UI
- `make harbor-up` - Start Docker services

## Key Paths
- Skills: `~/.claude/skills/*/SKILL.md`
- Harness: Set via `HARNESS_DIR` in `.env`
- Results: `output/` directory (gitignored)
