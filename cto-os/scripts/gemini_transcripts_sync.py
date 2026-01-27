#!/usr/bin/env python3
"""Sync Gemini meeting transcripts/notes from Gmail into a local folder.

Assumptions
- Notes arrive as emails from: Gemini <gemini-notes@google.com>
- We store each email as a raw .eml for fidelity.

This script is intentionally dependency-free (stdlib only).
It shells out to `gog` for Gmail access.

Usage:
  python3 cto-os/scripts/gemini_transcripts_sync.py \
    --out-dir cto-os/transcripts/gemini \
    --query "from:gemini-notes@google.com subject:Notes" \
    --max 500

State:
- Writes processed Gmail message IDs to: <out-dir>/.state.json
"""

from __future__ import annotations

import argparse
import base64
import json
import os
import re
import subprocess
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any

DEFAULT_QUERY = "from:gemini-notes@google.com subject:Notes"


def run(cmd: list[str]) -> str:
    p = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if p.returncode != 0:
        raise RuntimeError(f"Command failed ({p.returncode}): {' '.join(cmd)}\n{p.stderr.strip()}")
    return p.stdout


def slugify(s: str) -> str:
    s = s.strip().lower()
    s = re.sub(r"['\"]", "", s)
    s = re.sub(r"[^a-z0-9]+", "-", s)
    s = re.sub(r"-+", "-", s).strip("-")
    return s[:120] or "untitled"


def parse_date(s: str) -> datetime:
    # Example: "2026-01-26 10:24" (from gog search output)
    return datetime.strptime(s, "%Y-%m-%d %H:%M")


@dataclass
class MsgRef:
    id: str
    date: str
    from_: str
    subject: str


def load_state(path: Path) -> set[str]:
    if not path.exists():
        return set()
    obj = json.loads(path.read_text(encoding="utf-8"))
    return set(obj.get("processed_ids", []))


def save_state(path: Path, processed_ids: set[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(".tmp")
    tmp.write_text(json.dumps({"processed_ids": sorted(processed_ids)}, indent=2), encoding="utf-8")
    tmp.replace(path)


def gog_search(query: str, max_results: int) -> list[MsgRef]:
    out = run(["gog", "gmail", "messages", "search", query, "--max", str(max_results), "--json", "--no-input"])
    data = json.loads(out)
    msgs = []
    for m in data.get("messages", []):
        msgs.append(
            MsgRef(
                id=m["id"],
                date=m.get("date") or "",
                from_=m.get("from") or "",
                subject=m.get("subject") or "",
            )
        )
    return msgs


def gog_get_raw(message_id: str) -> dict[str, Any]:
    out = run(["gog", "gmail", "get", message_id, "--format", "raw", "--json", "--no-input"])
    return json.loads(out)


def extract_raw_b64(obj: dict[str, Any]) -> str | None:
    # gog returns {"message": {"raw": ...}, ...}
    if isinstance(obj.get("raw"), str):
        return obj["raw"]
    msg = obj.get("message")
    if isinstance(msg, dict) and isinstance(msg.get("raw"), str):
        return msg.get("raw")
    return None


def b64url_decode(s: str) -> bytes:
    # Gmail returns base64url without padding
    s = s.replace("-", "+").replace("_", "/")
    pad = (4 - (len(s) % 4)) % 4
    s = s + ("=" * pad)
    return base64.b64decode(s)


def subject_title(subject: str) -> str:
    # Often: Notes: 'Meeting name' 26 Jan 2026
    m = re.search(r"Notes:\s*'([^']+)'", subject)
    if m:
        return m.group(1).strip()
    # Fallback: strip leading "Notes:"
    return re.sub(r"^Notes:\s*", "", subject).strip()


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", type=Path, default=Path("cto-os/transcripts/gemini"))
    ap.add_argument("--query", type=str, default=DEFAULT_QUERY)
    ap.add_argument("--max", type=int, default=500)
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument(
        "--repair-missing",
        action="store_true",
        help="If set, repair existing .meta.json entries missing .eml files.",
    )
    args = ap.parse_args()

    out_dir: Path = args.out_dir
    state_path = out_dir / ".state.json"

    processed = load_state(state_path)

    # Optional: repair pass
    repaired = 0
    if args.repair_missing:
        for meta_path in out_dir.glob("**/*.meta.json"):
            try:
                meta = json.loads(meta_path.read_text(encoding="utf-8"))
            except Exception:
                continue
            # meta files are named <base>.meta.json, but the raw email should be <base>.eml
            eml_path = Path(str(meta_path).replace(".meta.json", ".eml"))
            if eml_path.exists():
                continue
            mid = meta.get("id")
            if not isinstance(mid, str) or not mid:
                continue
            raw_obj = gog_get_raw(mid)
            raw = extract_raw_b64(raw_obj)
            if not raw:
                continue
            eml_path.write_bytes(b64url_decode(raw))
            # Clear previous error if any
            meta.pop("error", None)
            meta.pop("raw_obj", None)
            meta_path.write_text(json.dumps(meta, indent=2), encoding="utf-8")
            repaired += 1

    msgs = gog_search(args.query, args.max)
    # Oldest-first so folder structure is orderly
    msgs = list(reversed(msgs))

    new_count = 0
    for m in msgs:
        if m.id in processed:
            continue
        if not m.date:
            continue

        dt = parse_date(m.date)
        title = subject_title(m.subject)
        slug = slugify(title)

        dest_dir = out_dir / f"{dt:%Y}" / f"{dt:%m}" / f"{dt:%Y-%m-%d}"
        dest_dir.mkdir(parents=True, exist_ok=True)

        base = f"{dt:%Y-%m-%d_%H%M}_{slug}"
        meta_path = dest_dir / f"{base}.meta.json"
        eml_path = dest_dir / f"{base}.eml"

        if args.dry_run:
            print(f"[DRY] would save {m.id} -> {eml_path}")
            processed.add(m.id)
            new_count += 1
            continue

        raw_obj = gog_get_raw(m.id)
        raw = extract_raw_b64(raw_obj)
        if not raw:
            # still mark, but keep evidence
            meta_path.write_text(
                json.dumps(
                    {
                        "id": m.id,
                        "date": m.date,
                        "from": m.from_,
                        "subject": m.subject,
                        "error": "missing raw payload",
                        "raw_obj": raw_obj,
                    },
                    indent=2,
                ),
                encoding="utf-8",
            )
            processed.add(m.id)
            new_count += 1
            continue

        eml_path.write_bytes(b64url_decode(raw))

        meta_path.write_text(
            json.dumps(
                {
                    "id": m.id,
                    "date": m.date,
                    "from": m.from_,
                    "subject": m.subject,
                    "title": title,
                    "query": args.query,
                },
                indent=2,
            ),
            encoding="utf-8",
        )

        processed.add(m.id)
        new_count += 1

    save_state(state_path, processed)
    print(f"Repaired {repaired} missing .eml; synced {new_count} new transcript emails into {out_dir}")


if __name__ == "__main__":
    main()
