from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass
from typing import Optional


@dataclass(frozen=True)
class S5Cmd:
    endpoint: str

    def _base_args(self) -> list[str]:
        return ["s5cmd", "--endpoint-url", self.endpoint]

    def run(self, args: list[str], *, input_bytes: bytes | None = None) -> subprocess.CompletedProcess:
        cmd = self._base_args() + args
        return subprocess.run(
            cmd,
            input=input_bytes,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )

    def ls(self, uri_glob: str) -> str:
        cp = self.run(["ls", uri_glob])
        if cp.returncode != 0:
            raise RuntimeError(f"s5cmd ls failed ({cp.returncode}): {cp.stderr.decode('utf-8', 'ignore')}")
        return cp.stdout.decode("utf-8", "ignore")

    def cp(self, src: str, dst: str) -> None:
        cp = self.run(["cp", src, dst])
        if cp.returncode != 0:
            raise RuntimeError(f"s5cmd cp failed ({cp.returncode}): {cp.stderr.decode('utf-8', 'ignore')}")

    def cat(self, uri: str) -> bytes:
        cp = self.run(["cat", uri])
        if cp.returncode != 0:
            raise FileNotFoundError(cp.stderr.decode("utf-8", "ignore"))
        return cp.stdout

    def pipe(self, uri: str, data: bytes) -> None:
        cp = self.run(["pipe", uri], input_bytes=data)
        if cp.returncode != 0:
            raise RuntimeError(f"s5cmd pipe failed ({cp.returncode}): {cp.stderr.decode('utf-8', 'ignore')}")


def parse_s5_ls(output: str) -> list[dict]:
    """Parse `s5cmd ls` output lines.

    Expected line format:
    YYYY/MM/DD HH:MM:SS  SIZE  key
    or for buckets listing, user provided list outside this tool.

    Returns list of dicts with keys: last_modified(str), size(int), key(str)
    """
    rows: list[dict] = []
    for line in output.splitlines():
        line = line.strip()
        if not line:
            continue
        parts = line.split(maxsplit=3)
        if len(parts) < 4:
            continue
        date, time, size_s, key = parts
        try:
            size = int(size_s)
        except ValueError:
            continue
        rows.append({"last_modified": f"{date} {time}", "size": size, "key": key})
    return rows
