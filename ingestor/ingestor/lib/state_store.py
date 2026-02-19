from __future__ import annotations

import json
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Any, Optional

from .s5cmd import S5Cmd
from .util import sha256_hex


@dataclass(frozen=True)
class StateStore:
    s5: S5Cmd
    bucket: str

    def _manifest_path(self, bucket: str, key: str) -> str:
        h = sha256_hex(key)
        return f"s3://{self.bucket}/v1/manifests/{bucket}/{h}.json"

    def _key_path(self, bucket: str, key: str) -> str:
        h = sha256_hex(key)
        return f"s3://{self.bucket}/v1/keys/{bucket}/{h}.txt"

    def _run_path(self, run_id: str) -> str:
        return f"s3://{self.bucket}/v1/runs/{run_id}.json"

    def load_manifest(self, bucket: str, key: str) -> Optional[dict[str, Any]]:
        uri = self._manifest_path(bucket, key)
        try:
            data = self.s5.cat(uri)
        except FileNotFoundError:
            return None
        try:
            return json.loads(data.decode("utf-8"))
        except Exception:
            return None

    def save_manifest(self, bucket: str, key: str, manifest: dict[str, Any]) -> None:
        manifest_uri = self._manifest_path(bucket, key)
        key_uri = self._key_path(bucket, key)
        self.s5.pipe(key_uri, (key + "\n").encode("utf-8"))
        self.s5.pipe(manifest_uri, json.dumps(manifest, ensure_ascii=False, indent=2).encode("utf-8"))

    def save_run(self, run_id: str, summary: dict[str, Any]) -> None:
        uri = self._run_path(run_id)
        self.s5.pipe(uri, json.dumps(summary, ensure_ascii=False, indent=2).encode("utf-8"))


def utc_now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()
