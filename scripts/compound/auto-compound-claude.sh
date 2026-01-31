#!/usr/bin/env bash
# Fork of Ryan Carson's auto-compound pipeline, adapted for Claude Code.
# Pipeline: report → PRD → tasks → implementation loop → PR

set -euo pipefail

PROJECT_DIR="${PROJECT_DIR:-$PWD}"

cd "$PROJECT_DIR"

# Fetch latest (including tonight's AGENTS.md updates)

git fetch origin main

git reset --hard origin/main

# Find the latest prioritized report
LATEST_REPORT="$(ls -t reports/*.md 2>/dev/null | head -1 || true)"
if [[ -z "${LATEST_REPORT}" ]]; then
  echo "No reports found in reports/*.md" >&2
  exit 2
fi

# Analyze and pick #1 priority (outputs JSON)
ANALYSIS_JSON="$(./scripts/compound/analyze-report.sh "$LATEST_REPORT")"

PRIORITY_ITEM="$(echo "$ANALYSIS_JSON" | jq -r '.priority_item')"
BRANCH_NAME="$(echo "$ANALYSIS_JSON" | jq -r '.branch_name')"

if [[ -z "$PRIORITY_ITEM" || "$PRIORITY_ITEM" == "null" ]]; then
  echo "analyze-report.sh did not return a priority_item" >&2
  exit 3
fi
if [[ -z "$BRANCH_NAME" || "$BRANCH_NAME" == "null" ]]; then
  echo "analyze-report.sh did not return a branch_name" >&2
  exit 3
fi

# Create feature branch

git checkout -b "$BRANCH_NAME"

PRD_PROMPT="scripts/compound/prompts/create-prd.md"
TASKS_PROMPT="scripts/compound/prompts/prd-to-tasks.md"
LOOP_PROMPT="scripts/compound/prompts/implementation-loop.md"

# Create PRD
claude -p "$(cat "$PRD_PROMPT")

Project dir: $PROJECT_DIR
Feature: $PRIORITY_ITEM
Write PRD to: tasks/prd-$(basename "$BRANCH_NAME").md" \
  --dangerously-skip-permissions

# Convert PRD to tasks JSON
claude -p "$(cat "$TASKS_PROMPT")

Project dir: $PROJECT_DIR
PRD path: tasks/prd-$(basename "$BRANCH_NAME").md
Write tasks to: scripts/compound/prd.json" \
  --dangerously-skip-permissions

# Run the execution loop
./scripts/compound/loop.sh 25 "$LOOP_PROMPT"

# Create PR

git push -u origin "$BRANCH_NAME"

gh pr create --draft --title "Compound: $PRIORITY_ITEM" --base main
