from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path

import requests


@dataclass(frozen=True)
class Quickwit:
    base_url: str

    def _url(self, path: str) -> str:
        return self.base_url.rstrip("/") + path

    def list_indexes(self) -> list[dict]:
        r = requests.get(self._url("/api/v1/indexes"), timeout=10)
        r.raise_for_status()
        return r.json()

    def ensure_index(self, index_id: str, index_config_yaml: str) -> None:
        existing = [x.get("index_config", {}).get("index_id") for x in self.list_indexes()]
        if index_id in existing:
            return
        r = requests.post(
            self._url("/api/v1/indexes"),
            data=index_config_yaml.encode("utf-8"),
            headers={"Content-Type": "application/yaml"},
            timeout=20,
        )
        r.raise_for_status()

    def ingest_ndjson(self, index_id: str, ndjson_bytes: bytes) -> dict:
        r = requests.post(
            self._url(f"/api/v1/{index_id}/ingest"),
            data=ndjson_bytes,
            headers={"Content-Type": "application/json"},
            timeout=60,
        )
        r.raise_for_status()
        return r.json()

    def search(self, index_id: str, query: str, max_hits: int = 3) -> dict:
        r = requests.get(
            self._url(f"/api/v1/{index_id}/search"),
            params={"query": query, "max_hits": max_hits},
            timeout=20,
        )
        r.raise_for_status()
        return r.json()


def load_index_config(path: Path) -> str:
    return path.read_text("utf-8")
