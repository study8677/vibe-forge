#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Skill & Agent Evaluation Platform Setup ==="
echo ""

# Step 1: Check prerequisites
echo "[1/5] Checking prerequisites..."

if ! command -v node &> /dev/null; then
    echo "ERROR: Node.js is required. Install it from https://nodejs.org/"
    exit 1
fi
echo "  Node.js: $(node --version)"

if ! command -v docker &> /dev/null; then
    echo "WARNING: Docker not found. Harbor/Ollama features will be unavailable."
else
    echo "  Docker: $(docker --version)"
fi

# Step 2: Install npm dependencies
echo ""
echo "[2/5] Installing npm dependencies..."
cd "$PROJECT_DIR"
npm install

# Step 3: Set up .env
echo ""
echo "[3/5] Setting up environment..."
if [ ! -f "$PROJECT_DIR/.env" ]; then
    cp "$PROJECT_DIR/.env.example" "$PROJECT_DIR/.env"
    echo "  Created .env from .env.example"
    echo "  IMPORTANT: Edit .env and set your ANTHROPIC_API_KEY"
else
    echo "  .env already exists"
fi

# Step 4: Verify skill access
echo ""
echo "[4/5] Verifying skill access..."
SKILLS_DIR="${SKILLS_DIR:-$HOME/.claude/skills}"
if [ -d "$SKILLS_DIR" ]; then
    SKILL_COUNT=$(find "$SKILLS_DIR" -name "SKILL.md" -maxdepth 2 | wc -l | tr -d ' ')
    echo "  Found $SKILL_COUNT skills in $SKILLS_DIR"
    for skill_dir in "$SKILLS_DIR"/*/; do
        if [ -f "$skill_dir/SKILL.md" ]; then
            echo "    - $(basename "$skill_dir")"
        fi
    done
else
    echo "  WARNING: Skills directory not found at $SKILLS_DIR"
fi

# Step 5: Optional - Start Docker services
echo ""
echo "[5/5] Docker services (optional)..."
if command -v docker &> /dev/null; then
    read -p "  Start Ollama + promptfoo-ui? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cd "$PROJECT_DIR"
        docker compose up -d
        echo "  Services started:"
        echo "    - Ollama: http://localhost:11434"
        echo "    - promptfoo UI: http://localhost:3000"

        # Pull judge model
        echo "  Pulling Ollama judge model..."
        docker exec eval-ollama ollama pull llama3.1:8b 2>/dev/null || echo "  (model pull may take a while, continuing...)"
    fi
fi

echo ""
echo "=== Setup Complete ==="
echo ""
echo "Quick start:"
echo "  1. Edit .env and set ANTHROPIC_API_KEY"
echo "  2. Run smoke test:  make eval-all"
echo "  3. Run skill evals: make eval-skills"
echo "  4. Run agent evals: make eval-agents"
echo "  5. View dashboard:  make dashboard"
echo ""
