#!/usr/bin/env bash
# scorecard_x_collect.sh
# Collects X/Twitter metrics for the Content Marketing Scorecard.
# Reads:  ~/clawd/data/twitter/tweets.jsonl  (Alex's posts, appended daily)
# Reads:  /tmp/scorecard/metrics_x_prev.json  (previous week's metrics, if exists)
# Outputs JSON to stdout — full metrics with comparison to previous week.
#
# Usage: ./scorecard_x_collect.sh
set -euo pipefail

WORKDIR="${WORKDIR:-/home/alex/clawd}"
INDIR="$WORKDIR/data/twitter"
OUTDIR="${OUTDIR:-/tmp/scorecard}"
PREV_FILE="$OUTDIR/metrics_x_prev.json"

mkdir -p "$OUTDIR"

# ── helpers ──────────────────────────────────────────────────────────────────

parse_epoch() {
    python3 - "$1" <<'PY'
import sys, email.utils
s = sys.argv[1]
try:
    dt = email.utils.parsedate_to_datetime(s)
    print(int(dt.timestamp()))
except Exception:
    print(0)
PY
}

days_ago_epoch() {
    python3 - "$1" <<PY
import time, sys
print(int(time.time()) - int(sys.argv[1]) * 86400)
PY
}

# ── compute metrics ────────────────────────────────────────────────────────────

THIS_WEEK_EPOCH=$(days_ago_epoch 7)
WEEK_AGO_EPOCH=$(days_ago_epoch 14)

python3 - "$INDIR/tweets.jsonl" "$THIS_WEEK_EPOCH" "$WEEK_AGO_EPOCH" "$PREV_FILE" <<'PY'
import json, sys, email.utils, os
from datetime import datetime, timezone

path_in       = sys.argv[1]
this_week_cut = int(sys.argv[2])
week_ago_cut  = int(sys.argv[3])
prev_file    = sys.argv[4]

def parse_epoch(s):
    try:
        return int(email.utils.parsedate_to_datetime(s).timestamp())
    except:
        return 0

defeng = lambda t: t.get('likeCount',0) + t.get('retweetCount',0) + t.get('replyCount',0)

# ── load previous metrics ──────────────────────────────────────────────────
prev = {}
if os.path.exists(prev_file):
    try:
        prev = json.load(open(prev_file))
    except: pass

# ── load + split posts ─────────────────────────────────────────────────────
all_posts  = []
this_week  = []
week_ago   = []

with open(path_in) as f:
    for line in f:
        line = line.strip()
        if not line: continue
        try:
            t = json.loads(line)
        except: continue
        ts = parse_epoch(t.get('createdAt') or t.get('created_at', ''))
        if ts == 0: continue
        all_posts.append(t)
        if ts >= this_week_cut:
            this_week.append(t)
        elif ts >= week_ago_cut:
            week_ago.append(t)

# ── aggregate ────────────────────────────────────────────────────────────────
def aggregate(posts):
    if not posts:
        return {
            'count': 0, 'likes': 0, 'retweets': 0, 'replies': 0,
            'engagement': 0, 'avg_engagement': 0,
            'max_engagement': 0, 'top_post': None
        }
    likes      = sum(p.get('likeCount',0)    for p in posts)
    retweets   = sum(p.get('retweetCount',0) for p in posts)
    replies    = sum(p.get('replyCount',0)   for p in posts)
    eng_list   = [defeng(p) for p in posts]
    eng_total  = sum(eng_list)
    top_post   = max(posts, key=defeng)

    return {
        'count':          len(posts),
        'likes':          likes,
        'retweets':       retweets,
        'replies':        replies,
        'engagement':     eng_total,
        'avg_engagement': round(eng_total / len(posts), 1),
        'max_engagement': max(eng_list),
        'top_post': {
            'text':       top_post.get('text','')[:120],
            'likes':      top_post.get('likeCount',0),
            'retweets':   top_post.get('retweetCount',0),
            'replies':    top_post.get('replyCount',0),
            'engagement': defeng(top_post),
            'url':        f"https://x.com/{top_post.get('author',{}).get('username','?')}/status/{top_post.get('id','?')}"
        } if top_post else None
    }

tw  = aggregate(this_week)
wago= aggregate(week_ago)

# ── trend ─────────────────────────────────────────────────────────────────
def trend(cur, prev):
    if prev == 0: return 'N/A'
    delta = ((cur - prev) / prev) * 100
    if delta > 10:  return '↑'
    if delta < -10: return '↓'
    return '→'

# ── top 3 posts this week ────────────────────────────────────────────────
top3 = sorted(this_week, key=defeng, reverse=True)[:3]
top3_data = []
for p in top3:
    eng = defeng(p)
    top3_data.append({
        'text':      p.get('text','')[:120],
        'likes':     p.get('likeCount',0),
        'retweets':  p.get('retweetCount',0),
        'replies':   p.get('replyCount',0),
        'engagement': eng,
        'url':       f"https://x.com/{p.get('author',{}).get('username','?')}/status/{p.get('id','?')}"
    })

# ── build result ─────────────────────────────────────────────────────────
result = {
    'generated_at':     datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z'),
    'this_week':        tw,
    'week_ago':         wago,
    'trends': {
        'count':         trend(tw['count'],         wago['count']),
        'likes':         trend(tw['likes'],          wago['likes']),
        'retweets':      trend(tw['retweets'],       wago['retweets']),
        'replies':       trend(tw['replies'],        wago['replies']),
        'engagement':    trend(tw['engagement'],    wago['engagement']),
    },
    'top3':             top3_data,
    'prev_week_ago':    wago['count'],
    # For bookmarkCount: bird does not expose it in bulk/tweet output
    'bookmarkCount_note': 'not available in bird output (API gap)'
}

# Save as prev for next run
os.makedirs(os.path.dirname(prev_file), exist_ok=True)
with open(prev_file, 'w') as f:
    json.dump({'this_week': tw, 'generated_at': result['generated_at']}, f, indent=2)

print(json.dumps(result, ensure_ascii=False, indent=2))
PY
