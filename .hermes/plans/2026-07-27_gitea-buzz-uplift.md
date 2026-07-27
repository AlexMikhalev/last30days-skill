# Gitea Fork Uplift — Buzz-Inspired Features Implementation Plan

> **For Hermes:** Use subagent-driven-development skill to implement this plan task-by-task.

**Goal:** Uplift the `terraphim/gitea` fork (git.terraphim.cloud) with the four Buzz features that beat our current flow: signed agent identity + audit trail, unified event log/search, branch-as-room, and declarative automation triggers — without destabilizing the in-flight upstream sync.

**Architecture:** Additive-only patches to the fork (new routers/services/models, no modification of files touched by upstream-sync issues #43–#51). Anything achievable outside the fork core goes into `gitea-robot` or a new `gitea-automations` daemon instead. Nostr itself is NOT ported — we steal the *semantics* (signed events, one log, rooms) and implement them natively in Gitea.

**Tech Stack:** Go (Gitea fork), gitea-robot (Rust CLI), better-auth-rust (agent identity/SPIFFE), act_runner CI, Postgres.

---

## Current Context / Assumptions

- `terraphim/gitea` fork: 98 custom commits ahead, 460 behind upstream; sync triage in flight (issues #43–#51, cherry-picks B1–B5 + F1–F8). **Rule: no uplift commit touches a file in the upstream-sync blast radius.** New code lives in new files (`routers/api/v1/agent/`, `services/agentaudit/`, etc.) or in separate repos.
- Existing agent surface: `gitea-robot` CLI (pre-authed, PageRank triage, `claim`, `mcp-server`, session wiki) — already our "buzz-cli equivalent".
- Issue #39 ("structured session handover comments") already open on the fork — uplift work must reference, not duplicate.
- Buzz spike running locally at `~/projects/buzz-spike` (relay on `ws://localhost:3000`) — use it as the living reference for feature semantics.
- Buzz feature reference: agents as keypair members with scoped channel membership + per-agent audit; one signed event log covering chat/patches/CI/approvals with NIP-50 search; branch-as-room channels; YAML workflows (message/reaction/schedule/webhook triggers).

## Feature Mapping (Buzz → Gitea uplift)

| Buzz feature | Gitea uplift | Where it lives |
|---|---|---|
| Agents as signed keypair members, own audit trail | **F1: Agent identity + signed action attestations + agent audit API** | fork (additive) + better-auth-rust |
| One signed event log, unified search | **F2: Unified repo event stream API + FTS search** | fork (additive) |
| Branch-as-room | **F3: Auto-room per branch (PR + CI + review in one thread)** | gitea-robot + small fork webhook patch |
| YAML workflows (message/reaction/schedule/webhook) | **F4: Automation rules engine** | new `gitea-automations` daemon (NOT fork core) |
| buzz-cli JSON in/out + ACP harness | Already have: gitea-robot + mcp-server; extend JSON coverage | gitea-robot |

---

## Phase 0 — Safety & scaffolding (do first, blocks everything)

### Task 0.1: Upstream-sync blast-radius guard
**Objective:** Prevent uplift commits colliding with sync cherry-picks.

**Files:**
- Create: `terraphim/gitea` repo file `.terraphim/sync-blast-radius.txt`

**Steps:**
1. From issues #43–#51, list every file touched by cherry-picks B1–B5/F1–F8 (OAuth, mermaid, wiki/LFS tokens, avatar fetch, link-account sync).
2. Write that list into `.terraphim/sync-blast-radius.txt` with a header comment: "uplift PRs must not modify these files until #43 closes".
3. Add a CI check (new workflow `.gitea/workflows/check-blast-radius.yml`) that fails a PR diff touching any listed path while issue #43 is open.
4. Verify: open a throwaway PR touching a listed file → CI fails; touching a new file → CI passes.
5. Commit: `chore: add upstream-sync blast radius guard`

### Task 0.2: Uplift tracking epic
**Steps:**
1. `gitea-robot create-issue --owner terraphim --repo gitea --title "[Epic] Buzz-inspired uplift: agent identity, event stream, rooms, automations" --labels epic`
2. Create child issues F1–F4 with `add-dep` edges: F2,F3,F4 depend on F1 (identity is the substrate).
3. Verify: `gitea-robot triage --owner terraphim --repo gitea` shows F1 unblocked, F2–F4 blocked.

---

## Phase 1 — F1: Agent identity & signed audit trail (the substrate)

**Design:** Gitea already distinguishes users/orgs; we add a first-class `is_agent` flag, per-agent scoped tokens, and an append-only signed audit record for every agent API mutation. **Signing uses NIP-98 HTTP Auth** (secp256k1 Schnorr over a kind:27235 event committing to URL+method+payload hash) — the same scheme buzz-cli uses against the Buzz relay, so one nsec per agent works across both systems and we inherit a published spec + existing libs instead of inventing a bespoke signature format. Keys register per agent with a human `owner_user_id` (Buzz's NIP-OA ownership chain, per spike verdict); key custody via better-auth-rust.

### Task 1.1: `is_agent` user flag
**Files:**
- Modify: `models/user/user.go` (new field only, additive)
- Create: `models/migrations/v1_26/v999.go` (next migration number — check `models/migrations/` for current max first)
- Test: `models/user/user_test.go`

**Steps (TDD):**
1. Failing test: `TestUserIsAgent` — create user with `IsAgent=true`, reload, assert flag persists.
2. Run: `go test ./models/user/ -run TestUserIsAgent -v` → FAIL (unknown field).
3. Add `IsAgent bool `xorm:"NOT NULL DEFAULT false"`` to `User` struct + migration adding the column.
4. Re-run → PASS.
5. Commit: `feat: add is_agent flag to user model`

### Task 1.2: Agent Nostr key registration endpoint
**Files:**
- Create: `routers/api/v1/agent/keys.go` — `POST /api/v1/agent/keys` registers a Nostr pubkey (secp256k1, hex or npub) against the authed agent user (must have `is_agent=true`). Body: `{pubkey, owner_user_id}` — owner must be a human (`is_agent=false`) with admin rights over the agent.
- Create: `services/agentauth/keys.go` — storage + validation.
- Test: `tests/integration/agent_keys_test.go`

**Steps:**
1. Failing integration test: register key as normal user → 403; as agent → 201; duplicate key → 409; owner pointing at another agent → 400.
2. Implement minimal handler + service + `agent_key` table migration (`pubkey TEXT UNIQUE`, `agent_user_id`, `owner_user_id`).
3. Pass tests. Commit: `feat: agent Nostr key registration endpoint`

### Task 1.3: NIP-98 verification + audit log
**Files:**
- Create: `services/agentauth/nip98.go` — verify `Authorization: Nostr <base64(kind:27235 event)>` per NIP-98: Schnorr signature valid, `u` tag matches request URL, `method` tag matches, `payload` tag = SHA-256 of body when present, `created_at` within ±60s.
- Create: `services/agentaudit/audit.go` — append-only `agent_audit_event` table: `{id, agent_user_id, owner_user_id, action, repo_id, event_id, payload_hash, created_unix}`.
- Create: `routers/api/v1/agent/audit.go` — `GET /api/v1/repos/{owner}/{repo}/agent-audit` (admin/agent-owner only).
- Modify: `routers/api/v1/api.go` — route registration only (2 lines).
- Test: `tests/integration/agent_audit_test.go`

**Steps:**
1. Middleware: on any API mutation by an `is_agent` user presenting NIP-98 auth, verify the event against a registered key and write an audit row (event id + payload hash + owner). Unsigned agent mutations still allowed via classic tokens but logged with `event_id=NULL` (rollout-friendly).
2. Failing test: NIP-98-signed issue creation → audit row with valid event id; wrong-URL tag → 401; stale timestamp → 401; unregistered pubkey → 403.
3. Implement (Go: `fiatjaf.com/nostr` or `nbd-wtf/go-nostr` for Schnorr verify — no crypto written by hand).
4. Pass. Commit: `feat: NIP-98 agent auth + signed audit API`

### Task 1.4: gitea-robot NIP-98 signing
**Files:** `gitea-robot` repo — `src/main.rs` (or wherever requests are issued)
**Steps:**
1. Add `GITEA_NOSTR_KEY` env (nsec/hex, same key an agent uses on Buzz): wrap every mutating call with a NIP-98 `Authorization: Nostr <event>` header (Rust: `nostr` crate).
2. Verify end-to-end on staging: signed issue creation → audit row carries the event id and owner.
3. Commit: `feat: NIP-98 sign mutating API calls`

---

## Phase 2 — F2: Unified event stream + search

**Design:** One additive endpoint aggregating existing Gitea primitives (issues, comments, reviews, CI statuses, releases, agent-audit events) into a single reverse-chronological, filterable stream. No new write paths — read-side unification only (Buzz's "one log" semantics over data we already have).

### Task 2.1: Event stream endpoint
**Files:**
- Create: `services/eventstream/stream.go` — unions `Action`, `Comment`, `Review`, `CommitStatus`/`CheckRun`, `agent_audit_event` into `[]Event{kind, actor, ts, ref, payload}`.
- Create: `routers/api/v1/repo/events.go` — `GET /api/v1/repos/{owner}/{repo}/events?kinds=&since=&actor=&cursor=`.
- Test: `tests/integration/repo_events_test.go`

**Steps:**
1. Failing test: seed repo with issue + comment + review + CI status → single call returns all four, ordered, cursor-paginated.
2. Implement service (SQL UNION ALL over subqueries, cursor = `(created_unix, id)`).
3. Pass. Commit: `feat: unified repo event stream API`

### Task 2.2: Cross-event FTS search
**Files:**
- Create: `services/eventstream/search.go` — Postgres FTS over event payloads (respect existing `ENABLE_FTS`/indexer config; fall back to `ILIKE` for other DBs).
- Extend `events.go` with `?q=`.
- Test: `tests/integration/repo_events_search_test.go`

**Steps:**
1. Failing test: `?q=root+cause` matches a comment and an agent-audit payload, not unrelated events.
2. Implement `tsvector` index migration (guarded: Postgres only).
3. Pass. Commit: `feat: FTS across unified event stream`

### Task 2.3: gitea-robot `events` command
**Steps:**
1. Add `gitea-robot events --owner X --repo Y [--kinds ...] [--q ...]` printing JSON lines.
2. Verify against staging; add to `mcp-server` tool list.
3. Commit: `feat: events command + MCP tool`

---

## Phase 3 — F3: Branch-as-room

**Design:** For branches matching a configurable pattern (`feat/*`, `agent/*`), a bot creates/attaches a "room": one issue (or draft PR) that aggregates patches, CI results, agent first-pass review, and the merge decision. 90% automation-side; one small fork patch.

### Task 3.1: `room` command in gitea-robot
**Steps:**
1. `gitea-robot room open --repo X --branch feat/foo` → creates issue titled `Room: feat/foo` with labels `room`, body containing branch metadata + JSON front-matter (aligns with #39 structured handover format); idempotent (finds existing by label+branch tag).
2. `room status` posts CI results as structured comments; `room close` on merge.
3. Tests: mock Gitea API (httptest), assert idempotency + payload shape.
4. Commit: `feat: room open/status/close commands`

### Task 3.2: Branch-push webhook automation
**Files:** new repo `terraphim/gitea-automations` (Rust daemon) — see Phase 4; minimal version here is a webhook handler that calls `gitea-robot room open` on matching branch pushes.
**Steps:**
1. Receive `push` webhook → branch matches pattern → `room open`; CI status webhook → `room status`.
2. Deploy on bigbox alongside gitea-robot; point one repo's webhooks at it as pilot.
3. Verify: push `feat/test-room` → room issue appears within 30s with CI status comment after run.

### Task 3.3 (fork, optional): Room backlink in PR UI
**Files:** `templates/repo/issue/view_content/` — additive template block rendering `room` label issues with branch badge. Only if pilot shows UI value; skip otherwise (YAGNI).

---

## Phase 4 — F4: Automation rules engine (`gitea-automations` daemon)

**Design:** Buzz's YAML workflows (message/reaction/schedule/webhook triggers), scoped to what we actually use: webhook + schedule + reaction triggers, actions = gitea-robot commands. Deliberately a separate daemon — keeps the fork clean and the blast-radius guard trivially satisfied.

### Task 4.1: Daemon skeleton + rule schema
**Files:** new repo `terraphim/gitea-automations`: `src/main.rs`, `src/rules.rs`, `rules/*.yml`
```yaml
# rules/triage.yml
name: auto-triage
on:
  webhook: { events: [issues], action: opened }
  reaction: { emoji: "🐝" }        # any comment reaction = human approval signal
  schedule: { cron: "*/15 * * * *" }
run:
  - gitea-robot triage --owner {{owner}} --repo {{repo}}
  - gitea-robot comment --issue {{issue}} --body-file triage.md
```
**Steps:**
1. Failing test (pytest-style Rust: `cargo test`): parse above YAML → struct; invalid trigger → error.
2. Implement parser (serde_yaml) + dispatcher executing allowlisted commands only (no arbitrary shell).
3. Pass. Commit: `feat: rule schema + parser + dispatcher`

### Task 4.2: Reaction trigger poller
**Steps:**
1. Poll `events` API (Phase 2!) for `reaction` kinds matching configured emoji → fire rule. (This is why F2 lands first.)
2. Test with fixture server. Commit: `feat: reaction trigger via event stream`

### Task 4.3: Deploy + dogfood
**Steps:**
1. Deploy on bigbox as systemd unit `gitea-automations.service`.
2. Dogfood rule: `🐝` reaction on any `terraphim/gitea` issue comment → agent posts triage summary.
3. Verify: react 🐝 on a test issue → comment appears, signed + audited (F1).

---

## Tests / Validation (global)

- Per-task: `go test ./<pkg>/ -run <Test> -v` / `cargo test` as specified.
- Fork regression: `TAGS='sqlite sqlite_unlock_notify' make test-backend` on the fork before each uplift merge (matches existing fork CI).
- End-to-end: staging gitea (bigbox) — signed agent creates issue → appears in event stream → searchable via `?q=` → room opened on branch push → 🐝 reaction triggers automation. All five features exercised in one scripted flow (`tests/e2e/buzz-uplift.sh` in gitea-automations repo).

## Risks, Tradeoffs, Open Questions

- **Upstream sync collision** — mitigated by Task 0.1 blast-radius guard; all fork work additive in new files.
- **Scope creep toward "port Nostr"** — explicitly out. If we ever want real signed-event federation, adopt Buzz's relay as a sidecar, don't reimplement inside Gitea.
- **Event stream performance** — UNION over large tables; mitigate with cursor pagination + per-kind limits; benchmark at 1M actions before enabling FTS by default.
- **Signature UX for agents** — unsigned-agent grace mode (F1.3) keeps current gitea-robot flows working during rollout; flip to enforced per-repo later.
- **Open question:** should `gitea-automations` eventually absorb the PageRank scheduler from gitea-robot, or stay a thin trigger layer? Decide after Phase 4 dogfood.
- **Open question:** Buzz's ACP harness vs our pi/Claude Code flow — spike verdict pending (`~/projects/buzz-spike`); if buzz-agent's bounded-session model proves better, consider ACP support in pi rather than adopting buzz-agent wholesale.

## Spike Verdict (2026-07-27, local relay on :3000)

Exercised buzz-cli end-to-end against a local relay: channel create, signed message send (kind:9), NIP-50 search, NIP-34 diff posting, reactions — all work, all JSON in/out, all signed by the agent's own key. Notable discoveries that change the uplift plan:

- **NIP-AE agent memory** (`buzz mem ls/get/set`): per-agent persistent memory stored relay-side — but **gated on NIP-OA owner attestation** (`BUZZ_AUTH_TAG` / `--owner`). Buzz's model: every agent has a *human owner pubkey*, and memory/write-authority derives from that ownership chain. Our Gitea F1 design should adopt this: `agent_key` table gets an `owner_user_id` column, and agent audit events record the owning human. This is a strictly better accountability model than "robot account with token".
- **Signed diff events** (kind from NIP-34, posted via `send-diff --repo --commit`) map 1:1 to Gitea PR patches — F2's event stream should treat patches as first-class signed events, matching Buzz semantics.
- `users get` self-profile returns empty until profile is published — agent onboarding in F1 must include a profile-publish step or agents are invisible in search/membership.
- Relay stood up cleanly via `just setup && just relay` (Hermit pinned toolchain, Docker services auto-started, migrations + community seed automatic). First full build ≈ 25 min on 4 cores; incremental is seconds.
