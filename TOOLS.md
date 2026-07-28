# TOOLS.md - Local Notes

Skills define *how* tools work. This file is for *your* specifics — the stuff that's unique to your setup.

## What Goes Here

Things like:
- Camera names and locations
- SSH hosts and aliases  
- Preferred voices for TTS
- Speaker/room names
- Device nicknames
- Anything environment-specific

## Alex's Executive System

**Location:** `~/clawd/cto-executive-system/`

Key components I should know:
- **Daily sweep:** `rhythms/daily-sweep.md` — 5-10 min daily planning
- **Weekly review:** `rhythms/weekly-review.md` — 30-60 min weekly
- **Philosophy:** `philosophy/essentialism.md` + `flawless-execution.md`
- **Scorecards:** `scorecards/engineering.md`, `scorecards/ai-research.md`
- **Prompts:** `prompts/` — Claude Code prompts for running rhythms

Guardrails (hard limits):
- Max 3 outcomes per day
- Max 5 active projects
- Min 10h deep-work per week
- Min 1 marketing act per week

## Examples

```markdown
### Cameras
- living-room → Main area, 180° wide angle
- front-door → Entrance, motion-triggered

### SSH
- home-server → 192.168.1.100, user: admin

### TTS
- Preferred voice: "Nova" (warm, slightly British)
- Default speaker: Kitchen HomePod
```

## Why Separate?

Skills are shared. Your setup is yours. Keeping them apart means you can update skills without losing your notes, and share skills without leaking your infrastructure.

---

Add whatever helps you do your job. This is your cheat sheet.

## Conway (Autonomous AI Wallet)
- **API Key:** `cnwy_k_fJ7-m72Dm3t23VSPn8u_lkOSC9aAZMVN`
- **Wallet:** `0x5B962e41D4D1e6974BDFB8B61090cdAF935ec7Bf`
- **Installed:** via `npx conway-terminal`

## Gitea (Kairo Identity)
- **Instance:** https://git.terraphim.cloud
- **User:** kairo
- **Access:** HTTPS only (no SSH configured)
- **Token:** stored in `~/.clawdbot/credentials/gitea-kairo.env` (env: `GITEA_TOKEN`)
- **Auth:** `https://kairo:<TOKEN>@git.terraphim.cloud` for git operations
- **Scopes:** write:repository, read:user, read:issue (repo creation requires site admin)
- **CLI:** `~/.local/bin/gitea-robot` (set `GITEA_URL` + `GITEA_TOKEN` env vars)
