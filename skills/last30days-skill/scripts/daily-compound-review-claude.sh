#!/usr/bin/env bash
# Fork of Ryan Carson's daily compound review, adapted for Claude Code.
# Runs BEFORE auto-compound to update AGENTS.md with learnings.

set -euo pipefail

PROJECT_DIR="${PROJECT_DIR:-$PWD}"

cd "$PROJECT_DIR"

# Ensure we're on main and up to date

git checkout main

git pull --ff-only origin main

PROMPT_FILE="scripts/compound/prompts/daily-compound-review.md"

# Claude Code invocation: pass prompt file content as the main prompt.
# We keep it as a file so the prompt is versioned and can evolve.

claude -p "$(cat "$PROMPT_FILE")

Project dir: $PROJECT_DIR
Task: Run the daily compound review for the last 24 hours of work.
Deliverables: update AGENTS.md and any relevant docs; commit and push to main." \
  --dangerously-skip-permissions
