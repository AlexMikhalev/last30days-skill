from __future__ import annotations

import json
import urllib.parse
import urllib.request
from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class Filer:
    base_url: str  # e.g. http://rpi4node3:8888

    def _url(self, path: str) -> str:
        if not path.startswith("/"):
            path = "/" + path
        return self.base_url.rstrip("/") + path

    def list_bucket_prefix(self, bucket: str, prefix: str, *, limit: int = 1000) -> dict[str, Any]:
        # SeaweedFS Filer S3-bucket view:
        #   /buckets/<bucket>/<prefix>/?limit=N
        # Returns JSON when Accept: application/json
        prefix = prefix.lstrip("/")
        path = f"/buckets/{bucket}/" + prefix
        if not path.endswith("/"):
            path += "/"
        url = self._url(path) + "?" + urllib.parse.urlencode({"limit": str(limit)})
        req = urllib.request.Request(url, headers={"Accept": "application/json"})
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = resp.read()
        return json.loads(data.decode("utf-8", "replace"))

    def download_full_path(self, full_path: str) -> bytes:
        # full_path is like /buckets/books/....
        url = self._url(full_path)
        req = urllib.request.Request(url)
        with urllib.request.urlopen(req, timeout=60) as resp:
            return resp.read()
