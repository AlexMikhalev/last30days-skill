from __future__ import annotations

import argparse
import json
import os
import socket
import sys
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Optional

import yaml

from .lib.chunk import ChunkingConfig, chunk_text
from .lib.quickwit import Quickwit, load_index_config
from .lib.s5cmd import S5Cmd, parse_s5_ls
from .lib.state_store import StateStore, utc_now_iso
from .lib.util import doc_id, sha256_hex
from .lib.extract.text_md import extract_text_bytes
from .lib.extract.pdf import extract_pdf
from .lib.extract.epub import extract_epub


SUPPORTED_EXTS = {"txt", "md", "markdown", "pdf", "epub"}


@dataclass
class AppConfig:
    seaweed_endpoint: str
    state_bucket: str
    allow_buckets: list[str]
    extensions: list[str]
    quickwit_base: str
    quickwit_index: str
    max_files: int
    max_bytes: int
    workers_download: int
    workers_extract: int
    chunk_chars: int
    overlap_chars: int


def load_config(path: Path) -> AppConfig:
    data = yaml.safe_load(path.read_text("utf-8"))
    seaweed = data["seaweed"]
    qw = data["quickwit"]
    limits = data.get("limits", {})
    workers = data.get("workers", {})
    chunking = data.get("chunking", {})

    return AppConfig(
        seaweed_endpoint=seaweed["endpoint"],
        state_bucket=seaweed["state_bucket"],
        allow_buckets=list(seaweed["allow_buckets"]),
        extensions=list(seaweed["extensions"]),
        quickwit_base=qw["base_url"],
        quickwit_index=qw.get("index_id", "seaweed_docs"),
        max_files=int(limits.get("max_files", 1000)),
        max_bytes=int(limits.get("max_bytes", 500 * 1024 * 1024)),
        workers_download=int(workers.get("download", 4)),
        workers_extract=int(workers.get("extract", 2)),
        chunk_chars=int(chunking.get("chunk_chars", 12000)),
        overlap_chars=int(chunking.get("overlap_chars", 600)),
    )


def infer_extension(key: str) -> str:
    key_l = key.lower()
    if "." not in key_l:
        return ""
    return key_l.rsplit(".", 1)[-1]


def make_run_id() -> str:
    ts = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    host = socket.gethostname()
    return f"{ts}-{host}"


def s3_uri(bucket: str, key: str) -> str:
    return f"s3://{bucket}/{key}"


def list_candidate_objects(cfg: AppConfig, s5: S5Cmd, bucket: str, prefix: str) -> list[dict[str, Any]]:
    glob = f"s3://{bucket}/{prefix}*" if prefix else f"s3://{bucket}/*"
    out = s5.ls(glob)
    rows = parse_s5_ls(out)

    allowed_exts = {e.lower() for e in cfg.extensions}
    results: list[dict[str, Any]] = []
    for r in rows:
        key = r["key"]
        ext = infer_extension(key)
        if ext not in allowed_exts:
            continue
        results.append({**r, "ext": ext})
    return results


def head_etag(s5: S5Cmd, bucket: str, key: str) -> str:
    # s5cmd has no dedicated HEAD output; `ls` for exact object returns a single line with size, but not etag.
    # SeaweedFS S3 ETag is not accessible via s5cmd directly; for v1 we approximate etag using size+mtime+key.
    # If you later want real ETag, switch to boto3 or signed HEAD.
    #
    # Deterministic change token:
    #   sha256(key + last_modified + size)
    out = s5.ls(f"s3://{bucket}/{key}")
    rows = parse_s5_ls(out)
    if not rows:
        raise FileNotFoundError(key)
    r = rows[0]
    token = sha256_hex(f"{key}|{r['last_modified']}|{r['size']}")
    return token


def extract_file(ext: str, local_path: Path) -> str:
    if ext in ("txt", "md", "markdown"):
        return local_path.read_text("utf-8", errors="replace")
    if ext == "pdf":
        return extract_pdf(local_path)
    if ext == "epub":
        return extract_epub(local_path)
    raise RuntimeError(f"unsupported extension: {ext}")


def build_ndjson_docs(
    *,
    bucket: str,
    key: str,
    etag: str,
    last_modified: str,
    content_type: str,
    ext: str,
    text: str,
    chunk_cfg: ChunkingConfig,
) -> tuple[bytes, dict[str, Any]]:
    chunks = chunk_text(text, chunk_cfg)
    docs: list[dict[str, Any]] = []
    for i, c in enumerate(chunks):
        _id = doc_id(bucket, key, etag, i)
        docs.append(
            {
                "id": _id,
                "bucket": bucket,
                "key": key,
                "etag": etag,
                "last_modified": last_modified,
                "extension": ext,
                "content_type": content_type,
                "chunk_id": i,
                "source_uri": s3_uri(bucket, key),
                "text": c,
            }
        )

    ndjson = ("\n".join(json.dumps(d, ensure_ascii=False) for d in docs) + "\n").encode("utf-8")
    meta = {"chunks": {"count": len(chunks), "chars": len(text)}}
    return ndjson, meta


def cmd_init(cfg: AppConfig, qw: Quickwit) -> None:
    index_cfg_path = Path(__file__).parent / "schemas" / "quickwit-index.yaml"
    index_yaml = load_index_config(index_cfg_path)
    qw.ensure_index(cfg.quickwit_index, index_yaml)
    print(f"ok: ensured quickwit index {cfg.quickwit_index}")


def cmd_discover(cfg: AppConfig, s5: S5Cmd, bucket: str, prefix: str) -> None:
    objs = list_candidate_objects(cfg, s5, bucket, prefix)
    print(json.dumps({"bucket": bucket, "prefix": prefix, "count": len(objs), "objects": objs[:50]}, indent=2))


def cmd_run(cfg: AppConfig, s5: S5Cmd, state: StateStore, qw: Quickwit, bucket: str, prefix: str, max_files: int | None) -> None:
    run_id = make_run_id()
    started = utc_now_iso()

    objs = list_candidate_objects(cfg, s5, bucket, prefix)
    if max_files is None:
        max_files = cfg.max_files
    objs = objs[:max_files]

    processed = 0
    skipped = 0
    errors = 0

    chunk_cfg = ChunkingConfig(cfg.chunk_chars, cfg.overlap_chars)

    with tempfile.TemporaryDirectory(prefix="seaweed-qw-") as td:
        td_path = Path(td)
        for o in objs:
            key = o["key"]
            size = int(o["size"])
            last_modified = o["last_modified"]
            ext = o["ext"]
            if size > cfg.max_bytes:
                skipped += 1
                continue

            etag = head_etag(s5, bucket, key)
            manifest = state.load_manifest(bucket, key)
            if manifest and manifest.get("status") == "ok" and manifest.get("etag") == etag and int(manifest.get("size", -1)) == size:
                skipped += 1
                continue

            local = td_path / "file"
            try:
                s5.cp(f"s3://{bucket}/{key}", str(local))
                text = extract_file(ext, local)
                ndjson_bytes, meta = build_ndjson_docs(
                    bucket=bucket,
                    key=key,
                    etag=etag,
                    last_modified=last_modified,
                    content_type="application/octet-stream",
                    ext=ext,
                    text=text,
                    chunk_cfg=chunk_cfg,
                )
                qw.ingest_ndjson(cfg.quickwit_index, ndjson_bytes)

                state.save_manifest(
                    bucket,
                    key,
                    {
                        "version": 1,
                        "bucket": bucket,
                        "key": key,
                        "etag": etag,
                        "last_modified": last_modified,
                        "size": size,
                        "content_type": "application/octet-stream",
                        "extension": ext,
                        "status": "ok",
                        "run_id": run_id,
                        "indexed_at": utc_now_iso(),
                        **meta,
                        "error": None,
                    },
                )
                processed += 1
            except Exception as e:
                errors += 1
                try:
                    state.save_manifest(
                        bucket,
                        key,
                        {
                            "version": 1,
                            "bucket": bucket,
                            "key": key,
                            "etag": etag,
                            "last_modified": last_modified,
                            "size": size,
                            "content_type": "application/octet-stream",
                            "extension": ext,
                            "status": "error",
                            "run_id": run_id,
                            "indexed_at": utc_now_iso(),
                            "chunks": {"count": 0, "chars": 0},
                            "error": {"message": str(e), "type": type(e).__name__},
                        },
                    )
                except Exception:
                    pass

    summary = {
        "version": 1,
        "run_id": run_id,
        "started_at": started,
        "finished_at": utc_now_iso(),
        "bucket": bucket,
        "prefix": prefix,
        "totals": {"processed": processed, "skipped": skipped, "errors": errors, "candidates": len(objs)},
    }
    state.save_run(run_id, summary)
    print(json.dumps(summary, indent=2))


def cmd_verify(cfg: AppConfig, qw: Quickwit, bucket: str, prefix: str) -> None:
    # Verification mode: pick first 10 candidate files and ensure search can find docs by bucket/key.
    # We require that ingestion has been run already.
    s5 = S5Cmd(cfg.seaweed_endpoint)
    objs = list_candidate_objects(cfg, s5, bucket, prefix)[:10]
    if len(objs) < 10:
        raise SystemExit("Need at least 10 candidate files under bucket/prefix for verify.")

    results: list[dict[str, Any]] = []
    for o in objs:
        key = o["key"]
        q = f'bucket:{bucket} AND key:"{key}"'
        resp = qw.search(cfg.quickwit_index, q, max_hits=1)
        ok = int(resp.get("num_hits", 0)) >= 1
        results.append({"key": key, "query": q, "num_hits": resp.get("num_hits"), "ok": ok})

    all_ok = all(r["ok"] for r in results)
    print(json.dumps({"ok": all_ok, "results": results}, indent=2))
    if not all_ok:
        raise SystemExit(2)


def cmd_report(cfg: AppConfig, s5: S5Cmd) -> None:
    # v1: list last 5 run summaries (best effort)
    out = s5.ls(f"s3://{cfg.state_bucket}/v1/runs/*")
    rows = parse_s5_ls(out)
    rows = sorted(rows, key=lambda r: r["last_modified"], reverse=True)[:5]
    print(json.dumps({"runs": rows}, indent=2))


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(prog="ingestor")
    p.add_argument("--config", default=str(Path(__file__).resolve().parents[1] / ".." / "config.yaml"))

    sub = p.add_subparsers(dest="cmd", required=True)

    sub.add_parser("init")

    d = sub.add_parser("discover")
    d.add_argument("--bucket", required=True)
    d.add_argument("--prefix", default="")

    r = sub.add_parser("run")
    r.add_argument("--bucket", required=True)
    r.add_argument("--prefix", default="")
    r.add_argument("--max-files", type=int, default=None)

    v = sub.add_parser("verify")
    v.add_argument("--bucket", required=True)
    v.add_argument("--prefix", default="")

    rep = sub.add_parser("report")
    rep.add_argument("--last", action="store_true")

    return p


def main(argv: Optional[list[str]] = None) -> None:
    args = build_parser().parse_args(argv)
    cfg_path = Path(args.config).resolve()
    cfg = load_config(cfg_path)

    if args.cmd in {"discover", "run", "verify"}:
        if args.bucket not in cfg.allow_buckets:
            raise SystemExit(f"bucket {args.bucket} not in allowlist")

    s5 = S5Cmd(cfg.seaweed_endpoint)
    state = StateStore(s5=s5, bucket=cfg.state_bucket)
    qw = Quickwit(cfg.quickwit_base)

    if args.cmd == "init":
        cmd_init(cfg, qw)
    elif args.cmd == "discover":
        cmd_discover(cfg, s5, args.bucket, args.prefix)
    elif args.cmd == "run":
        cmd_run(cfg, s5, state, qw, args.bucket, args.prefix, args.max_files)
    elif args.cmd == "verify":
        cmd_verify(cfg, qw, args.bucket, args.prefix)
    elif args.cmd == "report":
        cmd_report(cfg, s5)
    else:
        raise SystemExit("unknown cmd")


if __name__ == "__main__":
    main()
