# ZeroClaw (theonlyhennygod/zeroclaw) vs TinyClaw (terraphim-ai PR #529)

Date: 2026-02-14

Scope
- Compare the `zeroclaw/src` implementation (Rust) against the latest TinyClaw implementation as represented by Terraphim PR #529: https://github.com/terraphim/terraphim-ai/pull/529
- Output a concise, decision-oriented summary.
- Inputs accessed:
  - ZeroClaw: `src/main.rs` and `src/` tree listing
  - TinyClaw: PR description + file list/patch excerpts from GitHub API (diff fetch was flaky)

## 1) Executive summary

TinyClaw (PR #529) is a *real execution* implementation focused on: (a) actually running shell/tool/LLM steps, (b) wiring Telegram/Discord adapters to a message bus with allowlists, (c) fixing a gateway outbound dispatch bug, and (d) adding conversation compression via Ollama.

ZeroClaw, by contrast (from `src/main.rs`), looks like a *CLI-first product shell* with a broad command surface (onboard/agent/gateway/daemon/service/status/cron/channel/integrations/skills/migrate) that implies similar ambitions, but we can’t yet confirm parity in the “real execution” details without reading deeper modules (`src/agent`, `src/tools`, `src/channels`, etc.).

If you want ZeroClaw to match TinyClaw’s “functional agent with real execution” posture, the key design targets are:
- A step executor with explicit, testable semantics for shell/tool/LLM steps, including timeouts and graceful degradation.
- Channel adapters as *thin IO* that forward into a central bus (with allowlists), rather than embedding agent logic.
- Gateway mode that guarantees outbound dispatch (spawned loop reading outbound queue).
- Compression/summarization as a first-class pipeline stage with a clear fallback chain.

## 2) What TinyClaw PR #529 actually changes (ground truth from PR body)

PR title: `feat(tinyclaw): implement functional TinyClaw agent with real execution`

Key components called out:

### 2.1 SkillExecutor (`executor.rs`)
- Shell steps:
  - Uses `tokio::process::Command`
  - 120s timeout
  - working directory support
- Tool steps:
  - Integrates `ToolRegistry` behind `Option<Arc<ToolRegistry>>`
  - “graceful fallback when unconfigured”
- LLM steps:
  - Calls Ollama HTTP API `/api/generate`
  - “graceful degradation when unavailable”

### 2.2 Channel adapters
- Telegram:
  - `teloxide::Dispatcher`
  - allowlist by user id/username
  - forwards into message bus
- Discord:
  - `serenity::Client` + `EventHandler`
  - bot message filtering
  - allowlist
  - bus integration

### 2.3 Agent loop (`agent_loop.rs`)
- `compress()`:
  - real Ollama API call for summarization
  - extractive fallback
- `text_only()`:
  - tries proxy → direct Ollama → static fallback
- wires `keep_last_messages` into compression logic

### 2.4 Bug fix
- Gateway outbound dispatch:
  - adds missing `tokio::spawn` loop to read `outbound_rx` and dispatch to channels
  - previously gateway mode “silently dropped all agent responses”

### 2.5 Cleanup
- Removes unused deps: `terraphim_multi_agent`, `terraphim_config`, `terraphim_automata`
- Replaces blanket `#![allow(dead_code)]` with targeted annotations

## 3) What ZeroClaw implements (ground truth from `src/main.rs`)

### 3.1 Product surface / commands
ZeroClaw exposes a large CLI surface:
- `Onboard` (quick setup or interactive wizard, channels-only repair)
  - flags: `--interactive`, `--channels-only`, `--api-key`, `--provider`, `--memory`
- `Agent` (agent loop)
  - flags: `--message` single-shot, `--provider`, `--model`, `--temperature`
- `Gateway` (server: webhooks, websockets)
  - flags: `--port`, `--host`
- `Daemon` (gateway + channels + heartbeat + scheduler)
  - flags: `--port`, `--host`
- `Service` (install/start/stop/status/uninstall)
- `Doctor`
- `Status`
- `Cron` (list/add/remove)
- `Channel` (list/start/doctor/add/remove)
- `Integrations` (info)
- `Skills` (list/install/remove)
- `Migrate` (imports from OpenClaw workspace)

This is structurally similar to TinyClaw’s “gateway + channels + agent loop” framing.

### 3.2 Architecture signals
From module declarations, ZeroClaw has these subsystems:
- `agent`, `channels`, `gateway`, `daemon`, `heartbeat`, `cron`, `doctor`, `health`
- `providers` (implies multi-provider LLM: openrouter/anthropic/openai)
- `skills`, `tools`, `integrations`
- `memory`, `migration`
- `runtime`, `service`, `security`, `observability`, `tunnel`

Notable design difference vs TinyClaw PR:
- TinyClaw PR is explicitly *Ollama-first* for execution + compression.
- ZeroClaw CLI explicitly supports `openrouter, anthropic, openai` as providers.

## 4) Gap analysis (ZeroClaw vs TinyClaw PR #529)

### 4.1 Execution engine semantics
TinyClaw:
- Clearly defined step types: shell/tool/LLM
- Enforced timeout (120s) for shell
- ToolRegistry optional wiring
- LLM is an HTTP client to Ollama with fallback

ZeroClaw (from `main.rs` only):
- Agent exists but step semantics are not visible yet.

Design recommendation (align with TinyClaw):
- Make a single `Executor` contract that:
  - takes a structured plan (steps)
  - emits structured results (stdout/stderr/exit code/tool result/token usage)
  - enforces timeouts and working dir
  - has explicit fallback behavior (provider/tool missing)

### 4.2 Channel adapters and message bus
TinyClaw:
- “thin adapters” forwarding to a message bus
- explicit allowlists

ZeroClaw:
- Has `channels` module and channel commands (telegram/discord/slack)
- allowlisting not confirmed

Design recommendation:
- Centralize the “agent brain” behind a bus/queue API; channels should only:
  - authenticate
  - filter (allowlist, bot-self messages)
  - convert message ↔ internal event

### 4.3 Gateway outbound dispatch reliability
TinyClaw:
- Fixes a real bug: missing spawned loop for outbound dispatch

ZeroClaw:
- Has a `Gateway` and `Daemon` command; unknown if outbound dispatch is robust.

Design recommendation:
- Treat outbound dispatch as a non-optional runtime component with:
  - backpressure strategy
  - reconnect logic
  - metrics/health endpoints

### 4.4 Compression / conversation management
TinyClaw:
- `compress()` uses Ollama API + fallback
- uses `keep_last_messages` config

ZeroClaw:
- Has `memory` module and a `memory backend` option (sqlite/markdown/none)
- compression not confirmed

Design recommendation:
- Implement compression as a first-class policy:
  - when to compress (token budget / message count)
  - what to keep (system + last N + pinned)
  - fallback (extractive summary)

### 4.5 Provider strategy
TinyClaw:
- Ollama-centric and degrades when unavailable

ZeroClaw:
- Multi-provider oriented (openrouter/anthropic/openai) from CLI flags

Design recommendation:
- Keep ZeroClaw’s provider flexibility, but implement the *same* degradation chain concept:
  - preferred provider → secondary provider → static fallback response

## 5) Practical action list to bring ZeroClaw closer to “TinyClaw PR #529 posture”

1) Document and implement the executor contract
- Step types: shell/tool/llm
- Timeout + cwd rules
- Deterministic error mapping

2) Add allowlists in all channel adapters
- By user id and optionally username
- Default-deny stance in daemon mode

3) Add an explicit outbound dispatcher task in gateway/daemon
- Spawn a loop that drains outbound queue and sends to channels
- Add tests to prevent regression (“responses not delivered”)

4) Implement compression policy
- Config: `keep_last_messages`
- Summarizer provider chain

5) Add a “doctor” that validates runtime wiring
- Verify outbound dispatch is running
- Verify channels connected
- Verify provider reachable (or shows degraded mode)

## 6) Notes / limitations
- I have not yet read ZeroClaw’s deeper modules (agent/channels/tools/skills/etc.). This summary is grounded in:
  - ZeroClaw `src/main.rs`
  - TinyClaw PR #529 description + file list and patch excerpts available via GitHub API
- If you want a true line-by-line comparison, next step is to fetch and parse:
  - ZeroClaw: `src/agent/*`, `src/tools/*`, `src/channels/*`, `src/gateway/*`, `src/daemon/*`
  - TinyClaw: the exact diff files for PR #529 (or checkout the branch `codex/pr492-merge-main`)
