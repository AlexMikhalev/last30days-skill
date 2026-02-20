# Research Document: SeaweedFS → Quickwit ingestion pipeline (text/md/pdf/epub)

**Status**: Draft
**Author**: OpenClaw agent
**Date**: 2026-02-19
**Reviewers**: Alex

## Executive Summary

We want an automated, repeatable pipeline that crawls an allowlisted set of SeaweedFS S3 buckets, extracts text from supported document types (txt/md/pdf/epub), chunks it, and indexes it into Quickwit running on linux-small-box. The core challenges are: (1) reliable change detection and idempotency, (2) safe handling of large / binary / malformed files, (3) maintaining a durable state/checkpoint store in Seaweed itself, and (4) operational guardrails (rate limits, retries, observability, and backpressure).

## Essential Questions Check

| Question | Answer | Evidence |
|----------|--------|----------|
| Energizing? | Yes | Enables unified search across personal docs and backups; immediate utility. |
| Leverages strengths? | Yes | Existing SeaweedFS S3 gateway and Quickwit already running; we can wire them together. |
| Meets real need? | Yes | Explicit request to index all docs across buckets and prove search works. |

**Proceed**: Yes (3/3)

## Problem Statement

### Description
Build a disciplined ingestion system that indexes documents stored in SeaweedFS (via its S3 gateway) into Quickwit for full-text search.

### Impact
- Primary user: Alex.
- Impact: search and retrieval across many buckets; reduces manual hunting; enables downstream agents/automation.

### Success Criteria
1. For a given test set of N documents, each is searchable in Quickwit by a unique identifier and by a content snippet.
2. Re-running the ingestion does not create duplicate logical documents for unchanged inputs (idempotent behavior).
3. State/checkpoint is persisted in a dedicated Seaweed bucket (`quickwit_ingest_state`) and survives restarts.
4. Pipeline can be safely run on a schedule without destabilizing Quickwit or the host.

## Current State Analysis

### Existing Implementation
- SeaweedFS cluster exists with master at `rpi4node3:9333` and S3 gateway at `http://rpi4node3:8333/`.
- Quickwit installed on linux-small-box, running via systemd, configured with:
  - `default_index_root_uri: s3://quickwit/indexes`
  - S3 endpoint `http://rpi4node3:8333`, path-style.
- Verified previously: Quickwit can write split objects to Seaweed S3 and read/search them.

### Integration Points
- Seaweed S3 API: list objects, head objects (etag/mtime/size), get objects.
- Quickwit REST ingest API: create indexes, ingest docs, query.

## Constraints

### Technical Constraints
- Storage backend is S3-compatible (SeaweedFS), likely path-style required.
- Documents may be very large; extraction tools can be CPU/memory heavy (pdf/epub).
- We must avoid indexing from non-allowlisted buckets.

### Non-Functional Requirements
| Requirement | Target | Notes |
|-------------|--------|------|
| Idempotency | No duplicate logical docs on rerun | Achieved via deterministic doc IDs + etag-based change detection |
| Safety | Hard limits on file size and concurrency | prevent host overload |
| Durability | State stored in Seaweed bucket | no reliance on local sqlite unless optional |
| Observability | run logs + per-object manifests | enables debugging |

## Vital Few (Essentialism)

### Essential Constraints (Max 3)
| Constraint | Why It's Vital | Evidence |
|------------|----------------|----------|
| Allowlist buckets only | Prevents indexing sensitive/irrelevant data | User explicitly requested allowlist |
| State in Seaweed | Survives restarts/rebuilds; central source of truth | User explicitly requested |
| Deterministic IDs + change detection | Enables idempotent runs | Required for safe scheduling |

### Eliminated from Scope (v1)
| Eliminated Item | Why Eliminated |
|----------------|----------------|
| OCR for scanned PDFs/images | Adds heavy deps + complexity; not required for v1 |
| Multi-node distributed ingest workers | v1 is single host runner |
| Real-time filesystem watch | Use periodic runs instead |
| Perfect exactly-once semantics | S3 etag/mtime is best-effort; acceptable |
| Index-per-bucket | Extra management; start with one index |

## Dependencies

### External Dependencies
| Dependency | Purpose | Risk | Alternative |
|------------|---------|------|-------------|
| s5cmd | fast S3 list/copy | Low | awscli, rclone |
| poppler (pdftotext) | PDF text extraction | Med (format edge cases) | pymupdf |
| Python libs (ebooklib/bs4/lxml) | EPUB extraction | Med | calibre ebook-convert |

## Risks and Unknowns

### Known Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Very large files overwhelm CPU/RAM | Med | High | size limits, bounded concurrency |
| PDF/EPUB extraction failures | High | Med | fallback extractor, error manifests |
| Seaweed S3 listing scale/timeouts | Med | Med | paginate by prefix, retries |
| Duplicate docs on content changes | Med | Med | etag-based ids; optional garbage collection later |

### Open Questions
1. Do we need to index `shared_encrypted`? (Currently **No**, excluded by allowlist.)
2. How to handle deletes? (v1: keep stale docs; v2: garbage collect via manifests.)

### Assumptions
1. Seaweed S3 provides stable ETag/Last-Modified for objects.
2. Quickwit REST API is reachable locally and can sustain ingestion load with batching.

## Research Findings

### Key Insights
1. Deterministic doc IDs + state manifests make the system restart-safe and rerun-safe.
2. A “KV store” can be implemented simply as objects in a dedicated S3 bucket (hashed key namespace).
3. Extraction should be modular with per-type tools and clear fallbacks.

### Technical Spikes Needed
| Spike | Purpose | Estimated Effort |
|-------|---------|------------------|
| Validate Seaweed S3 HEAD semantics | confirm etag/mtime/size reliability | 1-2h |
| Extract sample EPUB/PDF corpus | confirm tool quality and speed | 2-4h |

## Recommendations

### Proceed/No-Proceed
Proceed.

### Scope Recommendations
- Build v1 with: allowlist, extension filter, extraction+chunking, Quickwit ingest, Seaweed state bucket.
- Add v2 later: delete detection + garbage collection, OCR, richer metadata.

### Risk Mitigation Recommendations
- Add strict guardrails (max bytes, max files, max concurrency).
- Store per-object error details to state bucket for auditing.
