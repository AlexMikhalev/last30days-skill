# polymarket-outliers

Detect wallets whose **profit** is unusually high given their **win rate** and sample size.

This repo provides:
- `resolve`: turn a Polymarket profile `@handle` into a proxy wallet address (scrapes the profile page).
- `score`: outlier scoring given a dataset of wallet summaries.

> Note: Polymarket does not expose win/loss counts directly on the public profile page.
> You must supply `wins`/`n`/`profit` from your own trade parsing (fills/positions), a subgraph,
> or another data source.

## Install

```bash
cd polymarket_outliers
uv sync
```

## Resolve a handle → wallet

```bash
uv run polymarket-outliers resolve https://polymarket.com/@CRYINGLITTLEBABY
# or
uv run polymarket-outliers resolve CRYINGLITTLEBABY
```

## Score outliers

Create an input CSV like:

```csv
wallet,n,wins,profit
0xabc...,120,71,10234.50
0xdef...,55,28,3200.10
```

Then:

```bash
uv run polymarket-outliers score --input wallets.csv --out outliers.csv --min-n 50 --z 3.5
```

The output includes:
- `winrate_shrunk` (Beta prior smoothing)
- a robust baseline fit (`a`, `b`, `sigma_per_bet`)
- residual and z-score
- `flag` boolean
