from __future__ import annotations

from pathlib import Path


def extract_epub(path: Path) -> str:
    try:
        from ebooklib import epub  # type: ignore
        from bs4 import BeautifulSoup  # type: ignore
    except Exception as e:
        raise RuntimeError(
            "EPUB dependencies missing. Install with `uv sync --extra epub` (ebooklib, bs4, lxml)."
        ) from e

    book = epub.read_epub(str(path))
    texts: list[str] = []
    for item in book.get_items():
        if item.get_type() == 9:  # DOCUMENT
            soup = BeautifulSoup(item.get_content(), "lxml")
            texts.append(soup.get_text("\n"))
    return "\n\n".join(texts)
