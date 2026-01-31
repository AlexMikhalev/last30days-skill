#!/usr/bin/env bash
# Fork of Ryan Carson's daily compound review, adapted for OpenCode.
# Runs BEFORE auto-compound to update AGENTS.md with learnings.

set -euo pipefail

PROJECT_DIR="${PROJECT_DIR:-$PWD}"
OPENCODE_MODEL="${OPENCODE_MODEL:-}"

cd "$PROJECT_DIR"

# Ensure we're on main and up to date

git checkout main

git pull --ff-only origin main

# Notes:
# - OpenCode doesn't (by default) have a universal notion of “threads from last 24h” like Amp.
# - This prompt focuses on local artifacts (git history, changed files, logs, memory/*) and
#   asks the agent to retroactively compound learnings into AGENTS.md.

PROMPT_FILE="scripts/compound/prompts/daily-compound-review.md"

ARGS=(run)
if [[ -n "$OPENCODE_MODEL" ]]; then
  ARGS+=(--model "$OPENCODE_MODEL")
fi
ARGS+=(--prompt "$PROMPT_FILE")

opencode "${ARGS[@]}" \
  -- \
  "Project dir: $PROJECT_DIR" \
  "Task: Run the daily compound review for the last 24 hours of work." \
  "Deliverables: update AGENTS.md and any relevant docs; commit and push to main."
