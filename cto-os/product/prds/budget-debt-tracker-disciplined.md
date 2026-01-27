# Budget + Debt Recovery Tracker (Terraphim + TrueLayer)

## DISCIPLINED RESEARCH

### 1) Problem framing (what we’re building)
We’re building a personal finance system that can start with a **spreadsheet upload** and optionally graduate to **Open Banking connectivity**. It should:
- Ingest transactions from:
  - **Google Sheet / CSV / XLSX** upload (user-provided exports)
  - **TrueLayer** (bank accounts, cards; loans/balances where available)
- Categorize transactions for budgeting using a **predefined category knowledge graph**:
  - category concepts + synonyms/aliases + merchant normalization
  - “rare but predictable” expenses (e.g., annual vehicle tax) must be recognized and **amortized** into monthly sinking funds
- Produce outputs:
  - categorized spend summaries and budget
  - recommendations to **repay debt** and/or build a **savings plan**
- Deploy as:
  - standalone web app/service
  - or **Clawdbot** workflow (email/WhatsApp trigger, reply in-channel)

### 2) User journeys (critical paths)
**Journey A — Spreadsheet-first onboarding**
1. User uploads transaction sheet.
2. System validates and normalizes columns, currencies, and merchant strings.
3. System categorizes transactions using KG + rules, returning confidence + reason.
4. User reviews “low confidence” items and corrects categories.
5. System generates budget + sinking funds + repayment/savings plan.
6. User exports plan (Markdown/PDF/CSV) and optionally schedules reminders.

**Journey B — Connect accounts (TrueLayer)**
1. User connects accounts via TrueLayer consent.
2. System syncs transactions on schedule.
3. Categorization runs continuously; user overrides feed back into rules/KG.
4. Clawdbot sends periodic summaries (“weekly budget check”, anomalies, reminders).

### 3) Constraints and decision gates (what must be explicit)
Given the sensitivity of bank/transaction data:
- Least-privilege scopes.
- Encrypted token storage.
- Clear separation of:
  - what user approves (intent/risk/outcomes)
  - what user is *informed* about (implementation choices)

### 4) Categorization approach (why KG matters)
We want reliable, explainable categorization without brittle regex-only rules.
- **Deterministic layer** (high precision): known merchant mappings; user overrides.
- **KG layer** (coverage + robustness): match by synonym connectivity and concept neighborhood.
- **Fallback layer** (optional): constrained LLM classification for ambiguous items, always writing provenance and never auto-categorizing below threshold.

### 5) “Rare expense” handling (vehicle tax example)
We treat rare expenses as a first-class budgeting problem:
- Detect “annual-ish” or “irregular but repeatable” patterns (time-series heuristics) or known tax merchants.
- Categorize to `Vehicle Tax`.
- Convert into sinking fund: `annual_amount/12` (or next-due-date based).

### 6) Evaluation plan (how we know it works)
**Categorization quality**
- Labeled set from:
  - user-reviewed corrections (gold labels)
  - seeded synthetic merchant lists per category
- Metrics:
  - top-1 accuracy
  - coverage at confidence thresholds
  - calibration (confidence vs correctness)

**Budget/planning usefulness**
- Reduction in uncategorized spend
- Adoption: % users accepting suggested sinking funds
- Debt plan adherence: % months meeting plan

### 7) Open questions (to answer early)
- TrueLayer coverage differences for loans/credit and which accounts are in-scope for v1.
- What minimum viable schema we require for spreadsheet import.
- Whether we store raw descriptions or only normalized fields for privacy.

---

## DISCIPLINED DESIGN

### 1) Architecture (high level)
```text
            +-------------------+
            |  UI / Clawdbot     |
            |  (upload, review,  |
            |   summaries)       |
            +---------+---------+
                      |
                      v
+---------------------+---------------------+
|          API / Orchestrator              |
|  - auth (TrueLayer)                      |
|  - ingestion (sheet/truelayer)           |
|  - categorization pipeline               |
|  - budgeting + planning                  |
+----------+-------------------+-----------+
           |                   |
           v                   v
+-------------------+   +-------------------+
| Storage            |   | Terraphim KG      |
| - transactions     |   | - concepts        |
| - overrides        |   | - synonyms        |
| - debts/budgets    |   | - graph scoring   |
+-------------------+   +-------------------+
           |
           v
+-------------------+
| Notifications      |
| - email/WhatsApp   |
| - reports          |
+-------------------+
```

### 2) Data model (minimal)
**Transaction**
- `id`
- `source` (sheet|truelayer)
- `source_id` (provider id)
- `account_id`
- `posted_at`
- `amount`, `currency`
- `merchant_raw`, `merchant_norm`, `description`
- `category_concept_id`
- `confidence`
- `reason` (rule|kg|llm)

**CategoryConcept (KG)**
- `concept_id`, `label`
- `synonyms[]`
- `parent_concept_id` (optional)
- `tags[]` (fixed_cost, irregular, debt, tax)

**Override**
- `merchant_norm` → `concept_id`
- `created_at`, `source` (user)

**Debt**
- `type` (credit_card|loan|overdraft)
- `provider`, `balance`, `apr`, `min_payment`, `due_day`

### 3) APIs / integration points
- `POST /import/sheet` (upload)
- `POST /connect/truelayer/start` (consent)
- `POST /connect/truelayer/callback`
- `POST /sync/truelayer` (job)
- `POST /categorize/run` (batch)
- `POST /overrides` (user corrections)
- `GET /budget/summary?month=`
- `GET /plan/debt` (avalanche/snowball)

### 4) Terraphim touchpoints (high-level)
- Use Terraphim types for representing:
  - concepts, synonyms, and graph adjacency
  - retrieval/scoring interfaces
- Store category ontology as a versioned artifact (so changes are traceable)

### 5) TrueLayer integration (high-level)
- OAuth consent, token storage, periodic sync.
- Scope minimization; encrypted tokens at rest.
- Audit log for access.

### 6) Deployment modes
**Standalone**
- Web UI + API + DB + scheduled sync jobs

**Clawdbot**
- Workflows:
  - “email spreadsheet → return categorized budget + plan”
  - “weekly budget check” scheduled messages

### 7) Phased delivery plan
**Phase 0**
- Define category ontology + synonyms
- Define sheet schema and import

**Phase 1**
- Categorization pipeline + review UI
- Budget summary + sinking funds

**Phase 2**
- Debt payoff planner + savings planner
- Overrides feedback loop

**Phase 3**
- TrueLayer connectivity + sync
- Clawdbot triggers/notifications

---

## GitHub Issue Summary
Design and build a Budget + Debt Recovery Tracker that starts from spreadsheet uploads and optionally connects to accounts via TrueLayer. Use Terraphim’s knowledge graph (synonym connectivity) to categorize transactions—especially rare/irregular expenses like annual vehicle tax—then produce a budget with sinking funds plus a prioritized debt repayment and/or savings plan. Support both a standalone deployment and a Clawdbot workflow (email/WhatsApp triggers). Include an evaluation plan, data model, security/privacy posture, and phased delivery milestones.
