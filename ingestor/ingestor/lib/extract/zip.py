"""Extract text from ZIP archives by extracting and processing all supported files inside."""

from __future__ import annotations

import zipfile
import tempfile
from pathlib import Path
from typing import Set

from .text_md import extract_text_bytes
from .pdf import extract_pdf
from .epub import extract_epub


SUPPORTED_EXTS = {"txt", "md", "markdown", "pdf", "epub"}


def infer_extension(filename: str) -> str:
    """Infer file extension from filename."""
    filename_l = filename.lower()
    if "." not in filename_l:
        return ""
    return filename_l.rsplit(".", 1)[-1]


def extract_file_from_zip(local_path: Path, ext: str) -> str:
    """Extract text from a single file based on its extension."""
    if ext in ("txt", "md", "markdown"):
        return local_path.read_text("utf-8", errors="replace")
    if ext == "pdf":
        return extract_pdf(local_path)
    if ext == "epub":
        return extract_epub(local_path)
    return ""


def extract_zip(local_path: Path) -> str:
    """
    Extract text from a ZIP archive by processing all supported files inside.
    
    Args:
        local_path: Path to the ZIP file
        
    Returns:
        Combined text from all supported files in the archive
    """
    texts = []
    
    with zipfile.ZipFile(local_path, 'r') as zip_ref:
        # Get list of files in the ZIP
        file_list = zip_ref.namelist()
        
        # Create a temporary directory to extract files
        with tempfile.TemporaryDirectory(prefix="zip-extract-") as temp_dir:
            temp_path = Path(temp_dir)
            
            for filename in file_list:
                # Skip directories
                if filename.endswith('/'):
                    continue
                
                # Check if file has a supported extension
                ext = infer_extension(filename)
                if ext not in SUPPORTED_EXTS:
                    continue
                
                try:
                    # Extract the file to temp directory
                    extracted_path = temp_path / Path(filename).name
                    with zip_ref.open(filename) as source, open(extracted_path, 'wb') as target:
                        target.write(source.read())
                    
                    # Extract text from the file
                    text = extract_file_from_zip(extracted_path, ext)
                    if text.strip():
                        # Add a header to identify which file this came from
                        texts.append(f"\n\n=== {filename} ===\n\n{text}")
                        
                except Exception as e:
                    # Log error but continue processing other files
                    print(f"Warning: Failed to extract {filename} from ZIP: {e}")
                    continue
    
    # Combine all extracted text
    combined_text = "\n".join(texts)
    return combined_text
