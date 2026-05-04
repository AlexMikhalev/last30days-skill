#!/usr/bin/env bash
# scorecard_reddit_collect.sh
# Collects Reddit metrics for alex_mikhalev's posts.
# Reads:  Reddit .json API (no auth needed for public posts)
# Outputs JSON to stdout.
#
# Usage: ./scorecard_reddit_collect.sh
set -euo pipefail

OUTDIR="${OUTDIR:-/tmp/scorecard}"
PREV_FILE="$OUTDIR/metrics_reddit_prev.json"
USER="alex_mikhalev"

mkdir -p "$OUTDIR"

days_ago_epoch() {
    python3 - "$1" <<PY
import time, sys
print(int(time.time()) - int(sys.argv[1]) * 86400)
PY
}

THIS_WEEK_EPOCH=$(days_ago_epoch 7)
WEEK_AGO_EPOCH=$(days_ago_epoch 14)

python3 - "$USER" "$THIS_WEEK_EPOCH" "$WEEK_AGO_EPOCH" "$PREV_FILE" <<'PY'
import json, sys, os, email.utils
from datetime import datetime, timezone

user        = sys.argv[1]
this_week_cut = int(sys.argv[2])
week_ago_cut  = int(sys.argv[3])
prev_file    = sys.argv[4]

def parse_epoch_utc(utc_ts):
    try:
        return int(utc_ts)
    except:
        return 0

defeng = lambda p: p.get('score', 0) + p.get('num_comments', 0)

# ── fetch from Reddit ──────────────────────────────────────────────────────
import urllib.request, urllib.error

def fetch_user_posts():
    url = f"https://www.reddit.com/user/{user}/submitted/.json?limit=100&sort=new"
    req = urllib.request.Request(url, headers={
        'User-Agent': 'Kairo-Scorecard/1.0 (social media metrics)',
        'Accept': 'application/json'
    })
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"Reddit fetch error: {e}", file=sys.stderr)
        return None

data = fetch_user_posts()
if data is None:
    print(json.dumps({'error': 'failed to fetch Reddit data'}))
    exit(1)

# ── load previous ─────────────────────────────────────────────────────────
prev = {}
if os.path.exists(prev_file):
    try:
        prev = json.load(open(prev_file))
    except: pass

# ── parse posts ──────────────────────────────────────────────────────────
this_week = []
week_ago  = []

children = data.get('data', {}).get('children', [])
for child in children:
    post = child.get('data', {})
    ts = parse_epoch_utc(post.get('created_utc', 0))
    if ts >= this_week_cut:
        this_week.append(post)
    elif ts >= week_ago_cut:
        week_ago.append(post)

# ── aggregate ────────────────────────────────────────────────────────────
def aggregate(posts):
    if not posts:
        return {
            'count': 0, 'score': 0, 'comments': 0,
            'avg_score': 0, 'max_score': 0,
            'top_post': None
        }
    scores  = [p.get('score', 0) for p in posts]
    comments= [p.get('num_comments', 0) for p in posts]
    top     = max(posts, key=lambda p: p.get('score', 0))
    return {
        'count':       len(posts),
        'score':       sum(scores),
        'comments':    sum(comments),
        'avg_score':   round(sum(scores) / len(posts), 1),
        'max_score':  max(scores),
        'top_post': {
            'title':     top.get('title','')[:120],
            'score':     top.get('score', 0),
            'comments':  top.get('num_comments', 0),
            'subreddit': top.get('subreddit', ''),
            'url':       f"https://reddit.com{top.get('permalink','')}"
        } if top else None
    }

tw   = aggregate(this_week)
wago = aggregate(week_ago)

def trend(cur, prev_val):
    if prev_val == 0: return 'N/A'
    delta = ((cur - prev_val) / prev_val) * 100
    if delta > 10:  return '↑'
    if delta < -10: return '↓'
    return '→'

top3 = sorted(this_week, key=lambda p: p.get('score', 0), reverse=True)[:3]
top3_data = [{
    'title':     p.get('title','')[:120],
    'score':     p.get('score', 0),
    'comments':  p.get('num_comments', 0),
    'subreddit': p.get('subreddit', ''),
    'url':       f"https://reddit.com{p.get('permalink','')}"
} for p in top3]

result = {
    'generated_at':  datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z'),
    'this_week':     tw,
    'week_ago':      wago,
    'trends': {
        'count':    trend(tw['count'],    wago['count']),
        'score':    trend(tw['score'],    wago['score']),
        'comments': trend(tw['comments'], wago['comments']),
    },
    'top3': top3_data,
    'prev_week_ago_count': wago['count']
}

# Save prev
os.makedirs(os.path.dirname(prev_file), exist_ok=True)
with open(prev_file, 'w') as f:
    json.dump({'this_week': tw, 'generated_at': result['generated_at']}, f, indent=2)

print(json.dumps(result, ensure_ascii=False, indent=2))
PY
