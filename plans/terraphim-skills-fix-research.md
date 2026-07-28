# Research Document: terraphim-skills Catalogue & Platform Consistency Fix

**Status**: Draft
**Author**: Kairo (Terraphim agent)
**Date**: 2026-07-25
**Reviewers**: Alex

## Executive Summary

The terraphim-skills ecosystem has drifted into **three divergent catalogues**: the git repo (45 skills), the live MCP backend (45 skills, but a *different* 45), and the public website (42 skills, static). The website's dynamic API (`api.terraphim-skills.md`) returns 404 on every probed route, breaking sign-in and any live catalogue features — while the MCP endpoint on the same infrastructure is fully healthy. The repo additionally carries 1,473 tracked `node_modules` files and 2 tracked `.skill` zip bundles, and the issue tracker contains duplicates and stalled marketplace-epic issues.

## Essential Questions Check

| Question | Answer | Evidence |
|----------|--------|----------|
| Energizing? | Yes | Marketplace is the public face of the Terraphim skills effort; it's currently misrepresenting the product |
| Leverages strengths? | Yes | We own repo, backend (MCP tools respond), and DNS/Cloudflare — no external blocker |
| Meets real need? | Yes | Users hitting "Sign in" / `tsm install` flow hit a dead API; site claims 42 skills when 45+ exist |

**Proceed**: Yes — 3/3 YES

## Problem Statement

### Description
Three systems that should present one catalogue disagree with each other, and one public-facing service (REST API) is down while its sibling (MCP) is up.

### Impact
- **Visitors**: see stale catalogue (42), broken sign-in buttons (point at dead `api.` domain)
- **CLI/MCP users**: can install 45 skills including `handover`/`session-start` that don't exist in the repo — no source of truth, no way to audit or contribute fixes to them
- **Contributors**: newest repo skills (`terraphim-linear`, `linear-issue-to-pr`, `infrastructure/*`) are invisible/uninstallable via the marketplace
- **Maintainers**: 1,473 tracked node_modules files make every clone/diff/review noisy; duplicate issues split discussion

### Success Criteria
1. Repo `main` is the single source of truth; every published skill exists in repo and vice versa (or divergence is explicitly documented)
2. `https://api.terraphim-skills.md/skills` (the exact URL the website frontend calls) returns 200 with the catalogue
3. Website counter and listing match the backend
4. `node_modules` and `*.skill` artefacts untracked (PR #29 merged or equivalent)
5. Issue tracker: no duplicates; each open issue has a current owner/status

## Current State Analysis

### Three-way catalogue comparison (verified 2026-07-25)

| Source | Count | Notable |
|--------|-------|---------|
| Repo `terraphim/terraphim-skills` main | 45 skill dirs | Has `terraphim-linear`, `infrastructure/{1password-secrets,caddy,xero}`; lacks `handover`, `session-start` |
| MCP backend (`mcp.terraphim-skills.md/mcp`) | 45 | Has `handover`, `session-start`; lacks `terraphim-linear`, `infrastructure` |
| Website `terraphim-skills.md/skills` | 42 | Static render; missing `linear-issue-to-pr`, `handover`, `session-start`; homepage badge says "42" |

### Component map

| Component | Location | Status |
|-----------|----------|--------|
| Skills repo | `git.terraphim.cloud/terraphim/terraphim-skills` (local: `~/projects/terraphim-skills`, in sync w/ origin) | Healthy |
| MCP server | `mcp.terraphim-skills.md/mcp` — `rmcp 1.8.0`, 11 tools (list/search/show/install/publish/verify/use_skill, whoami, Stripe checkout ×2) | **Healthy** |
| REST API | `api.terraphim-skills.md/*` — all probed routes 404 (`/skills`, `/health`, `/v1/skills`, `/api/skills`, …) | **Down/misrouted** |
| Website | `terraphim-skills.md` (Cloudflare-fronted, static content renders) | Degraded (dynamic calls fail) |
| Auth | OAuth buttons → `https://api.terraphim-skills.md/auth/oauth/{github,google}` | Broken (dead domain routes) |

### ~~Key technical facts~~ (CORRECTED 2026-07-25 after locating server source)
- **The REST API was never down.** Probed wrong paths in initial research. Live verification: `GET /api/catalogue` → **200** (45 skills, identical set to MCP), `/api/pricing` → 200, `/auth/oauth/github` → **302** (OAuth works). Source: `terraphim/terraphim-skills-server` (Rust/axum), routes in `src/main.rs`.
- **The actual bug**: the website frontend fetches `https://api.terraphim-skills.md/skills` (404 — no such route) instead of the real endpoint `/api/catalogue`. One-line mismatch.
- The website is a **static site served by nginx from `deploy/static-site/` on bigbox — NOT tracked in git** (not in terraphim-skills-server repo). This is why the bug can't be PR'd and why the "42" counter never updates.
- Infra: cloudflared tunnel splits `api.` → Rust server:8080, `mcp.` → tsm-mcp-server:3000, apex → nginx static:80. Deploy host: bigbox, docker-compose in `deploy/`.
- **New finding — tier skew**: catalogue is 44 premium / 1 community (`acceptance-testing`). The zero-setup raw endpoint `/skills/{name}/SKILL.md` serves community tier only → **402 on 44/45 skills**. This is the root cause PR #26's "402 detection" was chasing.
- DNS: same Cloudflare IPs for apex + api. (tunnel-level split, not host-level)
- Repo hygiene: **FIXED** — PR #29 merged; `node_modules` tracked = 0, `.skill` tracked = 0 (verified after pull)
- 19 remote branches, most stale task branches for already-merged work (#10, #23, #28, #31)

### Issue tracker state (open)
- **Duplicates**: #30 ≡ #31 ("add linear-ticket-writer skill"); PR #32 refs #31
- **Stalled fixes**: PR #26 (installer default source + 402 detection, Jul 13), PR #29 (stop tracking artefacts, Jul 14), PR #32 (Jul 16)
- **Marketplace epic**: #16–#21 open since May 23 — Phase 1 (static showcase) shipped; Phase 2 (CLI integration) partially shipped (MCP works); Phase 3 (quality gates/monetisation) partially present (Stripe checkout tools exist)
- Content bugs: #12 (MCP precheck), #13 (kg-rlm-ingest idempotency), #14 (adf-orchestrate confirm)

## Constraints

### Technical Constraints
- Single source of truth must remain a git repo (auditability, signatures, BLAKE3 hashing of skill content)
- Published skills are content-addressed/signed — republishing must not break existing installs (hash continuity for unchanged content)
- Cloudflare routing changes require zone access (Alex)

### Business Constraints
- Premium/monetised skills: catalogue changes must not break Stripe checkout flow or entitlements
- No budget/timeline pressure stated; assume low-risk incremental rollout

## Vital Few (Essentialism)

### Essential Constraints (Max 3)

| Constraint | Why It's Vital | Evidence |
|------------|----------------|----------|
| Repo main = single source of truth | Without it, drift recurs within weeks | Three-way divergence observed today |
| Don't break existing installs/entitlements | Paying users; signed content | `verify_skill`, checkout tools in MCP |
| Fix routing, not rewrite | MCP proves backend healthy; cheapest correct fix | Same-IP DNS, working MCP handshake |

### Eliminated from Scope

| Eliminated Item | Why Eliminated |
|-----------------|----------------|
| Redesigning the website UX/branding | Not broken; content drift is the bug |
| Adding new skills (linear-ticket-writer etc.) | Separate content work; tracked in #31/#32 |
| Migrating hosting off Cloudflare | No evidence hosting is the problem |
| Monetisation redesign (Phase 3 epic) | Checkout tools already exist; out of scope for consistency fix |
| Content bugs #12/#13/#14 | Real but independent; keep in tracker |

## Dependencies

### Internal Dependencies
| Dependency | Impact | Risk |
|------------|--------|------|
| `terraphim-skills-server` source (website + worker) | Needed to fix routing/publish pipeline | **Not visible to kairo's Gitea token** — repo may be private to root |
| Cloudflare zone for `terraphim-skills.md` | Route fix for api. subdomain | Needs Alex's access |
| `registry/` dir in repo | May already be intended catalogue index | Currently only contains `authors` |

### External Dependencies
| Dependency | Version | Risk | Alternative |
|------------|---------|------|-------------|
| Cloudflare Workers/Pages | — | Route config location unknown | Direct DNS to origin |
| Stripe | — | Checkout must keep working | — |

## Risks and Unknowns

### Known Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Website/worker source not in any visible repo → can't PR the fix | Med | High | Open question #1; worst case Alex points us at source |
| Republishing a skill changes its hash → breaks `verify_skill` for existing installs | Low | High | Only republish skills whose content is byte-identical; diff before publish |
| Deleting `handover`/`session-start` from backend breaks users who installed them | Med | Med | Prefer backfilling them INTO the repo over deleting from backend |
| Merging PR #29 (untrack node_modules) conflicts with other open PRs | Low | Low | Rebase; it's mechanical |

### Open Questions
1. **Where does the website + API worker source live?** (not in any repo visible to kairo token; issue #21 references `terraphim-skills-server`) — Alex
2. Is `infrastructure/` a collection of 3 sub-skills that should be published individually, or intentionally repo-only? — Alex
3. Were `handover`/`session-start` published from a deleted branch? Should they be restored to main? — Alex
4. Is the REST API supposed to exist at all, or is the website meant to be fully static + MCP-only? (The frontend still calls it → mismatch either way) — Alex

### Assumptions Explicitly Stated

| Assumption | Basis | Risk if Wrong | Verified? |
|------------|-------|---------------|-----------|
| Backend DB holds the 45-skill catalogue and is correct | `list_skills` returns rich metadata | Publish pipeline fixes target the wrong store | Yes (probe) |
| api. 404 is routing, not app crash | MCP on same infra healthy; all routes 404 uniformly incl. /health | Fix approach changes entirely | Strong evidence, not proven |
| Repo main should win conflicts | It's the auditable, signable source | If backend is actually canonical, plan inverts | No — decision for Alex |

### Multiple Interpretations Considered

| Interpretation | Implications | Why Chosen/Rejected |
|----------------|--------------|---------------------|
| A: Repo → backend one-way publish pipeline | Repo canonical; backfill 2 skills into repo, publish 3 missing to backend | **Chosen** — matches "signed, auditable" product claim |
| B: Backend canonical, repo is just a mirror | Different fix: sync repo FROM backend | Rejected: destroys auditability, contradicts signing story |
| C: Website fully static, kill the REST API | Remove frontend fetch + auth buttons | Rejected as default: kills premium/checkout UX; keep as fallback if Q4 says so |

## Research Findings

### Key Insights
1. The backend is **not down** — the REST routing layer is. This is a cheap fix, not a rebuild.
2. Drift is bidirectional: 2 skills exist only in the backend, 3 only in the repo → the publish pipeline is manual and nobody ran it both ways.
3. The website's "42" is a static snapshot that predates at least 3 publishes.
4. Open PR #26 ("installer default source + 402 detection") is likely a **symptom fix for the same broken API** — should be reviewed in this context, possibly superseded.

### Technical Spikes Needed
| Spike | Purpose | Effort |
|-------|---------|--------|
| Locate website/worker source | Confirm routing fix location | 30 min with Alex's answer to Q1 |
| Diff backend skills vs repo content | Establish hash continuity before republish | 1 h scripted |

## Recommendations

### Proceed/No-Proceed
**Proceed.** The fix is well-bounded: one routing fix + one bidirectional sync + one hygiene merge + tracker triage.

### Scope Recommendations
- Phase A (routing) first — it's user-facing and cheap
- Phase B (catalogue reconciliation) second — requires Q2/Q3 answers
- Phase C (hygiene + tracker) last — independent, can parallel

## Next Steps
1. Alex answers Open Questions 1–4
2. Proceed to Phase 2 design (see `terraphim-skills-fix-design.md`)
