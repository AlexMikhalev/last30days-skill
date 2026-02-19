import os
import subprocess
from pathlib import Path
from typing import Dict


def load_env_file(path: str) -> Dict[str, str]:
    p = Path(path)
    if not p.exists():
        return {}
    env: Dict[str, str] = {}
    for raw in p.read_text("utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            continue
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")
    return env


def load_quickwit_env() -> Dict[str, str]:
    # /etc/quickwit/quickwit.env is root-readable only. Try direct read first,
    # then fall back to sudo -n cat.
    path = "/etc/quickwit/quickwit.env"
    try:
        return load_env_file(path)
    except PermissionError:
        try:
            cp = subprocess.run(
                ["sudo", "-n", "cat", path],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
                timeout=2,
            )
            if cp.returncode != 0:
                return {}
            env: Dict[str, str] = {}
            for raw in cp.stdout.decode("utf-8", "replace").splitlines():
                line = raw.strip()
                if not line or line.startswith("#") or "=" not in line:
                    continue
                k, v = line.split("=", 1)
                env[k.strip()] = v.strip().strip('"').strip("'")
            return env
        except Exception:
            return {}


def effective_aws_env() -> Dict[str, str]:
    # Prefer explicit env vars, fall back to quickwit system env file.
    env: Dict[str, str] = {}
    for k in [
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_REGION",
        "AWS_DEFAULT_REGION",
        "QW_S3_ENDPOINT",
        "QW_S3_FORCE_PATH_STYLE_ACCESS",
    ]:
        if os.environ.get(k):
            env[k] = os.environ[k]

    if env.get("AWS_ACCESS_KEY_ID") and env.get("AWS_SECRET_ACCESS_KEY"):
        if "AWS_REGION" in env and "AWS_DEFAULT_REGION" not in env:
            env["AWS_DEFAULT_REGION"] = env["AWS_REGION"]
        return env

    quickwit_env = load_quickwit_env()
    for k in [
        "AWS_ACCESS_KEY_ID",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_REGION",
        "AWS_DEFAULT_REGION",
        "QW_S3_ENDPOINT",
        "QW_S3_FORCE_PATH_STYLE_ACCESS",
    ]:
        if k in quickwit_env and k not in env:
            env[k] = quickwit_env[k]

    if "AWS_REGION" in env and "AWS_DEFAULT_REGION" not in env:
        env["AWS_DEFAULT_REGION"] = env["AWS_REGION"]

    return env
