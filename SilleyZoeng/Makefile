.PHONY: setup eval-all eval-skills eval-agents eval-trigger eval-safety eval-protocol dashboard harbor-up harbor-down clean

# Setup
setup:
	bash scripts/setup.sh

# Run all evaluations
eval-all:
	npx promptfoo eval -c promptfooconfig.yaml

# Skill evaluations
eval-skills:
	npx promptfoo eval -c evals/skills/trigger-accuracy.yaml
	npx promptfoo eval -c evals/skills/protocol-compliance.yaml
	npx promptfoo eval -c evals/skills/safety.yaml

# Agent evaluations
eval-agents:
	npx promptfoo eval -c evals/agents/integrator-compliance.yaml
	npx promptfoo eval -c evals/agents/owner-compliance.yaml
	npx promptfoo eval -c evals/agents/mode-adherence.yaml

# Individual evaluation suites
eval-trigger:
	npx promptfoo eval -c evals/skills/trigger-accuracy.yaml

eval-safety:
	npx promptfoo eval -c evals/skills/safety.yaml

eval-protocol:
	npx promptfoo eval -c evals/skills/protocol-compliance.yaml

# Run a single skill evaluation: make eval-skill SKILL=deploy
eval-skill:
	npx promptfoo eval -c evals/skills/trigger-accuracy.yaml --filter-description "$(SKILL)"

# Dashboard
dashboard:
	npx promptfoo view

# Harbor / Docker
harbor-up:
	docker compose up -d

harbor-down:
	docker compose down

# Clean output
clean:
	rm -rf output/ .promptfoo/
