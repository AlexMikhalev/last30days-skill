# Debt Dashboard (local)

This folder contains a simple offline-first debt dashboard.

## What you get
- `debts.csv` — your debt list (edit this)
- `dashboard.html` — a dashboard with totals + charts (open in a browser)
- `generate_dashboard.py` — regenerates `dashboard.html` from `debts.csv`

## How to use
1) Open `dashboard.html` in your browser.
2) To update numbers:
   - edit `debts.csv`
   - run:

```bash
python3 generate_dashboard.py
```

## Notes
- Values were extracted from a financial statement PDF dated **27 Jan 2026**.
- Some creditor rows in the PDF are not perfectly structured; verify the numbers in `debts.csv`.
