from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class ChunkingConfig:
    chunk_chars: int
    overlap_chars: int


def chunk_text(text: str, cfg: ChunkingConfig) -> list[str]:
    if cfg.chunk_chars <= 0:
        raise ValueError("chunk_chars must be > 0")
    if cfg.overlap_chars < 0:
        raise ValueError("overlap_chars must be >= 0")
    if cfg.overlap_chars >= cfg.chunk_chars:
        raise ValueError("overlap_chars must be < chunk_chars")

    text = text.strip("\n")
    if not text:
        return []

    chunks: list[str] = []
    step = cfg.chunk_chars - cfg.overlap_chars
    i = 0
    n = len(text)
    while i < n:
        chunk = text[i : i + cfg.chunk_chars]
        chunk = chunk.strip()
        if chunk:
            chunks.append(chunk)
        i += step
    return chunks
