"""Bird (X CLI) integration for X (Twitter) discovery.

This provides a fallback/alternative to xAI live search. It relies on the
local `bird` CLI being authenticated (typically via browser cookies).

We intentionally keep this lightweight: discover candidate posts via
`bird search --json`, then normalize into the schema expected by the skill.
"""

import json
import re
import subprocess
import sys
from typing import Any, Dict, List, Optional, Tuple

from . import dates


def _log_error(msg: str):
    sys.stderr.write(f"[X ERROR] {msg}\n")
    sys.stderr.flush()


DEPTH_CONFIG = {
    "quick": (8, 12),
    "default": (20, 30),
    "deep": (40, 60),
}


def _run(cmd: List[str], timeout: int = 90) -> Tuple[int, str, str]:
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    return p.returncode, p.stdout, p.stderr


def _as_int(v: Any) -> Optional[int]:
    try:
        if v is None:
            return None
        return int(v)
    except Exception:
        return None


def _parse_bird_created_at_to_ymd(s: Optional[str]) -> Optional[str]:
    """Convert bird's createdAt like 'Fri Feb 13 23:52:29 +0000 2026' to YYYY-MM-DD."""
    if not s:
        return None
    dt = dates.parse_date(s)
    if dt:
        return dt.date().isoformat()
    # Fallback for common Twitter format
    m = re.match(r"^[A-Za-z]{3} ([A-Za-z]{3}) (\d{1,2}) .* (\d{4})$", s.strip())
    if not m:
        return None
    return None


def search_x(
    topic: str,
    from_date: str,
    to_date: str,
    depth: str = "default",
    count: Optional[int] = None,
    bird_bin: str = "bird",
) -> Dict[str, Any]:
    """Search X using the local `bird` CLI.

    Returns a dict shaped similarly to xai_x.search_x() raw output so the
    rest of the pipeline can treat it as a raw provider response.
    """
    min_items, max_items = DEPTH_CONFIG.get(depth, DEPTH_CONFIG["default"])
    n = count or max_items

    query = f"{topic} since:{from_date} until:{to_date}"
    cmd = [bird_bin, "search", query, "--count", str(n), "--json"]

    try:
        code, out, err = _run(cmd, timeout=90 if depth == "quick" else 120)
    except subprocess.TimeoutExpired:
        return {"error": "bird search timed out"}
    except FileNotFoundError:
        return {"error": "bird binary not found"}

    if code != 0:
        return {"error": (err or out or f"bird exited with code {code}")[:2000]}

    try:
        data = json.loads(out)
    except Exception:
        return {"error": "bird returned non-JSON output", "_raw": out[:2000]}

    items: List[Dict[str, Any]] = []

    tweets = data if isinstance(data, list) else data.get("items") or data.get("tweets") or []

    for t in tweets:
        if not isinstance(t, dict):
            continue

        url = t.get("url")
        if not url and t.get("id"):
            url = f"https://x.com/i/web/status/{t['id']}"

        text = t.get("text") or t.get("full_text") or t.get("content")

        author = (
            t.get("author_handle")
            or t.get("authorHandle")
            or (t.get("author", {}) or {}).get("username")
            or (t.get("user", {}) or {}).get("screen_name")
        )

        created = t.get("date") or t.get("createdAt") or t.get("created_at")
        date_ymd = _parse_bird_created_at_to_ymd(created)

        # Engagement field names vary between bird outputs; best-effort.
        eng = t.get("engagement") or {}
        likes = _as_int(eng.get("likes") or t.get("favorite_count") or t.get("likeCount"))
        reposts = _as_int(eng.get("reposts") or t.get("retweet_count") or t.get("retweetCount"))
        replies = _as_int(eng.get("replies") or t.get("reply_count") or t.get("replyCount"))
        quotes = _as_int(eng.get("quotes") or t.get("quote_count") or t.get("quoteCount"))

        if not url or not text:
            continue

        items.append(
            {
                "text": str(text)[:500],
                "url": url,
                "author_handle": (author or "").lstrip("@"),
                "date": date_ymd,
                "engagement": {
                    "likes": likes,
                    "reposts": reposts,
                    "replies": replies,
                    "quotes": quotes,
                },
                "why_relevant": "Discovered via bird search",
                "relevance": 0.6,
            }
        )

    return {"items": items[: max(n, min_items)], "query": query, "provider": "bird"}


def parse_x_response(response: Dict[str, Any]) -> List[Dict[str, Any]]:
    """Parse bird-shaped response into normalized X items."""
    if not response:
        return []
    if response.get("error"):
        _log_error(str(response.get("error")))
        return []

    items = response.get("items")
    if not isinstance(items, list):
        return []

    clean = []
    for i, item in enumerate(items):
        if not isinstance(item, dict):
            continue
        url = item.get("url")
        if not url:
            continue

        eng = item.get("engagement") if isinstance(item.get("engagement"), dict) else None

        clean.append(
            {
                "id": f"X{i+1}",
                "text": str(item.get("text", "")).strip()[:500],
                "url": url,
                "author_handle": str(item.get("author_handle", "")).strip().lstrip("@"),
                "date": item.get("date"),
                "engagement": eng,
                "why_relevant": str(item.get("why_relevant", "")).strip(),
                "relevance": float(item.get("relevance", 0.5)),
            }
        )

    return clean
