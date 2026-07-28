#!/usr/bin/env bash
set -euo pipefail

# Daily X/Twitter ingest using bird -> local JSONL files -> (optional) gno reindex.
# Designed to be safe to run repeatedly.

WORKDIR="/home/alex/clawd"
OUTDIR="$WORKDIR/data/twitter"
STATE="$OUTDIR/state.json"
TZ="Europe/London"

# Ensure Homebrew tools (e.g., newer cmake) are available for gno/node-llama-cpp builds.
export PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

mkdir -p "$OUTDIR"

# State
if [[ -f "$STATE" ]]; then
  last_run_epoch=$(python3 -c 'import json;print(json.load(open("'$STATE'")).get("last_run_epoch",0))')
else
  last_run_epoch=0
fi
now_epoch=$(date +%s)

write_state() {
  python3 - <<PY
import json, time
p="$STATE"
obj={"last_run_epoch": $now_epoch}
open(p,'w').write(json.dumps(obj, indent=2, sort_keys=True))
print(p)
PY
}

# Helper: normalize tweet objects into JSONL lines, filtering by createdAt when available.
filter_to_jsonl() {
  local kind="$1"; shift
  local tmp_json="$1"; shift
  local out_jsonl="$1"; shift

  python3 - "$kind" "$tmp_json" "$out_jsonl" "$last_run_epoch" <<'PY'
import json,sys,email.utils
kind=sys.argv[1]
tmp_json=sys.argv[2]
out_jsonl=sys.argv[3]
last_run=int(sys.argv[4])

with open(tmp_json,'r') as f:
  data=json.load(f)

tweets=data.get('tweets', data if isinstance(data,list) else [])

def parse_created_at(s):
  if not s:
    return None
  # Twitter-like: "Wed Apr 30 15:09:41 +0000 2025"
  try:
    dt=email.utils.parsedate_to_datetime(s)
    return int(dt.timestamp())
  except Exception:
    return None

new=[]
for t in tweets:
  if not isinstance(t,dict):
    continue
  ca=t.get('createdAt') or t.get('created_at')
  ts=parse_created_at(ca)
  if ts is None or ts >= last_run:
    t['_ingest_kind']=kind
    new.append(t)

# Append JSONL
with open(out_jsonl,'a') as out:
  for t in new:
    out.write(json.dumps(t, ensure_ascii=False))
    out.write('\n')

print(f"{kind}: appended {len(new)}")
PY
}

run_bird() {
  local cmd="$1"; shift
  if ! command -v bird >/dev/null 2>&1; then
    echo "bird not found" >&2
    return 1
  fi

  # Prefer explicit cookie tokens (avoids Linux keyring/secret-tool issues when extracting Chrome cookies).
  # NOTE: creds.sh should be chmod 600 and never committed.
  if [[ -f "$HOME/.config/bird/creds.sh" ]]; then
    # shellcheck disable=SC1091
    source "$HOME/.config/bird/creds.sh"
  fi

  eval "$cmd"
}

stamp=$(TZ="$TZ" date +%F)

# 1) Bookmarks
TMP1=$(mktemp)
if run_bird "bird bookmarks --auth-token \"${AUTH_TOKEN:-}\" --ct0 \"${CT0:-}\" --all --max-pages 10 --json > '$TMP1'"; then
  filter_to_jsonl "bookmarks" "$TMP1" "$OUTDIR/bookmarks.jsonl"
else
  echo "bookmarks: bird command failed" >&2
fi
rm -f "$TMP1"

# 2) Likes (best-effort; some installs may not have this subcommand)
TMP2=$(mktemp)
if run_bird "bird likes --auth-token \"${AUTH_TOKEN:-}\" --ct0 \"${CT0:-}\" --all --max-pages 10 --json > '$TMP2'"; then
  filter_to_jsonl "likes" "$TMP2" "$OUTDIR/likes.jsonl"
else
  echo "likes: bird command failed (ok to ignore)" >&2
fi
rm -f "$TMP2"

# 3) Own tweets
TMP3=$(mktemp)
if run_bird "bird user-tweets alex_mikhalev --auth-token \"${AUTH_TOKEN:-}\" --ct0 \"${CT0:-}\" --max-pages 5 --json > '$TMP3'"; then
  filter_to_jsonl "tweets" "$TMP3" "$OUTDIR/tweets.jsonl"
else
  echo "tweets: bird command failed" >&2
fi
rm -f "$TMP3"

# 4) Optional: reindex with gno (fast enough for ~/clawd scale)
if command -v gno >/dev/null 2>&1; then
  (cd "$WORKDIR" && gno index --yes) || echo "gno index failed (ok to ignore)" >&2
fi

write_state

# Simple daily heartbeat file
printf "%s\t%s\n" "$stamp" "$now_epoch" >> "$OUTDIR/runs.tsv"
