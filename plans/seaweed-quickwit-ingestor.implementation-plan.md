# Implementation Plan: SeaweedFS → Quickwit ingestor (v1)

**Status**: Draft
**Research Doc**: `plans/seaweed-quickwit-ingestor.research.md`
**Author**: OpenClaw agent
**Date**: 2026-02-19
**Estimated Effort**: 1-2 days (v1), +1-2 days for hardening

## Overview

### Summary
Implement a single-host ingestion CLI that:
- crawls allowlisted SeaweedFS S3 buckets
- selects only txt/md/pdf/epub
- extracts and chunks text
- ingests into Quickwit index `seaweed_docs`
- persists per-object state in Seaweed bucket `quickwit_ingest_state`

### Approach
- Use S3-compatible access via `s5cmd` (fast list/copy) and optionally Python `boto3` later.
- Implement Seaweed-backed state store as objects (manifest JSON + key reverse map).
- Deterministic document IDs based on `(bucket,key,etag,chunk_id)`.

### Scope
**In Scope (Top 5):**
1. Allowlist buckets: `books`, `backups`, `alex`, `alex_linux`, `common_linux`, `mac_home_alex`, `mac_home_karina`, `icloud_drive`, `sharedbucket`, `upload`
2. Filter extensions: `.txt .md .markdown .pdf .epub`
3. Extraction modules for each type, with clear errors
4. Chunking + ingest into Quickwit
5. Persistent state bucket `quickwit_ingest_state` (manifests + runs)

**Out of Scope:**
- OCR
- delete detection and purge (garbage collection)
- index-per-bucket
- distributed workers

**Avoid At All Cost (distractions):**
- implementing a full database/metastore for state (S3 objects is enough)
- adding fancy semantic enrichment before basic reliability is proven

## Architecture

### Component Diagram

Seaweed S3 (rpi4node3:8333)
  | (list/head/get)
  v
Ingestor CLI (linux-small-box)
  - Lister (s5cmd)
  - StateStore (S3 bucket quickwit_ingest_state)
  - Downloader (s5cmd cp)
  - Extractors (txt/md/pdf/epub)
  - Chunker
  - QuickwitClient (REST ingest)
  | (ingest)
  v
Quickwit (systemd service)
  | (splits)
  v
Seaweed S3 bucket quickwit/indexes/...

### Key Design Decisions
| Decision | Rationale | Alternatives Rejected |
|----------|-----------|----------------------|
| State stored in S3 bucket objects | Durable, simple, auditable | local-only sqlite; external postgres |
| Deterministic IDs using etag | Idempotent reruns, safe updates | random UUIDs |
| Single index `seaweed_docs` | simpler management | index per bucket |
| s5cmd for list/cp | fast, already installed | awscli, rclone |

### Simplicity Check
The simplest viable system is: list → filter → download → extract → chunk → ingest → write manifest. This plan keeps that straight-line pipeline and avoids premature features.

## File Changes (planned)

### New Files
| File | Purpose |
|------|---------|
| `ingestor/config.yaml` | endpoints, allowlist, limits |
| `ingestor/ingest.py` | CLI entrypoint |
| `ingestor/lib/s5cmd.py` | wrapper for list/cp |
| `ingestor/lib/state_store.py` | read/write manifests in `quickwit_ingest_state` |
| `ingestor/lib/extract/text_md.py` | txt/md extraction |
| `ingestor/lib/extract/pdf.py` | pdf extraction |
| `ingestor/lib/extract/epub.py` | epub extraction |
| `ingestor/lib/chunk.py` | chunking strategy |
| `ingestor/lib/quickwit.py` | create index, ingest, search verification |
| `ingestor/schemas/quickwit-index.yaml` | index config |

### Modified Files
None required in existing OpenClaw core.

## API Design

### CLI

Commands:
- `ingest.py init` (create buckets/index if missing)
- `ingest.py discover --bucket books` (dry run listing)
- `ingest.py run --bucket books --max-files 1000` (do ingestion)
- `ingest.py verify --bucket books --key <key>` (prove searches)
- `ingest.py report --last` (run summary)

Common flags:
- `--endpoint http://rpi4node3:8333`
- `--state-bucket quickwit_ingest_state`
- `--quickwit http://127.0.0.1:7280`
- `--workers-download N` `--workers-extract N`
- `--max-bytes` `--max-files`
- `--dry-run`

### StateStore functions (Python)

```python
def manifest_key(bucket: str, key: str) -> str:
    """Return S3 key for the manifest JSON object."""

def load_manifest(bucket: str, key: str) -> dict | None:
    """Load manifest from state bucket if present."""

def save_manifest(bucket: str, key: str, manifest: dict) -> None:
    """Persist manifest JSON and reverse mapping."""

def save_run(run_id: str, summary: dict) -> None:
    """Write run summary to v1/runs/<run_id>.json"""
```

### Quickwit client functions

```python
def ensure_index(index_id: str, index_config_yaml: str) -> None

def ingest_ndjson(index_id: str, ndjson_bytes: bytes) -> None

def search(index_id: str, query: str, max_hits: int = 3) -> dict
```

## Data Model

### Manifest JSON (v1)

```json
{
  "version": 1,
  "bucket": "books",
  "key": "path/to/file.pdf",
  "etag": "...",
  "last_modified": "2026-02-18T00:00:00Z",
  "size": 12345,
  "content_type": "application/pdf",
  "extension": "pdf",
  "status": "ok",
  "run_id": "2026-02-19T00:00:00Z-linux-small-box",
  "indexed_at": "2026-02-19T00:00:10Z",
  "chunks": {
    "count": 12,
    "chars": 54321
  },
  "error": null
}
```

## Test Strategy

### Unit Tests
- chunker: stable chunk boundaries
- id generation: deterministic
- manifest key hashing: stable

### Integration Tests (local)
1. Create a test index.
2. Upload 10 small text docs to a test prefix in an allowlisted bucket.
3. Run `ingest.py run --max-files 10`.
4. For each doc, query `bucket:<b> AND key:<k>` or `id:<id>` and assert at least 1 hit.

### Acceptance Test (proof)
- Same as above but across multiple file types: 2 txt, 2 md, 3 pdf, 3 epub.

## Step-by-step Implementation Sequence

1. **Bootstrap project**: create `ingestor/` layout, config, uv environment.
2. **Implement s5cmd wrapper**: list and copy with endpoint + credentials.
3. **Implement StateStore**: get/put JSON objects into `quickwit_ingest_state` using s5cmd.
4. **Implement extractors**:
   - txt/md (basic)
   - pdf via `pdftotext` (poppler)
   - epub via python ebooklib
5. **Implement chunker** (size + overlap).
6. **Implement Quickwit client** (ensure index, ingest).
7. **Implement run loop** with guardrails and concurrency.
8. **Add verify mode**: prove each of 10 docs is searchable.
9. **Write run summaries + error manifests**.

## Rollout Plan

- Phase 0: run on `books` only, max 100 files.
- Phase 1: add remaining allowlisted buckets, still with conservative limits.
- Phase 2: schedule via systemd timer.

## Open Items (v2)
- Delete detection + purge of old etags
- OCR pipeline
- Semantic enrichment (entities/tags)
