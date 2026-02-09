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
