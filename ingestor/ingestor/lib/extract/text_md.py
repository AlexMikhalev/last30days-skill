from __future__ import annotations


def extract_text_bytes(data: bytes) -> str:
    # Best-effort utf-8 decode.
    return data.decode("utf-8", errors="replace")
