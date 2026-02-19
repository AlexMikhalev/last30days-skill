from __future__ import annotations

import shutil
import subprocess
from pathlib import Path


def extract_pdf(path: Path) -> str:
    if shutil.which("pdftotext") is None:
        raise RuntimeError("pdftotext not found. Install poppler (e.g. `brew install poppler`).")

    # pdftotext writes to a file; use a temp sibling.
    out_txt = path.with_suffix(path.suffix + ".txt")
    try:
        cp = subprocess.run(
            ["pdftotext", "-layout", str(path), str(out_txt)],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
        if cp.returncode != 0:
            raise RuntimeError(f"pdftotext failed: {cp.stderr.decode('utf-8','ignore')}")
        return out_txt.read_text("utf-8", errors="replace")
    finally:
        try:
            if out_txt.exists():
                out_txt.unlink()
        except Exception:
            pass
