from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Iterator, Literal

import httpx
import numpy as np
import pandas as pd
from scipy import stats

POLYMARKET_PROFILE_RE = re.compile(r"https?://(?:www\.)?polymarket\.com/@(?P<handle>[A-Za-z0-9_\-]+)")
ETH_ADDR_RE = re.compile(r"0x[a-fA-F0-9]{40}")


@dataclass(frozen=True)
class WalletSummary:
    wallet: str
    n: int
    wins: int
    profit: float

    @property
    def winrate(self) -> float:
        return self.wins / self.n if self.n else 0.0


def _beta_shrink(wins: int, n: int, alpha: float = 2.0, beta: float = 2.0) -> float:
    """Posterior mean of win rate with a Beta(alpha, beta) prior."""
    return (wins + alpha) / (n + alpha + beta) if n else alpha / (alpha + beta)


def _fit_baseline(df: pd.DataFrame) -> tuple[float, float, float]:
    """Fit profit_per_bet ~ a + b*winrate_shrunk using robust regression.

    Returns (a, b, sigma) where sigma is robust std of residuals (per-bet).
    """
    x = df["winrate_shrunk"].to_numpy(dtype=float)
    y = df["profit_per_bet"].to_numpy(dtype=float)

    # Robust fit: Theil-Sen is stable and outlier-resistant.
    res = stats.theilslopes(y, x, 0.90)
    b = float(res[0])
    a = float(res[1])

    yhat = a + b * x
    resid = y - yhat

    # Robust sigma: MAD scaled
    mad = np.median(np.abs(resid - np.median(resid)))
    sigma = float(1.4826 * mad) if mad > 0 else float(np.std(resid) + 1e-9)

    return a, b, sigma


def score_outliers(
    summaries: Iterable[WalletSummary],
    *,
    min_n: int = 50,
    alpha: float = 2.0,
    beta: float = 2.0,
) -> pd.DataFrame:
    """Return a DataFrame with z-scores for wallets whose profit is unusually high
    given their (shrunk) winrate and sample size.

    Model:
      profit_per_bet ~ a + b*winrate_shrunk
      expected_profit = n * expected_profit_per_bet
      z = (profit - expected_profit) / (sigma * sqrt(n))

    This flags "profit too high for winrate" while penalizing small-n.
    """

    rows = []
    for s in summaries:
        if s.n <= 0:
            continue
        rows.append(
            {
                "wallet": s.wallet,
                "n": int(s.n),
                "wins": int(s.wins),
                "winrate": s.winrate,
                "winrate_shrunk": _beta_shrink(s.wins, s.n, alpha, beta),
                "profit": float(s.profit),
                "profit_per_bet": float(s.profit) / float(s.n),
            }
        )

    df = pd.DataFrame(rows)
    if df.empty:
        return df

    # Fit baseline on reasonably-sized samples only (avoid tiny-n distorting baseline)
    fit_df = df[df["n"] >= min_n].copy()
    if len(fit_df) < 10:
        # fallback: fit on everything
        fit_df = df.copy()

    a, b, sigma = _fit_baseline(fit_df)

    df["expected_profit_per_bet"] = a + b * df["winrate_shrunk"]
    df["expected_profit"] = df["n"] * df["expected_profit_per_bet"]
    df["residual"] = df["profit"] - df["expected_profit"]
    df["sigma_profit"] = sigma * np.sqrt(df["n"])  # scale by sqrt(n)
    df["z"] = df["residual"] / df["sigma_profit"].replace(0, np.nan)

    df["a"] = a
    df["b"] = b
    df["sigma_per_bet"] = sigma

    # Rank: only meaningful if enough bets
    df["flag"] = (df["n"] >= min_n) & (df["z"] > 3.5)

    return df.sort_values(["flag", "z"], ascending=[False, False]).reset_index(drop=True)


def polymarket_handle_to_wallet(handle: str, *, timeout: float = 20.0) -> str:
    """Resolve a Polymarket @handle to the proxy wallet address.

    Uses public endpoint: https://polymarket.com/api/profile/userData?address=<wallet>

    We first fetch the profile page and extract the wallet from __NEXT_DATA__.
    """
    url = f"https://polymarket.com/@{handle}"
    with httpx.Client(timeout=timeout, follow_redirects=True, headers={"User-Agent": "Mozilla/5.0"}) as c:
        r = c.get(url)
        r.raise_for_status()
        html = r.text

    # Extract embedded Next.js data
    m = re.search(
        r"<script[^>]*id=\"__NEXT_DATA__\"[^>]*>(.*?)</script>",
        html,
        flags=re.DOTALL,
    )
    if not m:
        raise RuntimeError(f"Could not locate __NEXT_DATA__ in {url}")

    data = json.loads(m.group(1))

    # Heuristic: profile userData queryKey has proxy wallet.
    q = (
        data.get("props", {})
        .get("pageProps", {})
        .get("dehydratedState", {})
        .get("queries", [])
    )
    for qq in q:
        qk = qq.get("queryKey")
        if (
            isinstance(qk, list)
            and len(qk) >= 2
            and qk[0] == "/api/profile/userData"
            and isinstance(qk[1], str)
            and ETH_ADDR_RE.fullmatch(qk[1])
        ):
            return qk[1]

    # Fallback: search for first address that looks like the profile wallet
    addrs = list(dict.fromkeys(ETH_ADDR_RE.findall(m.group(1))))
    if addrs:
        return addrs[0]

    raise RuntimeError(f"Could not resolve wallet for @{handle}")


def load_summaries(path: Path) -> list[WalletSummary]:
    """Load wallet summaries from CSV/JSONL.

    Expected fields per row:
      wallet, n, wins, profit

    - CSV: header required.
    - JSONL: one JSON object per line.
    """
    if not path.exists():
        raise FileNotFoundError(path)

    if path.suffix.lower() == ".csv":
        out: list[WalletSummary] = []
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            for row in reader:
                out.append(
                    WalletSummary(
                        wallet=str(row["wallet"]),
                        n=int(float(row["n"])),
                        wins=int(float(row["wins"])),
                        profit=float(row["profit"]),
                    )
                )
        return out

    if path.suffix.lower() in {".jsonl", ".ndjson"}:
        out = []
        for line in path.read_text(encoding="utf-8").splitlines():
            if not line.strip():
                continue
            obj = json.loads(line)
            out.append(
                WalletSummary(
                    wallet=str(obj["wallet"]),
                    n=int(obj["n"]),
                    wins=int(obj["wins"]),
                    profit=float(obj["profit"]),
                )
            )
        return out

    raise ValueError(f"Unsupported input format: {path}")


def write_df(df: pd.DataFrame, out_path: Path) -> None:
    out_path.parent.mkdir(parents=True, exist_ok=True)
    df.to_csv(out_path, index=False)


def main(argv: list[str] | None = None) -> None:
    p = argparse.ArgumentParser(
        prog="polymarket-outliers",
        description=(
            "Find wallets whose profit is unusually high for their winrate (outlier detection). "
            "Input is a set of wallet summaries (wallet,n,wins,profit)."
        ),
    )

    sub = p.add_subparsers(dest="cmd", required=True)

    r = sub.add_parser("resolve", help="Resolve a Polymarket @handle or profile URL to a wallet address")
    r.add_argument("handle_or_url")

    s = sub.add_parser("score", help="Score wallets and flag outliers")
    s.add_argument("--input", required=True, type=Path, help="CSV or JSONL with wallet,n,wins,profit")
    s.add_argument("--out", required=True, type=Path, help="Output CSV path")
    s.add_argument("--min-n", type=int, default=50)
    s.add_argument("--alpha", type=float, default=2.0)
    s.add_argument("--beta", type=float, default=2.0)
    s.add_argument("--z", type=float, default=3.5, help="Z threshold for flagging")

    args = p.parse_args(argv)

    if args.cmd == "resolve":
        x = args.handle_or_url.strip()
        m = POLYMARKET_PROFILE_RE.match(x)
        handle = m.group("handle") if m else x.lstrip("@").strip()
        wallet = polymarket_handle_to_wallet(handle)
        sys.stdout.write(wallet + "\n")
        return

    if args.cmd == "score":
        summaries = load_summaries(args.input)
        df = score_outliers(summaries, min_n=args.min_n, alpha=args.alpha, beta=args.beta)
        df["flag"] = (df["n"] >= args.min_n) & (df["z"] > float(args.z))
        write_df(df, args.out)
        return


if __name__ == "__main__":
    main()
