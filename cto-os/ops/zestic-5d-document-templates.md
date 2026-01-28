# Product & AI Development Document Templates

# 

# 

# 

# 

# 

# 

# 

# 

# **Zestic AI 5D:**  **Canonical Product & AI Development** **Account Management and Document Templates**  **DRAFT**

[**Purpose	3**](#purpose)

[Canonical Document Map	3](#canonical-document-map)

[**Account Manager Playbook (AI-First)	5**](#account-manager-playbook-\(ai-first\))

[The AM’s core mission	5](#the-am’s-core-mission)

[What the AM owns (explicitly)	5](#what-the-am-owns-\(explicitly\))

[What AMs must not do	6](#what-ams-must-not-do)

[One rule that protects Zestic AI legally and reputationally	6](#one-rule-that-protects-zestic-ai-legally-and-reputationally)

[How this all fits together (mental model)	6](#how-this-all-fits-together-\(mental-model\))

[Approval Matrix (DRAFT)	7](#approval-matrix-\(draft\))

[**Zestic 5D Document Templates (DRAFT)	8**](#zestic-ai-5d-document-templates-\(draft\))

[1\. Product Vision & Value Hypothesis (PVVH)	8](#1.-product-vision-&-value-hypothesis-\(pvvh\))

[2\. Personas & Jobs‑to‑be‑Done Pack	10](#2.-personas-&-jobs‑to‑be‑done-pack)

[3\. Domain Model	11](#3.-domain-model)

[4\. End‑to‑End Business Scenarios	12](#4.-end‑to‑end-business-scenarios)

[5\. AI & System Design Brief	13](#5.-ai-&-system-design-brief)

[6\. Responsible AI & Risk Register	14](#6.-responsible-ai-&-risk-register)

[7\. UX Flows & Interaction Contracts	15](#7.-ux-flows-&-interaction-contracts)

[8\. Prompt & Agent Specification	16](#8.-prompt-&-agent-specification)

[9\. Data & Model Specification	17](#9.-data-&-model-specification)

[10\. Acceptance Criteria & Test Scenarios	18](#10.-acceptance-criteria-&-test-scenarios)

[11\. Release & Ops Runbook	19](#11.-release-&-ops-runbook)

[12\. Performance, Drift & Learning Report (Living)	20](#12.-performance,-drift-&-learning-report-\(living\))

[**Client Governance	21**](#client-governance)

[Purpose	21](#purpose-1)

[Client Gates	21](#client-gates)

[What Clients Are Not Asked to Approve	22](#what-clients-are-not-asked-to-approve)

[Roles & Responsibilities (Client-Facing)	22](#roles-&-responsibilities-\(client-facing\))

[Communication Principles	23](#communication-principles)

[Escalation Rule	23](#escalation-rule)

[Purpose	23](#purpose-2)

[Roles	23](#roles)

[RACI by Client Gate	24](#raci-by-client-gate)

[RACI Rules (Non-Negotiable)	25](#raci-rules-\(non-negotiable\))

[Purpose	25](#purpose-3)

[How the Decision Log Is Used	25](#how-the-decision-log-is-used)

[Decision Log Template (Repeat per decision)	26](#decision-log-template-\(repeat-per-decision\))

[Governance Rules	26](#governance-rules)

[Why This Protects Everyone	27](#why-this-protects-everyone)

# **Purpose** {#purpose}

This document defines the **Account Management Playbook as an end‑to‑end set of product & AI development documents** used at Zestic AI.

Each document has a single responsibility and is designed to be usable **by humans and LLMs**.

## **Canonical Document Map** {#canonical-document-map}

| Phase | \# | Document | Primary Purpose | Key Decisions Locked | Owner |
| :---- | :---- | :---- | :---- | :---- | :---- |
| **DEFINE** | 1 | Product Vision & Value Hypothesis (PVVH) | Define *why* we are building and what success means | Problem framing, value hypothesis, success metrics, and ethical boundaries | Product Owner |
|  | 2 | Personas & Jobs-to-be-Done Pack | Anchor product & AI behaviour in real user intent | Target users, JTBD, risks & motivations | UX / Product |
|  | 3 | End-to-End Business Scenarios | Describe how work *actually* happens | Scope boundaries, happy paths, key variants | Product |
|  | 4 | Domain Model (Atomic / Ontology) | Establish shared language for humans & machines | Core entities, relationships, events, vocabulary | Product \+ Architect |
| **DESIGN** | 5 | AI & System Design Brief | Translate intent into system-level structure | Architecture pattern, AI approach, non-goals | System / ML Architect |
|  | 6 | Responsible AI & Risk Register | Make ethics, compliance & safety operational | Risk acceptance, mitigations, audit checkpoints | Responsible AI Lead |
|  | 7 | UX Flows & Interaction Contracts | Define user experience and AI interaction limits | UX flows, AI boundaries, tone & system messages | UX Lead |
| **DEVELOP** | 8 | Prompt & Agent Specification | Treat prompts as first-class product assets | Prompt structure, guardrails, agent roles | Prompt Eng / Product |
|  | 9 | Data & Model Specification | Make data and model assumptions explicit | Data sources, schemas, model configs | Data / ML Engineer |
|  | 10 | Acceptance Criteria & Test Scenarios | Define “working” before shipping | Behavioural correctness, edge cases | QA \+ Product |
| **DEPLOY** | 11 | Release & Ops Runbook | Ensure system can be run safely | Deployment, rollback, monitoring, support | DevOps / MLOps |
| **DRIVE** | 12 | Performance, Drift & Learning Report (Living) | Close the learning loop | Iteration decisions, retraining, A/B outcomes | Product \+ ML |

### **How to read this table (important)**

* **Mandatory** \= must exist before moving to the next phase gate  
* Documents **3, 7, 8** are the *highest-leverage assets* in AI-first delivery

# 

# **Account Manager Playbook (AI-First)** {#account-manager-playbook-(ai-first)}

## **The AM’s core mission** {#the-am’s-core-mission}

Ensure the client is **never surprised**, **never confused**, and **never asked to approve the wrong thing**.

The AM owns **clarity**, not delivery.

## **What the AM owns (explicitly)** {#what-the-am-owns-(explicitly)}

### **1\. Client Gate Ownership**

* Schedule all 5 gates upfront  
* Ensure:  
  * decision required is clear  
  * materials are pre-read  
  * sign-off is documented

No retroactive approvals. Ever.

### **2\. Decision Framing**

Every client interaction must answer:

1. What changed?  
2. Why it changed?  
3. Is a decision required?

If no decision → async update.

### **3\. Risk Translation**

The AM translates:

* AI risk → business impact  
* model behaviour → user trust implications  
* technical uncertainty → decision options

They do **not** explain implementation detail unless asked.

### **4\. Communication Cadence (best practice)**

**Weekly (30 min): Intent Check**

* Are we still solving the right problem?  
* Are we still within agreed risk tolerance?  
* Is anything making the client uneasy?

**Gate meetings**

* Decision-only  
* No status theatre

### **5\. The AM Red Flags (intervene immediately)**

* “Let’s just see how it goes”  
* “We’ll sort that out after launch”  
* “The AI decided that on its own”  
* Silent stakeholder drift

## **What AMs must *not* do** {#what-ams-must-not-do}

❌ Commit to scope changes without a gate  
 ❌ Shield the client from necessary decisions  
 ❌ Let delivery teams negotiate sign-off directly

## **One rule that protects Zestic AI legally and reputationally** {#one-rule-that-protects-zestic-ai-legally-and-reputationally}

**If it affects value, trust, or risk — it must pass a Client Gate.**

## **How this all fits together (mental model)** {#how-this-all-fits-together-(mental-model)}

* **Documents** \= evidence  
* **Client Gates** \= decisions  
* **Account Managers** \= guardians of trust

This is exactly how top AI consultancies scale without blowing up relationships.

## 

## **Approval Matrix (DRAFT)** {#approval-matrix-(draft)}

| Phase | Internal Document | Client Role | Sign-off Type |
| :---- | :---- | :---- | :---- |
| **Define** | 1.Product Vision & Value Hypothesis | Co-author | **Formal approval** |
|  | 2.Personas & JTBD | Review | Acknowledge |
|  | 3.Domain Model | Validate language | Confirm accuracy |
|  | 4.Business Scenarios | Validate reality | Confirm completeness |
| **Design** | 5.AI & System Design Brief | Review summary | Decision acceptance |
|  | 6.Responsible AI & Risk Register | Co-owner | **Formal approval** |
|  | 7.UX Flows & Interaction Contracts | Usability review | **Formal approval** |
| **Develop** | 8.Prompt & Agent Spec | Demo-based review | Accept behaviour |
|  | 9.Data & Model Spec | Informed | No sign-off |
|  | 10.Acceptance Criteria | Agree definition of done and test protocols and scenarios | **Formal approval** |
| **Deploy** | 11.Release & Ops Runbook | Informed | No sign-off |
| **Drive** | 12\. Performance & Drift Report | Review outcomes | Agree actions |

## 

# **Zestic AI 5D Document Templates (DRAFT)** {#zestic-ai-5d-document-templates-(draft)}

## **1\. Product Vision & Value Hypothesis (PVVH)** {#1.-product-vision-&-value-hypothesis-(pvvh)}

### **Purpose**

Define **why** the product exists, **who** it is for, and **how success is measured**. This document anchors all downstream decisions.

### **When this document is created / updated**

* At product inception  
* When the problem framing or success metrics change

### **Template**

**Product / Initiative name**

**One‑line value statement**

This product helps \_\_\_ achieve \_\_\_ by \_\_\_.

**Problem statement**

* What problem are we solving?  
* Who experiences this problem?  
* Why does it matter now?

**Target users**

* Primary users  
* Secondary / indirect users  
* Explicitly excluded users

**Value hypothesis**

* If we build \_\_\_  
* For \_\_\_  
* Then we expect \_\_\_  
* Because \_\_\_

**Success metrics**  
*Leading indicators*

* …

*Lagging indicators*

* …

**Strategic fit**

* How does this support company strategy?  
* Key dependencies or assumptions

**Ethical & trust boundaries**

* What the product must never do  
* Sensitive decisions or data involved

**In scope / Out of scope**  
*In scope*

* …

*Out of scope*

* …

**Open questions & assumptions**

* …

## 

## **2\. Personas & Jobs‑to‑be‑Done Pack** {#2.-personas-&-jobs‑to‑be‑done-pack}

### **Purpose**

Ensure the product and AI behaviour are grounded in **real user intent, context, and risk tolerance**.

### **When this document is created / updated**

* Early discovery  
* When new user groups are introduced

### **Template (repeat per persona)**

**Persona name**

**Role & context**

* Job role  
* Environment and constraints

**Goals**

* …

**Primary Job‑to‑be‑Done**

When \_\_\_, I want to \_\_\_, so that \_\_\_.

**Secondary Jobs‑to‑be‑Done**

* …

**Pains & blockers**

* …

**Risks & sensitivities**

* Data sensitivity  
* Error tolerance  
* Over‑reliance on AI

**Accessibility & inclusion considerations**

* …

## **3\. Domain Model**  {#3.-domain-model}

### **Purpose**

Create a **shared, unambiguous language** for business, product, engineering, and AI systems.

**Important: include data flow**

Domain modeling is not just nouns and relationships — it must also capture the **data flow** that makes the system real:

- **Systems of record** (where each entity’s truth lives)
- **Sources → transformations → destinations** (incl. joins/enrichment)
- **Events and state transitions** that generate/consume data
- **Latency/freshness** expectations (real-time vs batch)
- **Quality constraints** (missing fields, duplication, reconciliation rules)
- **Access boundaries** (PII, permissions, retention)

If the data flow is unclear, AI behavior and product behavior will drift — because the system can’t reliably answer: *what do we know, when do we know it, and where did it come from?*

### **When this document is created / updated**

* After initial discovery  
* Whenever core concepts change

### **Template**

**Core business concepts (entities)**

* Name  
  * Description  
  * Key attributes and their definitions, and what’s included and excluded (boundary conditions)  
  * Dependencies

**Relationships**

* Entity A → Entity B (relationship type)

**Business events**

* Event name  
* Trigger  
* Outcome

**Vocabulary & definitions**

* Term → definition, examples and counter-examples

**Notes & assumptions**

* …

## 

## **4\. End‑to‑End Business Scenarios** {#4.-end‑to‑end-business-scenarios}

### **Purpose**

Describe how work **actually happens** across users, systems, and AI components.

### **When this document is created / updated**

* Before detailed design  
* When workflows change

### **Template (repeat per scenario)**

**Scenario name**

**Business objective**

**Trigger**

* …

**Actors involved**

* …

**Trigger**

* …

**Supporting systems and tools**

* 

**Preconditions**

* 

**Main flow**

1. …  
2. …

**Variants & edge cases**

* …

**Expected outcome**

* …

**Business value**

* 

**Assumptions and expectations**

* …

**What specifically excluded**

* …

## 

## **5\. AI & System Design Brief** {#5.-ai-&-system-design-brief}

### **Purpose**

Translate intent into **clear architectural and AI design decisions**.

### **When this document is created / updated**

* Before development starts  
* When architecture changes materially

### **Template**

**System overview**

* What the system does at a high level

**Architecture approach**

* Key components  
* Integration points  
* Data availability  
* Authorisations and access management  
* Release and maintenance procedures

**AI patterns used**

* (e.g. RAG, fine‑tuning, agents)

**Model responsibilities**

* What AI does vs what software does

**Non‑goals & trade‑offs**

* …

**Key risks**

* …

## 

## **6\. Responsible AI & Risk Register** {#6.-responsible-ai-&-risk-register}

### **Purpose**

Make ethics, compliance, and trust **explicit and auditable**.

Principles for including / excluding risks are here: [5D:Product & AI Development Account Management and Document Templates](https://docs.google.com/document/d/1AEJWCHGk8fKoW-UmEGNXxUUVzoJLSonVWddui9YnLeM/edit?tab=t.5ho0xe3s1s1r)

### **When this document is created / updated**

* During design  
* Reviewed before release

### **Template (repeat per risk)**

**Risk description**

* …

**Impact**

* …

**Mitigation**

* …

**Owner**

* …

**Review checkpoint**

* …

## **4\. Risk & Trust Handling**

### **Charm Approach**

* Risk is treated as:

  * Preconditions

  * Blocks

  * Exceptions

  * Hard stops

* Extremely strong **operational risk control**

* Less explicit on:

  * User trust perception

  * Business-level risk framing

  * Ethical or AI-specific concerns (outside compliance)

### **5D Approach**

* Explicit **Responsible AI & Risk Register**

* Risks framed as:

  * Business impact

  * User trust impact

  * Ethical boundary violations

* Tied to **Gate 3 – Trust & Safety**

**Key Difference**

* Charm optimised for **regulatory and operational defensibility**

* 5D optimises for **client confidence and reputational safety**

**Improvement for 5D**

* Import Charm’s **exception taxonomy discipline** into the Risk Register:

  * Explicit risk states

  * Resolution paths

  * Ownership and expiry semantics

* Make risks feel less abstract and more “system-real”.

## 

## **7\. UX Flows & Interaction Contracts** {#7.-ux-flows-&-interaction-contracts}

### **Purpose**

Define the **user experience** and explicit **AI behaviour boundaries**.

### **When this document is created / updated**

* During design  
* When UX or AI behaviour changes

### **Template**

**User journey / flow name**

**Steps**

1. …  
2. …

**AI behaviour**

* What the AI is allowed to do  
* What the AI must not do

**Human override points**

* …

**Tone & system messages**

* …

## 

## **8\. Prompt & Agent Specification** {#8.-prompt-&-agent-specification}

### **Purpose**

Treat prompts and agents as **first‑class product assets**.

### **When this document is created / updated**

* During development  
* Whenever prompts change materially

### **Template**

**Agent / prompt name**

**Role & responsibility**

* …

**Inputs**

* …

**Expected outputs**

* …

**Guardrails & constraints**

* …

**Failure handling**

* …

## 

## **9\. Data & Model Specification** {#9.-data-&-model-specification}

### **Purpose**

Make **data and model assumptions explicit**.

### **When this document is created / updated**

* During development  
* When data sources or models change

### **Template**

**Data sources**

* …

**Schemas & formats**

* …

**Data quality rules**

* …

**Model configuration**

* …

## 

## **10\. Acceptance Criteria & Test Scenarios** {#10.-acceptance-criteria-&-test-scenarios}

### **Purpose**

Define what **"working" means before building or shipping**.

### **When this document is created / updated**

* Before implementation  
* Updated per iteration

### **Template**

**Acceptance criteria**

* …

**Test scenarios**

* Scenario  
* Expected outcome

**Edge cases**

* …

## 

## **11\. Release & Ops Runbook** {#11.-release-&-ops-runbook}

### **Purpose**

Ensure the system can be **deployed, monitored, and supported safely**.

### **When this document is created / updated**

* Before first release  
* Updated continuously

### **Template**

**Deployment steps**

* …

**Rollback strategy**

* …

**Monitoring & alerts**

* …

**Incident response**

* …

## 

## **12\. Performance, Drift & Learning Report (Living)** {#12.-performance,-drift-&-learning-report-(living)}

### **Purpose**

Close the loop by tracking **performance, drift, and learning over time**.

### **When this document is created / updated**

* After release  
* Reviewed on a regular cadence

### **Template**

**KPIs & signals**

* …

**Observed issues / drift**

* …

**Decisions taken**

* …

**Follow‑up actions**

* …

**Governance rule**  
If a decision matters, it must live in exactly one of these documents.

# **Client Governance** {#client-governance}

## **Purpose** {#purpose-1}

This section explains **how clients are involved**, **what they are asked to decide**, and **how Zestic AI ensures transparency, trust, and momentum** throughout the delivery lifecycle.

AI-driven development moves fast. Clear governance ensures:

* no surprises  
* shared ownership of risk  
* faster, safer decision-making

Clients are **not asked to approve technical implementation details**. They are asked to approve **intent, value, risk, and outcomes**.

## **Client Gates** {#client-gates}

Client Gates are formal decision points where Zestic AI pauses delivery to ensure alignment and secure explicit client approval on the matters that count.

### **Gate 1 — Value & Intent Alignment**

**When:** End of Define phase  
**Client approves:**

* The problem being solved  
* Target users  
* Success metrics:  
  * We are responsible for defining success metrics together with the client and designing a system that is capable of supporting those outcomes. We are not responsible for achieving the business outcome itself, because that depends on factors outside our control (sales execution, operations, change management, market conditions, etc.). The purpose of agreeing KPIs upfront is to ensure alignment of intent, not to transfer ownership of business results to Zestic AI.  
* Ethical and trust boundaries

**Outcome:**  
"Yes, this is the right problem to solve, in the right way."

### **Gate 2 — Reality Check**

**When:** After discovery and early design  
**Client confirms:**

* Business language and terminology are correct  
* Personas and scenarios reflect real workflows  
* No critical users or processes are missing

**Outcome:**  
"Yes — this reflects how our organisation actually works."

### **Gate 3 — Trust & Safety**

**When:** Before scaling AI behaviour or exposing to users  
**Client approves:**

* AI interaction boundaries  
* Human override and escalation points  
* Responsible AI risks and mitigations

**Outcome:**  
"Yes — we are comfortable letting this AI interact with users."

### **Gate 4 — Definition of Done**

**When:** Before release  
**Client approves:**

* Acceptance criteria  
* Key test scenarios  
* What success looks like at launch

**Outcome:**  
"Yes — we agree on what success looks like."

### **Gate 5 — Learn & Decide**

**When:** Post-launch, on a recurring cadence  
**Client reviews and agrees on:**

* Observed outcomes and KPIs  
* Model or UX drift  
* Decisions to improve, stop, or scale

**Outcome:**  
"Yes — we agree on the next investment decision."

## **What Clients Are Not Asked to Approve** {#what-clients-are-not-asked-to-approve}

To maintain velocity and clarity, clients are **not asked to approve**:

* Code or infrastructure  
* Model configurations or prompts  
* Internal tooling choices

These remain the responsibility of Zestic AI.

## **Roles & Responsibilities (Client-Facing)** {#roles-&-responsibilities-(client-facing)}

* **Client Sponsor** — Accountable for value, risk appetite, and final gate approvals  
* **Client Subject Matter Experts (SMEs)** — Validate business reality and provide domain insight  
* **Zestic AI Account Manager** — Owns communication, gate scheduling, and decision clarity  
* **Zestic Delivery Team** — Owns execution and technical quality

Each Client Gate has **exactly one accountable client decision-maker**.

## **Communication Principles** {#communication-principles}

* No surprises: changes are surfaced early  
* Decision-driven meetings: every synchronous meeting has a purpose  
* Async by default: information updates do not require meetings

## **Escalation Rule** {#escalation-rule}

If a decision affects **value, trust, scope, or risk**, delivery pauses until the appropriate Client Gate is completed.

This protects both the client and Zestic AI.

**Client-Facing RACI**

## **Purpose** {#purpose-2}

This RACI clarifies **who is responsible, accountable, consulted, and informed** across the AI delivery lifecycle.  
It prevents ambiguity, speeds up decisions, and ensures the right people are involved at the right time.

## **Roles** {#roles}

* **Client Sponsor** – Owns business value, risk appetite, and final decisions  
* **Client SMEs** – Provide domain expertise and validate business reality  
* **Zestic Account Manager (AM)** – Owns communication, governance, and Client Gates  
* **Zestic Delivery Lead** – Owns delivery quality and execution  
* **Zestic AI / UX Team** – Designs, builds, and validates the solution

## 

## **RACI by Client Gate** {#raci-by-client-gate}

| Client Gate | Client Sponsor | Client SMEs | Zestic AM | Delivery Lead | AI / UX Team |
| ----- | :---: | :---: | :---: | :---: | :---: |
| Gate 1 – Value & Intent | **A** | C | **R** | C | C |
| Gate 2 – Reality Check | A | **R** | **R** | C | C |
| Gate 3 – Trust & Safety | **A** | C | **R** | **R** | C |
| Gate 4 – Definition of Done | **A** | C | **R** | **R** |  |

**RACI by Key Activities**

| Activity | Client Sponsor | Client SMEs | Zestic AM | Delivery Lead | AI / UX Team |
| :---- | :---- | :---- | :---- | :---- | :---- |
| Vision & value definition | **A** | C | **R** | C | C |
| Domain & scenario validation | A | **R** | **R** | C | C |
| Responsible AI decisions | **A** | C | **R** | **R** | C |
| UX behaviour approval | **A** | C | **R** | C | **R** |
| Acceptance criteria definition | **A** | C | **R** | **R** | C |
| Delivery execution | I | I | C | **A** | **R** |
| Release & operations | I | I | C | **A** | **R** |
| Performance review & iteration | **A** | C | **R** | C | C |

## **RACI Rules (Non-Negotiable)** {#raci-rules-(non-negotiable)}

* Exactly **one Accountable (A)** per decision  
* Responsibility (R) may be shared, but accountability may not  
* Client accountability is limited to **value, trust, and outcomes**  
* Zestic AI remains accountable for **delivery quality and implementation**

**Decision Log (Client-Visible)**

## **Purpose** {#purpose-3}

The Decision Log provides a **single, shared record of key decisions** made during the engagement.  
It ensures transparency, avoids re-litigation of past decisions, and creates a clear audit trail for value, risk, and scope choices.

This log is **client-visible**, lightweight, and decision-focused.

## **How the Decision Log Is Used** {#how-the-decision-log-is-used}

* Captures **decisions**, not discussions  
* Updated after each Client Gate or significant decision  
* Referenced when scope, behaviour, or risk is questioned

If a decision is not in this log, it is considered **not formally agreed**.

## **Decision Log Template (Repeat per decision)** {#decision-log-template-(repeat-per-decision)}

**Decision ID**  
DL-001

**Decision title**  
Short, clear description of what was decided

**Decision type**  
☐ Value / Scope  
☐ Risk / Ethics  
☐ UX / Behaviour  
☐ Delivery / Timing

**Related Client Gate**  
Gate 1 / Gate 2 / Gate 3 / Gate 4 / Gate 5 / Outside Gate

**Decision summary**  
What was decided, in plain language (1–3 sentences)

**Context / rationale**  
Why this decision was taken (key facts only)

**Alternatives considered**

* Option A (why not chosen)  
* Option B (why not chosen)

**Implications**

* Impact on scope  
* Impact on risk  
* Impact on timeline or cost

**Accountable decision-maker (Client)**  
Name and role

**Decision date**  
YYYY-MM-DD

**Status**  
☐ Agreed  
☐ Superseded  
☐ Under review

**Related documents**  
(e.g. Product Vision, Responsible AI Register, Acceptance Criteria)

## 

## **Governance Rules** {#governance-rules}

* One accountable client decision-maker per entry  
* No retroactive decisions  
* Changes to a decision require a **new log entry**, not an edit

## **Why This Protects Everyone** {#why-this-protects-everyone}

* Clients: clarity, confidence, no surprises  
* Zestic AI: clear mandate, reduced risk, faster delivery  
* Delivery teams: fewer interruptions and rework

# ALT. AI & System Design Brief (As-Is / To-Be)

## **5\. AI & System Design Brief (As-Is / To-Be)**

### **Purpose**

Define **how AI fits into the existing IT landscape** and how the system should evolve over time. This document is **explicitly brownfield-first** and compatible with enterprise architecture practices (e.g. TOGAF).

AI is treated as an **incremental capability overlay**, not a wholesale platform replacement.

### ***When this document is created/updated***

* *After discovery and domain understanding*  
* *Before development starts*  
* *Updated when architectural constraints or target state change*

### **Section A — As-Is Architecture (Baseline)**

Describe the current state of the IS/IT landscape. The goal is shared understanding, not exhaustive documentation.

**Applications & systems**

* Key systems and their responsibilities  
* Systems of record vs systems of engagement

**Data landscape**

* Core data sources  
* Data ownership and quality constraints

**Integration patterns**

* APIs, ESB, events, batch jobs

**Operational constraints**

* Security and compliance requirements  
* Cloud / on-prem boundaries  
* Vendor or platform lock-ins

**What cannot change (near-term)**

* …

### 

### **Section B — AI Opportunity & Constraint Overlay**

Identify where AI can **augment, automate, or assist** within the existing landscape.

**Candidate AI touchpoints**

* Decision points  
* Knowledge retrieval  
* User assistance/copilots  
* Process automation

**Explicit non-targets**

* Where AI must not be introduced

**Key risks & dependencies**

* Data readiness  
* Latency or reliability constraints  
* Regulatory considerations

### **Section C — To-Be Target Architecture**

Describe the intended future state, assuming **incremental change**.

**New or evolved components**

* AI services  
* Supporting software services

**Responsibility changes**

* What shifts from human → AI  
* What remains human-controlled

**Integration approach**

* How AI components interact with existing systems

**Architecture principles**

* Strangler / incremental migration  
* Reversibility  
* Clear ownership boundaries

### 

### **Section D — Migration & Evolution Approach**

Explain how the organisation moves from As-Is to To-Be.

**Incremental steps**

* Phase 1 (pilot/augmentation)  
* Phase 2 (scaled usage)  
* Phase 3 (optimisation)

**Decision checkpoints**

* Which Client Gates apply

**Rollback considerations**

* How AI capabilities can be reduced or disabled if needed

### **Section E — Non-Goals & Trade-Offs**

Make constraints explicit to avoid later surprises.

* What this design intentionally does not solve  
* Known compromises made to fit the current landscape

### **Section F — Open Questions & Assumptions**

* Architectural unknowns  
* Assumptions to validate early

# E2E business scenario definition Alex T.

## Definition

An end-to-end business scenario shall be a normative description of coordinated business activity that is anchored to the complete lifecycle of a single primary business object and defined by the delivery of a specified customer-visible value outcome.

An end-to-end business scenario shall describe the evolution of the primary business object from its initial state to one of its defined terminal states, including all required decisions, state transitions, and hand-offs necessary to realize that outcome, independent of organizational, system, or implementation boundaries.

## Core Characteristics

An end-to-end business scenario **shall**:

a) **Identify a primary business object**  
The scenario **shall** specify exactly one primary business object whose lifecycle provides the structural basis of the scenario.

b) **Cover the full lifecycle of the primary business object**  
The scenario **shall** define:

* an explicit entry condition corresponding to the creation or recognition of the primary business object; and

* one or more valid terminal states representing completion, termination, or retirement of that object.

c) **Be value-oriented**  
The scenario **shall** specify the customer or beneficiary and the value outcome delivered upon successful completion of the lifecycle.

d) **Define lifecycle phases as state transitions**  
The scenario **shall** describe the lifecycle of the primary business object as an ordered set of meaningful state transitions.  
 Lifecycle phases **shall not** be defined solely by organizational units, systems, or task sequences.

e) **Include decision points as first-class elements**  
Decisions that materially affect the trajectory or terminal state of the primary business object **shall** be explicitly identified.

f) **Maintain continuity across boundaries**  
The scenario **shall** remain logically continuous across functional, organizational, system, or automation boundaries.

g) **Specify scope and exclusions**  
The scenario **shall** define its boundaries by lifecycle ownership of the primary business object and **shall** explicitly state which adjacent lifecycles or activities are excluded.

## Relationship to Processes and Workflows

An end-to-end business scenario **shall not** be equivalent to a business process, workflow, or system orchestration.

Business processes, workflows, and automation flows **may** be derived from, or mapped to, an end-to-end business scenario but **shall not** replace the scenario as the authoritative description of value delivery.

### **Conformance Criteria**

A documented scenario **shall be considered an end-to-end business scenario** only if it satisfies all of the following:

1. A single primary business object is explicitly identified.

2. Entry and terminal lifecycle states are explicitly defined.

3. Customer value is explicitly stated and traceable to lifecycle completion.

4. Lifecycle phases represent object state transitions, not task lists.

5. Decisions affecting lifecycle outcomes are explicitly described.

If any of the above conditions are not met, the scenario **shall not** be classified as an end-to-end business scenario.

### **Non-conformance (Informative)**

Descriptions that focus solely on:

* execution flows,

* actor interactions,

* system behavior,

* or task sequencing,

without explicit lifecycle anchoring to a primary business object **do not conform** to this definition.

# Risk register principles

## **1\. How Many Risks Should Be Included?**

**There is no fixed “correct” number**, but in practice:

### **Typical ranges that work well**

* **8–15 risks** → *Most AI consulting engagements (recommended default)*  
* **5–8 risks** → Small, low-risk internal or exploratory use cases  
* **15–25 risks** → High-impact, regulated, or customer-facing AI systems

If your risk register has **30+ items**, it has stopped being a decision tool and become a compliance artifact.

### **Rule of thumb**

A Responsible AI & Risk Register should include **only risks that could realistically influence a go / no-go, scope, design, or mitigation decision**.

If it wouldn’t change anything even if it materialized, it doesn’t belong.

## **2\. What Is the Purpose of the Risk Register?**

Before defining inclusion rules, it helps to be explicit about purpose.

In AI consulting, a Responsible AI & Risk Register exists to:

* Surface **material risks early**  
* Enable **informed tradeoffs**  
* Guide **mitigation investment**  
* Create a **shared risk narrative** with the client  
* Support defensible decisions (not eliminate risk)

It is **not**:

* A checklist  
* A legal document  
* A guarantee of safety  
* A catalog of everything that could possibly go wrong

## **3\. Principles for Including a Risk**

A risk should be included **only if it meets at least one of the following principles**.

### **Principle 1: Material Impact**

Include the risk if it could materially affect:

* Customers or end users  
* Financial performance  
* Legal or regulatory standing  
* Brand or reputation  
* Trust in AI outputs  
* Human safety or rights

If the worst-case outcome is “minor inconvenience,” exclude it.

### 

### **Principle 2: Plausibility**

Include the risk if it is:

* Credible given the **data, model type, and context**  
* Observed in similar systems or industries  
* Supported by known AI failure modes

Exclude:

* Purely theoretical or sci-fi risks  
* Risks with no plausible pathway to occurrence

### **Principle 3: Decision Relevance**

Include the risk if it could:

* Change architecture choices  
* Influence human-in-the-loop design  
* Affect deployment scope or timing  
* Require governance or monitoring  
* Trigger escalation or kill criteria

If no one would act differently because of it, it does not belong.

### **Principle 4: Stakeholder Sensitivity**

Include the risk if:

* Executives, regulators, or customers would care deeply  
* The client has low tolerance in this area (e.g., bias, privacy)  
* There is reputational asymmetry (small failure, big backlash)

### **Principle 5: Mitigation Feasibility**

Include the risk if:

* Mitigation is possible but non-trivial  
* Tradeoffs exist (e.g., accuracy vs. explainability)  
* Residual risk must be explicitly accepted

If mitigation is trivial or already guaranteed, it can usually be omitted or folded into another risk.

## **4\. Thresholds for Inclusion (A Practical Scoring Model)**

Many AI consulting firms use a **lightweight threshold model**, even if they don’t formalize it fully.

### **Suggested threshold dimensions**

Score each risk on a simple 1–5 scale:

| Dimension | Question |
| ----- | ----- |
| Impact | How severe is the harm if it occurs? |
| Likelihood | How plausible is occurrence? |
| Detectability | How hard is it to detect before harm? |
| Sensitivity | How sensitive are stakeholders to this risk? |

### **Inclusion thresholds (example)**

* Include if:  
  * **Impact ≥ 4**, OR  
  * **(Impact × Likelihood) ≥ 9**, OR  
  * **Sensitivity ≥ 4**, even if likelihood is low

This keeps the register focused and defensible.

## **5\. Risk Categories to Ensure Coverage (Not Exhaustion)**

Instead of adding many risks, aim for **balanced coverage** across categories.

A strong Responsible AI register usually includes **1–3 risks per category**, not every possible variant.

### **Common AI Risk Categories**

* **Bias & Fairness**  
* **Explainability & Transparency**  
* **Data Quality & Representativeness**  
* **Privacy & Data Protection**  
* **Security & Model Misuse**  
* **Human Oversight & Automation Risk**  
* **Performance Degradation / Drift**  
* **Regulatory & Compliance**  
* **Operational Dependency**  
* **Reputational & Trust Risk**

If you have 10–15 risks total and these categories are represented, you’re in a good place.

---

## **6\. What to Explicitly Exclude (On Purpose)**

Good risk registers are **curated**, not exhaustive.

Actively exclude:

* Duplicate risks with slightly different wording  
* Risks fully owned by existing enterprise controls  
* Risks that are already mitigated by design and monitored  
* Generic AI risks that don’t apply to the use case

You can document these separately as:

“Known but non-material risks”

This shows diligence without bloating the register.

---

## **7\. How to Explain This to Clients (Important)**

A useful framing in consulting conversations:

“This register focuses on **material, decision-relevant Responsible AI risks**—not every theoretical AI risk. The goal is to enable informed choices and targeted mitigation, not to create an exhaustive checklist.”

This builds trust and avoids “why didn’t you list X?” debates.

---

## **8\. Summary Guidance**

**How many risks?**

* Usually **8–15**  
* Rarely more than **20**

**Include a risk if it is:**

* Material  
* Plausible  
* Decision-relevant  
* Stakeholder-sensitive  
* Non-trivial to mitigate

**Exclude risks that:**

* Don’t change decisions  
* Are implausible or trivial  
* Are already fully controlled

---

If you’d like, I can next:

* Provide a **Responsible AI & Risk Register template**  
* Show an **example filled register** for a real AI use case  
* Define **risk acceptance vs. mitigation criteria**  
* Map this to **EU AI Act, ISO 23894, NIST AI RMF**

Just tell me where you want to go next.

