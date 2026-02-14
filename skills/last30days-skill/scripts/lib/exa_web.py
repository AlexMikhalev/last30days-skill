"""Exa web search client (HTTP) for last30days.

We call Exa directly so the script can fetch web pages without relying on
Claude/OpenClaw WebSearch tool execution.

If Exa is unavailable or returns 403/401, the caller should fall back to the
"tool" web provider (WEBSEARCH REQUIRED marker).
"""

import json
from typing import Any, Dict, List, Optional

from . import http


EXA_SEARCH_URL = "https://api.exa.ai/search"


def search(
    api_key: str,
    query: str,
    from_date: str,
    to_date: str,
    num_results: int = 10,
    use_autoprompt: bool = True,
    search_type: str = "neural",
) -> Dict[str, Any]:
    headers = {
        # Exa docs have historically used x-api-key; some setups accept Bearer.
        "x-api-key": api_key,
        "content-type": "application/json",
    }

    payload: Dict[str, Any] = {
        "query": query,
        "numResults": num_results,
        "type": search_type,
        "useAutoprompt": use_autoprompt,
        "startPublishedDate": from_date,
        "endPublishedDate": to_date,
    }

    return http.post(EXA_SEARCH_URL, payload, headers=headers, timeout=60)


def parse_search_results(raw: Dict[str, Any]) -> List[Dict[str, Any]]:
    """Parse Exa search response into minimal {title,url,snippet,date} dicts."""
    results = raw.get("results") if isinstance(raw, dict) else None
    if not isinstance(results, list):
        return []

    items: List[Dict[str, Any]] = []
    for r in results:
        if not isinstance(r, dict):
            continue
        url = r.get("url")
        title = r.get("title") or ""
        snippet = r.get("snippet") or r.get("text") or ""
        date = r.get("publishedDate") or r.get("published_date")
        if not url:
            continue
        items.append(
            {
                "title": title,
                "url": url,
                "snippet": snippet,
                "date": date,
                "source": "exa",
            }
        )

    return items
