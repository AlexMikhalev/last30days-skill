# Compound (review + auto-compound) — OpenCode + Claude Code

This folder contains **forked** scripts based on Ryan Carson’s “compound review + auto-compound” loop, adapted to run with:

- **OpenCode** (`opencode run ...`)
- **Claude Code** (`claude -p "..." --dangerously-skip-permissions ...`)

The intent:

1. **Daily review** (run first): look back over the last 24h of agent work, extract mistakes/patterns, and update **AGENTS.md / docs** so tomorrow’s work is smarter.
2. **Auto-compound** (run second): pull latest main (including fresh learnings), pick the top item from `reports/`, implement it, and open a PR.

## Prereqs

- `opencode` installed and authenticated
- `claude` (Claude Code) installed and authenticated
- `jq`, `gh`

## Skills used

These scripts explicitly reference the **Disciplined Engineering** skills in `../../terraphim-skills/skills/`:

- disciplined-research
- disciplined-design
- disciplined-implementation
- disciplined-quality-evaluation
- code-review

You can add/remove skills depending on the stage.

## Quick start

1) Make scripts executable:

```bash
chmod +x scripts/daily-compound-review-opencode.sh
chmod +x scripts/daily-compound-review-claude.sh
chmod +x scripts/compound/auto-compound-opencode.sh
chmod +x scripts/compound/auto-compound-claude.sh
chmod +x scripts/compound/analyze-report.sh
```

2) Run the review first:

```bash
PROJECT_DIR="$PWD" scripts/daily-compound-review-opencode.sh
# or
PROJECT_DIR="$PWD" scripts/daily-compound-review-claude.sh
```

3) Then run auto-compound:

```bash
PROJECT_DIR="$PWD" scripts/compound/auto-compound-opencode.sh
# or
PROJECT_DIR="$PWD" scripts/compound/auto-compound-claude.sh
```

## Notes

- The “review last 24h threads” step is **tool-specific**.
  - For OpenCode we default to reviewing recent exported sessions or local work artifacts.
  - For Claude Code we default to reviewing recent local artifacts.
  If you have a canonical thread store (Amp threads, OpenCode sessions, etc.), update the prompt to target it.

- `scripts/compound/analyze-report.sh` is intentionally conservative: it produces a small JSON plan (priority + branch name) and leaves the detailed PRD/tasks generation to the agent.
