from __future__ import annotations

import hashlib


def sha256_hex(data: str) -> str:
    return hashlib.sha256(data.encode("utf-8")).hexdigest()


def doc_id(bucket: str, key: str, etag: str, chunk_id: int) -> str:
    return sha256_hex(f"{bucket}:{key}:{etag}:{chunk_id}")
