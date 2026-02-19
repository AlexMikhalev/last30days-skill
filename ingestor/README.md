# SeaweedFS → Quickwit ingestor (v1)

Indexes **.txt / .md / .pdf / .epub** documents from an allowlisted set of SeaweedFS S3 buckets into Quickwit.

## Prereqs

- Quickwit running on this machine (default: `http://127.0.0.1:7280`)
- SeaweedFS S3 gateway reachable (default: `http://rpi4node3:8333`)
- `s5cmd` installed (we use it for list/copy and for the state bucket KV store)
- PDF support: `pdftotext` available (install via `brew install poppler`)
- EPUB support: install optional deps: `uv sync --extra epub`

## Install (uv)

From `/home/alex/clawd/ingestor`:

```bash
cd /home/alex/clawd/ingestor
uv venv
uv sync
# optional epub support
uv sync --extra epub
```

## Configure

Edit `config.yaml`.

## Commands

```bash
# Create quickwit index if needed
uv run ingestor init

# List candidate files (dry run)
uv run ingestor discover --bucket books

# Run ingestion
uv run ingestor run --bucket books --max-files 50

# Verify 10 docs (provide a bucket+prefix containing at least 10 supported files)
uv run ingestor verify --bucket books --prefix 'some/path/'

# Show last run summary
uv run ingestor report --last
```

## State bucket layout

All state is persisted to `s3://<state_bucket>/`:

- `v1/manifests/<bucket>/<sha256(key)>.json`
- `v1/keys/<bucket>/<sha256(key)>.txt` (reverse mapping)
- `v1/runs/<run_id>.json`

## Notes

- Idempotency: if manifest `status=ok` and `(etag,size)` unchanged, the object is skipped.
- Deterministic chunk doc IDs: `sha256(bucket:key:etag:chunk_id)`.
- v1 does not purge old docs when an object changes (future GC step).
