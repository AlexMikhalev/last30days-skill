# Implementation Plan: terraphim-skills Catalogue & Platform Consistency Fix

**Status**: Draft
**Research Doc**: `plans/terraphim-skills-fix-research.md`
**Author**: Kairo
**Date**: 2026-07-25
**Estimated Effort**: 1–1.5 days total (4 independent workstreams, most < 2 h each)

## Overview

### Summary
Restore a single source of truth for the Terraphim skills catalogue: fix the dead REST API routing, reconcile the three-way catalogue drift (repo 45 / backend 45 / website 42), merge the artefact-cleanup, and triage the issue tracker.

### Approach
Research interpretation **A**: repo `main` is canonical; backend is a publish target; website renders from backend. Fix routing first (user-facing), then reconcile content both directions, then hygiene.

### Scope
**In Scope:**
- WS-A: Restore `api.terraphim-skills.md/skills` (the exact route the frontend calls) to 200
- WS-B: Bidirectional catalogue reconciliation (2 skills backend→repo, 3 skills repo→backend, website counter)
- WS-C: Merge artefact cleanup (PR #29 or re-do), verify `.gitignore` covers `node_modules`, `*.skill`, `EOF`
- WS-D: Issue/PR triage (#30 dupe, #26 supersede decision, epic status updates)

**Out of Scope:**
- New skills (linear-ticket-writer, PR #32 review stays on its own track)
- Content bugs #12/#13/#14
- Website redesign, monetisation changes

**Avoid At All Cost** (5/25):
- Rewriting the backend or migrating hosting — MCP proves it works; routing fix only
- Republishing skills without content diff — risk of breaking `verify_skill` hashes for existing installs
- Building an automated CI publish pipeline now — manual documented sync first; automate only after it's proven (that's the real Phase 2 epic work)

## Architecture

### Data Flow (target state)
```
skills/*/SKILL.md (repo main, canonical, signed)
        │  publish_skill (MCP tool, manual for now)
        ▼
marketplace backend DB ──► MCP endpoint (mcp.terraphim-skills.md/mcp) ✅ already works
        │
        ▼
REST API (api.terraphim-skills.md/skills) ──► website frontend (fetch)
```

### Key Design Decisions
| Decision | Rationale | Alternatives Rejected |
|----------|-----------|----------------------|
| Repo main canonical | Signing/audit story; PR workflow | Backend-canonical (loses auditability) |
| Backfill `handover` + `session-start` INTO repo | They have real installs; deleting breaks users | Delete from backend (breaks users) |
| Publish `terraphim-linear`, `linear-issue-to-pr` to backend | They're on main and complete | Hold for review (main IS the review gate) |
| `infrastructure/*` → publish 3 sub-skills individually (pending Q2) | Matches how xero etc. are actually used | Publish as one bundle (unnavigable) |
| Routing fix at Cloudflare worker/route layer | Same-IP DNS; MCP route works on same zone | New host / DNS split (unnecessary) |

### Simplicity Check
**What if this could be easy?** The backend works. The catalogue exists in both places. The minimal fix is: repair one route, copy 5 skills across the gap, merge one cleanup PR, close one dupe issue. No new abstractions, no new services, no CI build-out.

**Nothing Speculative Checklist**:
- [x] No features the user didn't request
- [x] No abstractions "in case we need them later" (no CI pipeline yet)
- [x] No premature optimization

## File Changes

### WS-A: Frontend endpoint fix (REVISED — API was never down)
| File | Changes |
|------|---------|
| `deploy/static-site/` on bigbox (**untracked — move into git first**) | Change fetch URL `https://api.terraphim-skills.md/skills` → `https://api.terraphim-skills.md/api/catalogue`; update "42" counter to render from response length |
| `terraphim-skills-server/src/main.rs` (optional belt-and-braces) | Add alias route `.route("/skills", get(skills_api::catalogue))` so old frontend builds don't break |
| New: commit `static-site/` into `terraphim-skills-server` repo (or its own repo) | Makes the frontend reviewable/PR-able; currently impossible to fix without host access |

### WS-B: Catalogue reconciliation
| File | Changes |
|------|---------|
| `skills/handover/SKILL.md` (+ assets) | **New** — backfill from backend (`show_skill`/`install_skill` content) |
| `skills/session-start/SKILL.md` (+ assets) | **New** — backfill from backend |
| Backend publish: `terraphim-linear`, `linear-ticket-writer`, `infrastructure/{1password-secrets,caddy,xero}` | `publish_skill` via MCP after content diff |
| Website homepage + `/skills` page | Counter 42 → live count; listing rendered from `/api/catalogue` |
| `README.md` (repo) | Add "Publishing" section documenting the manual sync procedure |

### WS-C: Hygiene — ✅ DONE (PR #29 merged 2026-07-25; 0 tracked artefacts). Remainder: close issue #28, close stale PR #29 entry, delete merged branches

### WS-D: Tracker triage
| Item | Action |
|------|--------|
| Issue #30 | Close as duplicate of #31 |
| Issue #28 | Close (fixed by merged PR #29) |
| PRs #29, #32 | Close stale open entries (content already merged via branch merges) |
| PR #26 | Review in light of tier-skew finding: the 402s it detects are *by design* for premium tier — decide merge or close |
| Issues #17/#18 (Phase 1/2) | Update status: Phase 1 shipped, Phase 2 partially (MCP yes, CLI auth no) |
| Issue #21 | Link to WS-B findings |

### WS-E (NEW): Product decision — tier skew
44/45 skills are premium → the advertised zero-setup install (`/skills/{name}/SKILL.md`) 402s for nearly everything, and the homepage promise "install any skill with one command" only holds for subscribers. Decide: intended paywall (then fix marketing copy + merge PR #26's 402 messaging) or more community-tier skills (then re-tier).

## API Design

No new APIs. Contract to restore (consumed by the live frontend):
```
GET https://api.terraphim-skills.md/skills
→ 200 application/json
→ [{ name, description, tier, version, author, category }, ...]  (same shape as MCP list_skills)
```

## Test Strategy

### Verification per workstream
| Test | How | Pass criteria |
|------|-----|---------------|
| WS-A route restoration | `curl -s -o /dev/null -w '%{http_code}' https://api.terraphim-skills.md/skills` | 200 |
| WS-A frontend integration | agent-browser: open `/skills`, check `performance.getEntriesByType('resource')` for api. call status | No failed fetch; OAuth buttons reach a non-404 page |
| WS-B repo→backend parity | Script: diff `ls skills/ | grep -v '.skill'` vs MCP `list_skills` names | Empty symmetric diff |
| WS-B install round-trip | `install_skill terraphim-linear` (MCP) then `verify_skill` | Installs, signature OK |
| WS-B hash continuity | Before republish of any existing skill: `show_skill` content vs repo file — byte diff | Only publish byte-identical or intended changes |
| WS-C cleanliness | `git ls-files node_modules \| wc -l` → 0; `git ls-files '*.skill'` → 0 | 0, 0 |
| WS-D tracker | `gitea-robot list-issues --owner terraphim --repo terraphim-skills` | No dupes; each issue has status comment |

## Implementation Steps

### Step 0: Answers (BLOCKER for WS-A, partial WS-B)
**Owner:** Alex · **Est:** 5 min
Answer research-doc open questions Q1 (website/worker source repo), Q2 (infrastructure publishing), Q3 (handover/session-start provenance), Q4 (REST API vs static).

### Step 1: WS-A — Restore API route
**Est:** 1–2 h (assuming worker source located)
1. Locate worker/site source (per Q1)
2. Add/repair `/skills` (+ `/health`) route serving the same catalogue the MCP `list_skills` reads
3. Deploy; run WS-A verification tests
4. If source unreachable: fallback — patch frontend to render fully static and hide OAuth buttons (documents decision for Q4)

### Step 2: WS-B — Reconcile catalogues
**Est:** 2–3 h · **Dependencies:** Step 0 (Q2/Q3)
1. Backfill `handover`, `session-start` into repo (`install_skill` → write `skills/<name>/` → commit on branch `fix/catalogue-resync`)
2. Content-diff backend vs repo for all 43 common skills; log any drift
3. Publish `terraphim-linear`, `linear-issue-to-pr` (+ infrastructure sub-skills per Q2) via `publish_skill`
4. Update website counter + regenerate listing
5. Document the manual publish procedure in repo README
6. Run WS-B verification tests

### Step 3: WS-C — Hygiene merge
**Est:** 30 min · **Independent**
1. Checkout `task/28-stop-tracking-artefacts`, rebase on main
2. Verify deletion set (1,473 node_modules + 2 .skill zips + stray `EOF` file)
3. Merge PR #29 (closes #28); verify cleanliness tests

### Step 4: WS-D — Tracker triage
**Est:** 30 min · **Independent**
1. Close #30 as dupe of #31 (comment cross-link)
2. Comment on PR #26 with WS-A outcome (merge if still relevant)
3. Status comments on #17/#18/#21; link this plan

### Step 5: End-to-end verification
**Est:** 30 min · **Dependencies:** Steps 1–4
Run full test table; post summary comment on issue #20 (marketplace content audit).

## Rollback Plan
- WS-A: routing change is config — revert route binding; website returns to current (static, degraded) state
- WS-B: backend additions are additive (no deletes) → nothing to roll back; repo backfill is a normal revertible commit
- WS-C: untracking is reversible (`git rm` undo); node_modules remain on disk locally
- No destructive operation exists anywhere in this plan

## Open Items
| Item | Status | Owner |
|------|--------|-------|
| Q1–Q4 answers | Pending | Alex |
| Cloudflare zone access for route fix | Pending | Alex |
| Confirm `xero` sub-skill doesn't contain live credentials before publishing | Pending | Kairo (check in Step 2) |

## Approval
- [ ] Technical review complete
- [ ] Test strategy approved
- [ ] Human approval received
