#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

# Load .env if it exists
if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

usage() {
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  all          Run all evaluation suites"
    echo "  skills       Run all skill evaluations"
    echo "  agents       Run all agent evaluations"
    echo "  trigger      Run trigger accuracy tests"
    echo "  safety       Run safety tests"
    echo "  protocol     Run protocol compliance tests"
    echo "  integrator   Run integrator agent tests"
    echo "  owner        Run owner agent tests"
    echo "  mode         Run mode adherence tests"
    echo "  smoke        Run quick smoke test (master config)"
    echo "  view         Open promptfoo web UI"
    echo ""
    echo "Options:"
    echo "  --output DIR   Output directory (default: output/)"
    echo "  --verbose      Show detailed output"
    echo ""
}

COMMAND="${1:-smoke}"
OUTPUT_DIR="${OUTPUT_DIR:-output}"
VERBOSE=""

shift || true
while [[ $# -gt 0 ]]; do
    case "$1" in
        --output) OUTPUT_DIR="$2"; shift 2 ;;
        --verbose) VERBOSE="--verbose"; shift ;;
        *) echo "Unknown option: $1"; usage; exit 1 ;;
    esac
done

mkdir -p "$OUTPUT_DIR"

run_eval() {
    local config="$1"
    local name="$2"
    echo "Running: $name"
    echo "  Config: $config"
    npx promptfoo eval -c "$config" --output "$OUTPUT_DIR/${name}.json" $VERBOSE
    echo "  Done. Results: $OUTPUT_DIR/${name}.json"
    echo ""
}

case "$COMMAND" in
    all)
        run_eval "evals/skills/trigger-accuracy.yaml" "trigger-accuracy"
        run_eval "evals/skills/protocol-compliance.yaml" "protocol-compliance"
        run_eval "evals/skills/safety.yaml" "safety"
        run_eval "evals/agents/integrator-compliance.yaml" "integrator-compliance"
        run_eval "evals/agents/owner-compliance.yaml" "owner-compliance"
        run_eval "evals/agents/mode-adherence.yaml" "mode-adherence"
        echo "All evaluations complete. Run 'make dashboard' to view results."
        ;;
    skills)
        run_eval "evals/skills/trigger-accuracy.yaml" "trigger-accuracy"
        run_eval "evals/skills/protocol-compliance.yaml" "protocol-compliance"
        run_eval "evals/skills/safety.yaml" "safety"
        ;;
    agents)
        run_eval "evals/agents/integrator-compliance.yaml" "integrator-compliance"
        run_eval "evals/agents/owner-compliance.yaml" "owner-compliance"
        run_eval "evals/agents/mode-adherence.yaml" "mode-adherence"
        ;;
    trigger)
        run_eval "evals/skills/trigger-accuracy.yaml" "trigger-accuracy"
        ;;
    safety)
        run_eval "evals/skills/safety.yaml" "safety"
        ;;
    protocol)
        run_eval "evals/skills/protocol-compliance.yaml" "protocol-compliance"
        ;;
    integrator)
        run_eval "evals/agents/integrator-compliance.yaml" "integrator-compliance"
        ;;
    owner)
        run_eval "evals/agents/owner-compliance.yaml" "owner-compliance"
        ;;
    mode)
        run_eval "evals/agents/mode-adherence.yaml" "mode-adherence"
        ;;
    smoke)
        run_eval "promptfooconfig.yaml" "smoke"
        ;;
    view)
        npx promptfoo view
        ;;
    *)
        echo "Unknown command: $COMMAND"
        usage
        exit 1
        ;;
esac
