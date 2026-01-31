#!/usr/bin/env bash
# Simple iterative execution loop.
# Usage: loop.sh <max_iterations> <prompt_file>
#
# The prompt file should instruct the agent to:
# - read scripts/compound/prd.json
# - pick the next unfinished task
# - implement it
# - run tests/linters
# - mark task done (update prd.json)
#
# This loop invokes Claude Code by default for execution because it has strong tooling.
# You can swap to OpenCode by editing the RUNNER section.

set -euo pipefail

MAX_ITERS="${1:-25}"
PROMPT_FILE="${2:-}"

if [[ -z "$PROMPT_FILE" || ! -f "$PROMPT_FILE" ]]; then
  echo "Usage: $0 <max_iterations> <prompt_file>" >&2
  exit 2
fi

if [[ ! -f scripts/compound/prd.json ]]; then
  echo "Missing scripts/compound/prd.json (tasks file)." >&2
  exit 3
fi

for ((i=1; i<=MAX_ITERS; i++)); do
  echo "[compound loop] iteration $i/$MAX_ITERS"

  # Stop early if everything is done
  if jq -e 'all(.tasks[]; .status == "done")' scripts/compound/prd.json >/dev/null 2>&1; then
    echo "[compound loop] all tasks done"
    exit 0
  fi

  # RUNNER: Claude Code
  claude -p "$(cat "$PROMPT_FILE")" --dangerously-skip-permissions

done

echo "[compound loop] hit iteration limit ($MAX_ITERS); leaving branch for manual follow-up" >&2
exit 1
