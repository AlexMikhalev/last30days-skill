#!/usr/bin/env bash
set -euo pipefail

# Daily X/Twitter ingest using bird -> local JSONL files -> (optional) gno reindex.
# Designed to be safe to run repeatedly without duplicates.
#
# Key improvements:
# - Saves raw bird output to data/twitter/raw/YYYY-MM-DD/ before processing
# - Deduplicates by tweet ID against existing JSONL to avoid re-adding on re-runs
# - Preserves all fields from the original tweet object (media, quotedTweet, etc.)
# - Uses ISO8601 in state file for better human readability

WORKDIR="/home/alex/clawd"
OUTDIR="$WORKDIR/data/twitter"
RAWDIR="$OUTDIR/raw"
STATE="$OUTDIR/state.json"
TZ="Europe/London"

export PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

mkdir -p "$OUTDIR" "$RAWDIR"

# ── State ──────────────────────────────────────────────────────────────────
now_epoch=$(date +%s)
now_iso=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

if [[ -f "$STATE" ]]; then
  last_run_epoch=$(python3 -c "import json; print(json.load(open('$STATE')).get('last_run_epoch', 0))")
else
  last_run_epoch=0
fi

write_state() {
  python3 - <<PY
import json
obj = {"last_run_epoch": $now_epoch, "last_run_iso": "$now_iso"}
open("$STATE", "w").write(json.dumps(obj, indent=2, sort_keys=True))
PY
}

# ── Deduplication helper ────────────────────────────────────────────────────
# Loads existing tweet IDs from a JSONL into a set (ram-efficient for typical sizes).
load_ids() {
  local jsonl="$1"
  python3 - <<PY
import json
ids = set()
try:
  with open("$jsonl") as f:
    for line in f:
      try:
        d = json.loads(line)
        if "id" in d:
          ids.add(d["id"])
      except Exception:
        pass
  print(json.dumps(list(ids)))
except FileNotFoundError:
  print("[]")
PY
}

# ── Main ingest function ────────────────────────────────────────────────────
ingest() {
  local kind="$1"; shift
  local bird_cmd="$1"; shift

  local stamp
  stamp=$(TZ="$TZ" date +%Y-%m-%d)
  local raw_path="$RAWDIR/${stamp}_${kind}.json"

  echo "=== $kind: fetching ==="
  local tmp_json
  tmp_json=$(mktemp)

  if ! run_bird "$bird_cmd" > "$tmp_json" 2>&1; then
    echo "$kind: bird command failed" >&2
    rm -f "$tmp_json"
    return 1
  fi

  # Archive the raw output first (before any filtering)
  cp "$tmp_json" "$raw_path"
  echo "$kind: raw saved -> $raw_path"

  # Deduplicate against existing IDs
  local existing_ids_json
  existing_ids_json=$(load_ids "$OUTDIR/${kind}.jsonl")
  local out_jsonl="$OUTDIR/${kind}.jsonl"

  python3 - "$kind" "$tmp_json" "$out_jsonl" "$existing_ids_json" "$last_run_epoch" <<'PY'
import json, sys, email.utils

kind        = sys.argv[1]
tmp_json    = sys.argv[2]
out_jsonl   = sys.argv[3]
raw_ids_str = sys.argv[4]
last_run    = int(sys.argv[5])

existing_ids = set(json.loads(raw_ids_str))

with open(tmp_json) as f:
  raw = json.load(f)

tweets = raw if isinstance(raw, list) else raw.get('tweets', [])

def parse_created_at(s):
  if not s:
    return None
  try:
    return int(email.utils.parsedate_to_datetime(s).timestamp())
  except Exception:
    return None

new_count = 0
skipped_dup = 0

with open(out_jsonl, 'a') as out:
  for t in tweets:
    if not isinstance(t, dict):
      continue
    tid = t.get('id')
    if not tid:
      continue
    # Skip already-seen IDs (dedup across runs)
    if tid in existing_ids:
      skipped_dup += 1
      continue
    ts = parse_created_at(t.get('createdAt') or t.get('created_at'))
    if ts is None or ts >= last_run:
      t['_ingest_kind'] = kind
      out.write(json.dumps(t, ensure_ascii=False) + '\n')
      existing_ids.add(tid)   # prevent within-run duplicates
      new_count += 1

print(f"{kind}: new={new_count} skipped_dup={skipped_dup}")
PY

  rm -f "$tmp_json"
}

run_bird() {
  local cmd="$1"
  if ! command -v bird >/dev/null 2>&1; then
    echo "bird not found" >&2
    return 1
  fi
  if [[ -f "$HOME/.config/bird/creds.sh" ]]; then
    # shellcheck disable=SC1091
    source "$HOME/.config/bird/creds.sh"
  fi
  eval "$cmd"
}

# ── Run ────────────────────────────────────────────────────────────────────
ingest bookmarks "bird bookmarks --auth-token \"${AUTH_TOKEN:-}\" --ct0 \"${CT0:-}\" --all --max-pages 10 --json"
ingest likes     "bird likes     --auth-token \"${AUTH_TOKEN:-}\" --ct0 \"${CT0:-}\" --all --max-pages 10 --json"
ingest timeline  "bird home      --auth-token \"${AUTH_TOKEN:-}\" --ct0 \"${CT0:-}\" --count 100 --following --json"

# ── Optional reindex ────────────────────────────────────────────────────────
if command -v gno >/dev/null 2>&1; then
  (cd "$WORKDIR" && gno index --yes) || echo "gno index failed (ok to ignore)" >&2
fi

write_state
printf "%s\t%s\n" "$(TZ="$TZ" date +%F)" "$now_epoch" >> "$OUTDIR/runs.tsv"
echo "=== done ==="
