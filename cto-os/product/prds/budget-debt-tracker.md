# Design Proposal: Budget + Debt Recovery Tracker (Terraphim + TrueLayer)

## TL;DR
A system that ingests transaction data via:
1) **Uploaded Google Sheet** (CSV-like), and/or
2) **Connected accounts** (TrueLayer: cards, bank accounts, loans)

…and uses **Terraphim knowledge graph / graph embeddings (synonyms)** to categorize transactions (including rare events like annual vehicle tax), produce budgets, and recommend a **repayment and/or savings plan**.

Deployable as:
- **Standalone web app** (Terraphim service + UI)
- **Clawdbot workflow** (email/WhatsApp triggered, results returned in-channel)

---

## Goals
- Categorize transactions with high accuracy and explainability.
- Detect recurring/rare expenses (e.g., once-a-year vehicle tax) and budget for them.
- Turn spending into: **budget**, **priorities**, **debt payoff plan**, **savings plan**.
- Make it easy to onboard via spreadsheet and later upgrade to account connectivity.

## Non-goals (for v0)
- Automated payments / debt restructuring.
- Real-time credit underwriting.
- Tax/legal advice.

---

## User Journey

### Path A — Upload Google Sheet
1. User uploads a Google Sheet export (CSV/XLSX) of transactions.
2. System validates schema, normalizes amounts/dates/merchant names.
3. Terraphim categorizes each transaction using a **category graph** (synonyms + term connectivity).
4. System detects:
   - monthly recurring bills
   - subscription-like patterns
   - irregular but predictable expenses (annual tax, insurance)
5. User confirms/overrides a handful of uncertain categories.
6. System generates:
   - monthly budget buckets
   - “true monthly cost” for annual items (annual/12)
   - prioritized plan: debt repayment vs savings goals

### Path B — Connect Accounts (TrueLayer)
1. User connects accounts using TrueLayer consent flow.
2. Sync transactions periodically (daily / weekly).
3. Categorization + budgeting runs automatically.
4. System messages the user (email/WhatsApp) with updates, anomalies, and nudges.

---

## Core System Components

### 1) Ingestion Layer
**Inputs**
- Spreadsheet: CSV/XLSX export of transactions
- TrueLayer: transactions for accounts + cards, and loan balances where available

**Normalization**
- Standardize fields: `date`, `amount`, `currency`, `merchant`, `description`, `account_id`, `source`
- Normalize merchant strings (lowercase, strip punctuation, collapse whitespace)
- Deduplicate (hash by date/amount/merchant/source id)

**Outputs**
- `Transaction` records in local store

### 2) Knowledge Graph Category System (Terraphim)
We maintain a domain KG with:
- **Concepts**: `Groceries`, `Rent`, `Vehicle Tax`, `Insurance`, `Subscriptions`, `Debt Payment`, etc.
- **Synonyms / aliases**: merchant variants, abbreviations, common descriptors
- **Rules**: optional deterministic matches for high-precision merchants

Terraphim Graph scorer concept: rank/match influenced by synonym connectivity.

**Key design point**: categorize using a blend of:
1) deterministic rules (e.g., known loan provider, HMRC-style strings)
2) KG synonym/term matching (graph embeddings)
3) optional LLM fallback for ambiguous cases (with citations)

### 3) Categorization Pipeline
For each transaction:
1. Run `normalize(merchant, description)`
2. Candidate category retrieval:
   - Terraphim search/autocomplete against category thesaurus
   - include merchant synonyms and past user overrides
3. Score using TerraphimGraph + confidence
4. If below threshold, route to:
   - “needs review” UI
   - or LLM-assisted classification with constrained output and provenance

### 4) Budget Engine
- Compute totals by category per month
- Identify:
  - fixed costs
  - variable costs
  - irregular costs (annual/quarterly)
- Convert irregular to **amortized monthly**
- Estimate free cash flow and buffers

### 5) Debt/Savings Planner
Inputs:
- debts (balances, APR, min payments) from:
  - spreadsheet user input, and/or
  - TrueLayer / manual entry

Planner outputs:
- payoff plan options:
  - avalanche (highest APR first)
  - snowball (smallest balance first)
  - hybrid (risk-adjusted)
- savings plan:
  - emergency fund target
  - sinking funds (vehicle tax/insurance)

### 6) Delivery Surfaces
**Standalone**
- Web UI: upload, review categorizations, dashboard, plan generation

**Clawdbot**
- user emails a sheet → system replies with analysis
- user texts “update my budget” → system refreshes and replies

---

## Data Model (proposed)

### Transaction
- `id` (uuid)
- `source` (sheet|truelayer)
- `source_id` (provider id)
- `account_id`
- `posted_at`
- `amount`
- `currency`
- `merchant_raw`
- `merchant_norm`
- `description`
- `category_id` (canonical KG concept)
- `category_confidence`
- `category_reason` (rule hit / KG match / LLM)

### CategoryConcept (KG)
- `concept_id`
- `label`
- `synonyms[]`
- `parent_concept_id` (optional)
- `tags` (needs_review, fixed_cost, debt, tax, etc.)

### UserOverrides
- `merchant_norm` → `concept_id`
- `confidence_boost`

### Debt
- `id`
- `type` (credit_card|loan|overdraft)
- `provider`
- `balance`
- `apr`
- `min_payment`
- `due_day`

---

## Terraphim Crates to Leverage (high-level)
- `terraphim_types`: Document/Index, SearchQuery, routing types, KG types
- Terraphim Graph scorer: synonym-connectivity scoring for category matches
- `terraphim_mcp_server` / service APIs (if we expose categorization as tools)
- `terraphim_service` for search + indexing of category ontology and user overrides

---

## TrueLayer Integration (high-level)
- OAuth/consent + token storage
- Fetch accounts + transactions
- Optional: balances/loans where supported

Security:
- Encrypt tokens at rest
- Least-privilege scopes
- Audit log for access

---

## Deployment Options

### A) Standalone service
- Terraphim service + API
- DB for transactions + overrides
- Web UI

### B) Clawdbot workflow
- Email/WhatsApp triggers
- Fetch latest data, run categorization + planning
- Reply with:
  - summary
  - flagged anomalies
  - next action prompts

---

## Risks / Open questions
- Transaction categorization ground truth: how to evaluate accuracy?
- TrueLayer access coverage for loans/credit cards varies by bank.
- Handling refunds/chargebacks.
- Multi-currency.
- Privacy/compliance requirements.

---

## Phased Plan

### Phase 0 (1–2 days)
- Define canonical category ontology + synonyms (starter graph)
- Define spreadsheet schema and import

### Phase 1 (1 week)
- Basic categorization pipeline + review UI
- Monthly budget summary + irregular amortization

### Phase 2 (1–2 weeks)
- Debt planner + savings planner
- User overrides feedback loop into KG

### Phase 3 (2–4 weeks)
- TrueLayer connectivity + sync job
- Clawdbot email/WhatsApp triggers

---

## Acceptance Criteria (MVP)
- Upload a sheet → categories assigned with confidence + explainability.
- Rare annual expense detected + amortized monthly.
- Generate a budget + a debt payoff plan (avalanche/snowball).
- Export plan (markdown/pdf) and optionally deliver via email.
