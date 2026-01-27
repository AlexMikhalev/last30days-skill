# CTO-OS (Markdown)

A lightweight, markdown-first operating system for running:
- **You** (executive decisions)
- **Clawdbot** (your personal EA + CTO partner)
- **Claude Code** (implementation / deep technical work)

This repo is designed to be run *daily* and *weekly* with short prompts + structured notes.

## How to use (2 minutes)

### Daily (AM, 10–15 min)
1. Open `inbox/` and drop anything on your mind.
2. Fill today’s file: `daily/YYYY-MM-DD.md` (template below).
3. Ask Clawdbot to run **Daily CTO Sweep** (see prompts in `prompts/`).

### Weekly (45–60 min)
1. Fill `weekly/YYYY-WW.md`.
2. Run **Weekly CTO Review**.
3. Produce a single `decisions/` entry + update `roadmap/`.

### When you need Claude Code
Use `prompts/claude-code/` to generate a task packet:
- problem statement
- constraints
- repo links
- acceptance criteria
- files to read

## Folder map
- `inbox/` — raw dumps. Nothing stays here.
- `daily/` — daily CTO log + plan.
- `weekly/` — weekly review.
- `decisions/` — ADR-style decisions (date-stamped).
- `roadmap/` — quarterly + monthly planning.
- `scorecards/` — metrics and KPIs.
- `people/` — hiring, 1:1s, org design, calibration.
- `tech/` — architecture, reliability, security, platform, devex.
- `product/` — strategy, positioning, user research, PRDs.
- `ops/` — finance, vendors, legal, compliance, runbooks.
- `projects/` — project pages (status, risks, next actions).
- `prompts/` — reusable prompts for Clawdbot + Claude Code.
- `templates/` — canonical templates.

## Rules of the system
1. **Everything has a home** (inbox → processed).
2. **Decisions are written** (not just implied).
3. **Projects have one page** (status, risks, next action).
4. **Metrics are explicit** (scorecards beat vibes).
5. **One weekly narrative**: what changed, why, and what’s next.
