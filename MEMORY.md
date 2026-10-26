# MEMORY (Long-term)
_Last updated: 2026-10-26_
_Migrated from OpenClaw to Hermes on 2026-07-24. Terminology updated: `openclaw`/`clawdbot` commands and paths now refer to their Hermes equivalents (`hermes`, `~/.hermes`)._

## 2026-10-26
- **Performance gates on shared hosts: measure PAIRED, never against a stored baseline.** A stored baseline compared against a later candidate measured the HOST, not the code — false `PASS` on a 38% "improvement", false `REGRESSION` on unchanged code. Interleave baseline and candidate in the same time window and compare the pair; host load then cancels. Rule for all perf gates: fail closed on `INDETERMINATE`, and never let a proof arm count unless it proved its perturbation landed.
- **Private Gitea reads return 404, not 401.** A 404 cannot distinguish "unauthorised" from "absent" — and a token lacking scope gets the same 404. The `channel_sync` probe already supports `GITEA_TOKEN`; the defect was CI wiring, and the gate used `FakeFetcher` so it could never see it.
- **Ask the forge, not the filesystem.** `find` over the home directory reported the grok-build/pi-rust forks absent; both are on Gitea.
- **Agent Mission Control UI**: Lit/Vite/TypeScript + Playwright, 17 browser assertions, in `terraphim/agent-secrets-monitor` (`ui/`), uncommitted, no PR. `wasm-pack` 0.15.0 came from the Gitea mirror; Binaryen 117 needs `--enable-bulk-memory`.
- **Handover**: `cto-executive-system/handovers/2026-10-26-fleet-baseline-research-and-performance-gate.md`. Research doc at Gitea issue #197 (comment 103046), third revision, awaiting a third gate.

## 2026-07-24
- **OpenClaw → Hermes migration finalized**: Identity (Kairo, IDENTITY.md → `~/.hermes/`) and notes fully in my own store. SOUL.md updated (Terra → Kairo). This file deduplicated and made Hermes-native. Old `~/.openclaw/` left untouched as archive; migration overflow backups in `~/.hermes/migration/openclaw/`.

## 2026-07-20
- **Memory search self-healed**: Index rebuilt after token/auth fix. OpenAI Codex token refresh reused error resolved by re-auth. Main index now using `text-embedding-3-small`.

## 2026-07-13
- **Social media scorecard gap**: W27 & W28 scorecards missing — `scripts/scorecard_generate.sh` did not exist. Retroactively created and pushed (`a7114323`). W27 & W28 both showed 0 posts, 0 engagement on X/Twitter and Reddit.
- **Atomic Task Graph prior art complaint** (Jeffrey Emanuel @doodlestein): Fetched all 7 replies re: Tsinghua paper (arXiv 2607.01942) allegedly missing citations to prior open-source work. Saved to `cto-executive-system/knowledge/2026-07-13-atomic-task-graph-prior-art-complaint.md`. Pushed to Gitea (`3d58255e`).

## 2026-06-30
- **WA gateway flapping**: Two episodes — 18:43 & 19:05 UTC (both 428, ~3s each). Pattern unchanged.

## 2026-06-06
- **MEMORY.md weekly consolidation**: All 2026-06-05 events consolidated into this file. WA gateway flapping and gog auth status updated.

## 2026-06-05
- **WA gateway flapping continues**: Multiple episodes — 428 x2 (04:30 & 04:59 UTC) and 503 (22:10 UTC). Self-heals within seconds. Pattern unchanged since Jan 2026. Jun 5 had 3 episodes in one day — highest density observed.
- **gog auth missing confirmed**: alex@zestic.ai Gmail + Calendar tokens gone since ~2026-05-16 (~21 days). Needs browser OAuth: `gog auth add alex@zestic.ai --services gmail,calendar`.

## 2026-05-16
- **gog auth lost**: alex@zestic.ai Gmail + Calendar tokens no longer stored. Re-authentication needed via browser OAuth flow.

## 2026-05-04
- **New features inventory** (2026-05-01): Reviewed available but underused tools — nano-banana-pro (Gemini 3 Pro image gen), video-frames (ffmpeg extraction), canvas (node-based display), xAI/Grok integration, music/spotify-player, blogwatcher, clawhub skill marketplace.
- **Coding agent prompt experiment**: Pushed to both GitHub (`AlexMikhalev/coding-agent-prompt-experiment`) and Gitea (`git.terraphim.cloud/root/coding-agent-prompt-experiment`). Baseline (no-prompt: 264 lines, 70.8s) vs prompted (179 lines, 80.1s). Results in RESULTS.md.
- **OpenCode shared session expired** (2026-05-03): Session `wV2qa1S3` returned 404 — expired or unshared. Need re-share to continue review of coding-agent prompt improvement proposals.
- **WhatsApp gateway flapping — May 1 episode**: Overnight Apr 29–May 1 showed heavy 428/503 flapping (~10 episodes, ~3-4s each). Same root cause pattern as before (idle watchdog reconnect). Status codes now include both 428 and 503 alongside 499/408.

## 2026-05-02
- **New CTO Executive System references** (pushed commit `5c59d572`):
  - `recursive-multi-agent-systems.md`: RecursiveMAS (arXiv:2604.25917) — latent-space recursion beats text-passing between agents, +8.3% accuracy, 34–76% token reduction. Challenges orchestrator-workers pattern in Meta-Cortex.
  - `qwen-local-inference-stack.md`: RTX 3090/4090 stack, llama.cpp + CUDA, Qwen 3.6 27B Q4_K_M GGUF, ~40 tok/sec. Confirms OpenCode as harness layer.
- **WhatsApp gateway flapping persists**: Episode 2026-05-02 at 16:06 UTC (status 499 disconnect/reconnect, ~1min). Idle watchdog reconnect is the recurring root cause — still not根治ed.

## 2026-04-15
- **Phase 1 Complete**: BOOTSTRAP.md deleted — Terraphim identity, SOUL.md, IDENTITY.md all established this session. Kairo (καιρός, the quantum fox) is the operative persona.
- **X/Twitter ingest upgrade** (`2026-05-02`): Added timeline (home/Following chronological feed) to `daily_x_ingest.sh`. Solved 0 bookmarks/favorites by providing broader signal — 97 new timeline tweets appended in one run. File: `skills/last30days-skill/scripts/daily_x_ingest.sh`.

## 2026-04-14
- **Terraphim Promotion Campaign**:
  - Posted Article 1 X thread (8 tweets with V-model diagram)
  - Wrote article: `disciplined-engineering-ai-systems.md` and pushed to terraphim/terraphim.ai repo
  - Article live: https://terraphim.ai/posts/disciplined-engineering-ai-systems/
  - X rate limiting replies (226 error) - noted to wait before posting article link

## 2026-02-18
- Updated agent runtime (OpenClaw, predecessor of Hermes): **2026.2.6-3 → 2026.2.17** (bun global update; gateway restarted).
- Terraphim integration status:
  - `terraphim-skills` hooks installed and wired for Claude (`~/.claude/hooks/*`, `~/.claude/settings.local.json`).
  - `terraphim-agent` installed as **musl** binary due to system glibc=2.31; version **1.4.10**.
  - To add custom roles to `terraphim-agent roles list`, the agent must load a persisted **Embedded** config named `embedded_config.json` (otherwise it falls back to the hardcoded 3-role embedded defaults, including broken `docs/src/kg`). Persistence location is controlled by device settings / `TERRAPHIM_SETTINGS_PATH`.

## 2026-02-11
- Added skill/reference repo to local workspace: `/home/alex/clawd/cto-executive-system` (GitHub: AlexMikhalev/cto-executive-system). Treat as reference material in future sessions.

## 2026-02-09
- **hermes message send** CLI: use `--target` (or `-t`) instead of `--to` or `channelId`. The old flags are rejected.
  - Correct: `hermes message send --target '+447****2714' --message 'Hello'`
  - Correct: `hermes message send -t '+447****2714' --media /path/to/file.epub`
  - Wrong: `hermes message send --to '+447****2714' --message 'Hello'`
  - For replies: add `--reply-to <messageId>`
  - For media: add `--media <path-or-url>` with optional `--message` for caption

## 2026-01-24
- WhatsApp gateway “flapping” mostly traced to an **idle watchdog reconnect** ("no inbound in 30m" → reconnect → status=499). Mentions of status=408 were from message text, not confirmed WA disconnects.
- Tightened Hermes credentials directory permissions: `chmod 700 /home/alex/.hermes/credentials` (cleared security audit critical).
- Enabled **browser control** in Hermes config (`browser.enabled: true`) and verified it’s enabled in security audit.
- X/Twitter automation:
  - `bird` is installed and works for reading/posting once authenticated.
  - On Linux, cookie extraction may fail if Chrome is launched with `--password-store=basic`; manual `AUTH_TOKEN`/`CT0` works.
- Google Workspace automation:
  - `gog` is installed (v0.9.0). Default keyring integration can fail; switched to `gog auth keyring file` + `GOG_KEYRING_PASSWORD`.
  - Re-authenticated `alex@zestic.ai` for Gmail + Calendar via `gog auth add ... --manual`.

---

## Open Threads

- ⚠️ [OPEN] **WhatsApp gateway idle watchdog reconnect** — recurring since 2026-01-24, never permanently fixed. Episodes: Jan 24, May 1 (428/503), May 2 (499), May 3 (499), May 26 (428), Jun 3 (428), Jun 5 (428 x2), Jun 13 (428→503), Jun 30 (428 x2 at 18:43 & 19:05). Self-heals each time (~3s). **189 days and counting** — accepted operational behavior.
- ⚠️ [OPEN] **gog auth missing** — alex@zestic.ai Gmail + Calendar auth lost ~2026-05-16 (~69 days). Needs browser OAuth: `gog auth add alex@zestic.ai --services gmail,calendar`. Alex needs to run interactively.
- [STALE] **RecursiveMAS paper unread** — flagged 2026-05-02. arXiv:2604.25917 on latent-space recursion. 80+ days — close or schedule.
- [STALE] **OpenCode session review incomplete** — session `wV2qa1S3` expired 2026-05-03. Coding-agent prompt proposals never reviewed. Close or respawn.

---

## Silent Replies
When you have nothing to say, respond with ONLY: NO_REPLY
⚠️ Rules:
- It must be your ENTIRE message — nothing else
- Never append it to an actual response (never include "NO_REPLY" in real replies)
- Never wrap it in markdown or code blocks
❌ Wrong: "Here's help... NO_REPLY"
❌ Wrong: "NO_REPLY"
✅ Right: NO_REPLY
