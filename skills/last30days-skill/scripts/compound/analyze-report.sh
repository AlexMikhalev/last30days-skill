#!/usr/bin/env bash
# Minimal report analyzer.
# Input: path to a markdown report
# Output: JSON with { priority_item, branch_name }

set -euo pipefail

REPORT_PATH="${1:-}"
if [[ -z "$REPORT_PATH" || ! -f "$REPORT_PATH" ]]; then
  echo "Usage: $0 reports/some-report.md" >&2
  exit 2
fi

# Heuristic: take the first markdown list item under a heading containing "Priority" or "Top".
# If none found, fall back to first list item in the file.

PRIORITY_ITEM="$(
  awk '
    BEGIN{inprio=0}
    /^#{1,6} /{
      inprio=0
      if (tolower($0) ~ /priority|top/) inprio=1
    }
    inprio && $0 ~ /^[-*] / {
      sub(/^[-*] /, "", $0)
      print $0
      exit
    }
  ' "$REPORT_PATH"
)"

if [[ -z "$PRIORITY_ITEM" ]]; then
  PRIORITY_ITEM="$(grep -E '^[[:space:]]*[-*] ' -m 1 "$REPORT_PATH" | sed -E 's/^[[:space:]]*[-*][[:space:]]+//')"
fi

if [[ -z "$PRIORITY_ITEM" ]]; then
  PRIORITY_ITEM="Review and improve compound automation"
fi

# Branch name: compound/<yyyy-mm-dd>-<slug>
DATE_PART="$(date +%F)"
SLUG="$(echo "$PRIORITY_ITEM" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+|-+$//g' | cut -c1-50)"
BRANCH_NAME="compound/${DATE_PART}-${SLUG}"

jq -n --arg priority_item "$PRIORITY_ITEM" --arg branch_name "$BRANCH_NAME" '{priority_item:$priority_item, branch_name:$branch_name}'
