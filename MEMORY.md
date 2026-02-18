# MEMORY (Long-term)

## 2026-01-24
- WhatsApp gateway “flapping” mostly traced to an **idle watchdog reconnect** ("no inbound in 30m" → reconnect → status=499). Mentions of status=408 were from message text, not confirmed WA disconnects.
- Tightened Clawdbot credentials directory permissions: `chmod 700 /home/alex/.clawdbot/credentials` (cleared security audit critical).
- Enabled **browser control** in Clawdbot config (`browser.enabled: true`) and verified it’s enabled in security audit.
- X/Twitter automation:
  - `bird` is installed and works for reading/posting once authenticated.
  - On Linux, cookie extraction may fail if Chrome is launched with `--password-store=basic`; manual `AUTH_TOKEN`/`CT0` works.
- Google Workspace automation:
  - `gog` is installed (v0.9.0). Default keyring integration can fail; switched to `gog auth keyring file` + `GOG_KEYRING_PASSWORD`.
  - Re-authenticated `alex@zestic.ai` for Gmail + Calendar via `gog auth add ... --manual`.


## 2026-02-09
- **openclaw message send** CLI changed: use `--target` (or `-t`) instead of `--to` or `channelId`. The old flags are rejected.
  - Correct: `openclaw message send --target '+447842912714' --message 'Hello'`
  - Correct: `openclaw message send -t '+447842912714' --media /path/to/file.epub`
  - Wrong: `openclaw message send --to '+447842912714' --message 'Hello'`
  - For replies: add `--reply-to <messageId>`
  - For media: add `--media <path-or-url>` with optional `--message` for caption

## 2026-02-11
- Added skill/reference repo to local workspace: `/home/alex/clawd/cto-executive-system` (GitHub: AlexMikhalev/cto-executive-system). Treat as reference material in future sessions.

## 2026-02-18
- Updated OpenClaw: **2026.2.6-3 → 2026.2.17** (bun global update; gateway restarted).
- Terraphim integration status:
  - `terraphim-skills` hooks installed and wired for Claude (`~/.claude/hooks/*`, `~/.claude/settings.local.json`).
  - `terraphim-agent` installed as **musl** binary due to system glibc=2.31; version **1.4.10**.
  - To add custom roles to `terraphim-agent roles list`, the agent must load a persisted **Embedded** config named `embedded_config.json` (otherwise it falls back to the hardcoded 3-role embedded defaults, including broken `docs/src/kg`). Persistence location is controlled by device settings / `TERRAPHIM_SETTINGS_PATH`.

## Silent Replies
When you have nothing to say, respond with ONLY: NO_REPLY
⚠️ Rules:
- It must be your ENTIRE message — nothing else
- Never append it to an actual response (never include "NO_REPLY" in real replies)
- Never wrap it in markdown or code blocks
❌ Wrong: "Here's help... NO_REPLY"
❌ Wrong: "NO_REPLY"
✅ Right: NO_REPLY
