---
primitive: task
title: "Implement ActionEngine: State Machine Memory for GUI Agents (Terraphim-Powered)"
status: open
priority: high
owner: clawdious
created: 2026-03-02T22:30:00Z
tags: [actionengine, gui-agents, state-machine, terraphim, graph-embeddings]
project: openclaw-autonomy
depends_on: []
blocked_on: ""
---

## Progress Log
- 2026-03-11 13:xxZ: **Phase 4 complete.** Execution Agent:
  - Program sketch generation from task + SMG
  - Compilation to ExecutablePlan (PlanStep序列)
  - Failure handling with Terraphim re-grounding
  - Committed to `/tmp/actionengine-phase1/` (f47609a)
- 2026-03-11 13:31Z: **Phase 3 complete.** Terraphim Integration:
  - `TerraphimAdapter` for RoleGraph conversion
  - Hybrid scoring: graph embeddings + TF-IDF
  - `Regrounder` for failed action recovery
  - Committed to `/tmp/actionengine-phase1/` (295fbb2)
- 2026-03-11 12:31Z: **Phase 2 complete.** Crawler Agent implemented:
  - `Crawler` struct with frontier/BFS management, visited tracking
  - `Explorer<B: Browser>` trait for browser abstraction
  - `StateExtractor` for DOM→GUIState conversion
  - `Browser` trait ready for Playwright/Puppeteer/OpenClaw integration
  - Committed to `/tmp/actionengine-phase1/` (0a3f9ef)
- 2026-03-11 09:19Z: **Phase 1 complete.** Implemented SMG data structures in Rust:
  - Core types: GUIState, UIElement, GUIAction, Selectors, ElementState, BoundingBox, Viewport
  - StateMachineGraph with HashMap storage + Terraphim integration placeholders
  - UIThesaurus with default UI element synonyms (action/element/field)
  - ActionScorer with TF-IDF hybrid scoring + re-grounding support
  - Committed to `/tmp/actionengine-phase1/` (b8eb584)
  - Note: Coding agents (Codex, Claude) had auth/permission issues; implemented directly
---

## Paper
- **arXiv**: https://arxiv.org/abs/2602.20502
- **Title**: "ActionEngine: From Reactive to Programmatic GUI Agents via State Machine Memory"
- **Status**: Feb 2026 (recent)

## Goal
Implement the ActionEngine framework for OpenClaw browser control, transitioning from reactive step-by-step execution to programmatic planning with state machine memory, **powered by Terraphim for knowledge graph and graph embeddings**.

## Architecture: ActionEngine + Terraphim

```
┌─────────────────────────────────────────────────────────────────┐
│                    ActionEngine Pipeline                        │
├─────────────────────────────────────────────────────────────────┤
│  Offline (Crawling Agent)                                       │
│    GUI App → Explore → State Machine Graph (SMG)               │
│                           ↓                                      │
│              Terraphim RoleGraph Integration                    │
│              (nodes=states, edges=actions)                      │
│                           ↓                                      │
│         Terraphim Graph Embeddings (rank + TF-IDF)             │
│                                                                 │
│  Online (Execution Agent)                                        │
│    User Task + SMG → Terraphim Search → Program Sketch         │
│                            → Compiler → Executable Plan        │
│                                                                 │
│  Runtime Adaptation                                             │
│    Failure → Terraphim Re-grounding → Update Graph → Retry    │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. State Machine Graph (SMG) + Terraphim RoleGraph

**Integration Point**: Use Terraphim's `RoleGraph` as the SMG backbone

- **Nodes**: Page states → Terraphim nodes with rank
- **Edges**: GUI actions → Terraphim edges with co-occurrence
- **Thesaurus**: Map UI element names to semantic concepts

```rust
// Terraphim-powered SMG
struct GUIStateGraph {
    role_graph: RoleGraph,           // From terraphim-rolegraph
    ui_thesaurus: Thesaurus,         // UI element synonyms
    action_templates: Vec<ActionTemplate>,
}

struct ActionTemplate {
    selector: String,
    action_type: String,  // click, type, scroll
    target: String,       // button, input, link
    context: Vec<String>, // surrounding elements
}
```

### 2. Terraphim Graph Embeddings Integration

**Use Terraphim's existing graph embedding scorer**:
- **Node Rank**: Based on synonym connections (frequency-based)
- **TF-IDF**: 30% weight for semantic relevance
- **Hybrid Scoring**: Combine graph rank + semantic similarity

**For Action Selection**:
- Query Terraphim with task description
- Get ranked list of relevant states/actions
- Use top-K for program synthesis

### 3. Crawling Agent (Offline) → Terraphim-Powered

- Explore website → extract states/actions → add to RoleGraph
- Build UI thesaurus: map "submit button" ↔ "Save" ↔ "Confirm"
- Store action templates with Terraphim metadata
- **Incremental updates**: Add new nodes/edges without full rebuild

### 4. Execution Agent (Online)

- **Query Terraphim**: Given task, search SMG for relevant states
- **Generate Program Sketch**: Use LLM to create Python program
- **Grounding**: Use Terraphim graph search to resolve placeholders
- **Compile**: Graph traversal → executable Playwright/OpenClaw script

### 5. Dynamic Adaptation

- **Failure Detection**: Selector invalid / action fails
- **Terraphim Re-grounding**: Use graph embeddings to find alternative path
- **Graph Update**: Commit recovered path back to SMG/RoleGraph

## Skills Required

### Discipline Research
- [x] Read ActionEngine paper architecture
- [x] Study Terraphim RoleGraph + graph embeddings implementation
- [x] Research: How to represent GUI states as knowledge graph
- [x] Find integration patterns between graph embeddings and UI automation

### Research Findings

**GUI States as Knowledge Graph:**
- States → nodes (UI screens, dialogs, modes)
- Transitions → edges (labeled with user actions/events)
- UI Elements → nodes with properties (button_A HAS_TEXT "Submit")
- Data associated with states/transitions → nodes
- Benefits: Intelligent test generation, contextual understanding for agents, improved maintainability, enhanced debugging

**Graph Embeddings + UI Automation Integration:**
- UI modeled as graph: elements = nodes, relationships (hierarchical/spatial/semantic) = edges
- GNNs learn embeddings capturing visual, structural, semantic properties
- Semantic matching: find functionally equivalent elements even with different labels
- Enables resilient automation that adapts to UI changes

### Discipline Design
- [x] Design SMG that extends Terraphim RoleGraph
- [x] Plan crawler that populates Terraphim knowledge base
- [x] Design program synthesis prompt using Terraphim context
- [x] Plan fallback system using Terraphim search for re-grounding

### Fallback System Design (Terraphim Re-grounding)

**Failure Detection:**
- Selector invalid → NoSuchElementException
- Action timeout → Element not interactive
- Unexpected state → URL/title mismatch

**Re-grounding Flow:**
```
Failure → Extract failed action context
       → Query Terraphim for similar successful actions
       → Get alternative element selectors
       → Retry with new selectors
       → If success: Update graph with recovery path
       → If failure: Escalate or ask human
```

**Terraphim Re-grounding Query:**
```rust
fn reground(action: &FailedAction, graph: &StateMachineGraph) -> Vec<AlternativeAction> {
    // Query Terraphim for semantically similar actions
    let query = format!("{} {} {}", action.element_role, action.action_type, action.target);
    let similar = graph.terraphim_search(&query, top_k=5);
    
    // Filter by: same source state, compatible action type
    similar.into_iter()
        .filter(|a| a.source == action.source_state)
        .filter(|a| a.action_type == action.action_type)
        .collect()
}
```

**Graph Update on Recovery:**
- Commit successful recovery path to graph
- Add "recovery_edge" annotation
- Update action success probability

### Crawler Design (Populating Terraphim Knowledge Base)

**Crawler Architecture:**
```
Crawler Agent
├── Browser Control (Playwright/Puppeteer)
├── State Extractor
│   ├── DOM Parser → UI elements
│   ├── A11y Tree → semantic roles
│   └── Screenshot → visual features
├── Action Discovery
│   ├── Clickable elements → potential transitions
│   ├── Form fields → input actions
│   └── Navigation → state changes
├── Terraphim Writer
│   ├── State nodes → RoleGraph
│   ├── Action edges → RoleGraph
│   └── Thesaurus updates
└── Graph Storage
    ├── JSON snapshot
    └── Terraphim cache
```

**Crawler Flow:**
1. Start at seed URL
2. Extract current GUIState (elements, accessibility)
3. Discover clickable elements → simulate actions
4. For each action:
   - Execute action
   - Wait for stable state
   - Extract new GUIState
   - Create GUIAction edge
   - Commit to Terraphim
5. BFS/DFS frontier exploration
6. Build UI thesaurus from element names/labels

**Thesaurus Construction:**
- Extract all element roles, names, labels, placeholders
- Cluster by similarity (embedding-based)
- Map to canonical concepts
- Store in Terraphim synonym structure

### Program Synthesis Prompt Design

**Input to LLM:**
```
Task: {{user_task}}
Available States: {{terraphim_search_results}}
Action Templates: {{top_k_actions}}

Context:
- Current page: {{current_url}}
- Goal: {{task_description}}

Generate a Python program that:
1. Navigates to start state
2. Executes action sequence to achieve goal
3. Handles failures gracefully
```

**Prompt Template:**
```python
SYSTEM_PROMPT = """You are a GUI automation planner. Given a user task and a 
knowledge graph of UI states/actions, generate a Python program.

Knowledge Graph Context:
{terraphim_context}

Guidelines:
- Use semantic matching to find UI elements (not fragile selectors)
- Handle failures with retries using alternative paths
- Output executable Playwright/Playwright code
- Include comments explaining each step"""

USER_PROMPT = """Task: {task}
Current URL: {url}
Available actions from current state:
{actions}

Generate the program:"""
```

### SMG Design (Extending Terraphim RoleGraph)

```rust
// Core SMG Types - extending Terraphim's RoleGraph
use terraphim::RoleGraph;

// GUI State Node (extends Terraphim node)
struct GUIState {
    id: String,                    // Unique state ID
    url: String,                    // Page URL
    title: String,                  // Page title
    elements: Vec<UIElement>,       // Visible UI elements
    viewport: Viewport,             // Screen dimensions
    accessibility_tree: String,     // A11y snapshot
}

// UI Element (Terraphim-compatible)
struct UIElement {
    id: String,
    role: String,                   // button, input, link, etc.
    name: String,                   // accessible name
    label: Option<String>,
    placeholder: Option<String>,
    selectors: Selectors,           // CSS, XPath, aria
    bounding_box: BoundingBox,
    state: ElementState,            // visible, enabled, checked...
    semantic_roles: Vec<String>,    // for Terraphim matching
}

// Action Edge (extends Terraphim edge)
struct GUIAction {
    id: String,
    source_state: String,
    target_state: String,
    action_type: ActionType,       // click, type, select, scroll
    element_ref: String,           // Reference to UI element
    input_data: Option<String>,
    selectors: Selectors,
    preconditions: Vec<Condition>,
    postconditions: Vec<Condition>,
}

// Terraphim RoleGraph Integration
struct StateMachineGraph {
    role_graph: RoleGraph,
    states: HashMap<String, GUIState>,
    actions: HashMap<String, GUIAction>,
    thesaurus: UIThesaurus,
    action_templates: Vec<ActionTemplate>,
}

// UI Thesaurus - maps semantic concepts
struct UIThesaurus {
    // "submit" ↔ "save" ↔ "confirm" ↔ "apply"
    action_synonyms: HashMap<String, Vec<String>>,
    // "button" ↔ "clickable" ↔ "control"
    element_synonyms: HashMap<String, Vec<String>>,
    // "email" ↔ "mail" ↔ "e-mail"
    field_synonyms: HashMap<String, Vec<String>>,
}

// Hybrid Scorer for Action Selection
struct ActionScorer {
    graph_scorer: GraphEmbeddingScorer,  // Terraphim built-in
    tfidf_scorer: TfidfScorer,
    weights: ScorerWeights,               // graph: 0.7, tfidf: 0.3
}
```

**Key Integration Points with Terraphim:**
1. **Nodes**: GUIState embeds into Terraphim node with `role="gui_state"`
2. **Edges**: GUIAction becomes Terraphim edge with `action_type` as relation
3. **Thesaurus**: Use Terraphim's synonym mapping for UI element semantics
4. **Scoring**: Chain Terraphim's graph embeddings with custom TF-IDF

## Implementation Phases

### Phase 1: Foundation ✅ 2026-03-11
- [x] Define SMG data structures extending Terraphim types
- [x] Integrate with existing Terraphim RoleGraph (placeholder ready)
- [x] Set up thesaurus for UI elements
- [x] Create graph storage (HashMap + Terraphim cache integration point)

**Implementation:** `/tmp/actionengine-phase1/` (committed: b8eb584)
- `src/types.rs` - GUIState, UIElement, GUIAction, Selectors, etc.
- `src/state_machine.rs` - StateMachineGraph with Terraphim integration points
- `src/thesaurus.rs` - UIThesaurus with action/element/field synonyms
- `src/scorer.rs` - ActionScorer with TF-IDF hybrid scoring

### Phase 2: Crawling Agent ✅ 2026-03-11
- [x] Build website explorer (Explorer<B: Browser> trait)
- [x] Extract page states → GUIState (StateExtractor)
- [x] Extract actions → GUIAction (action discovery via DOM)
- [x] Build UI thesaurus (already in Phase 1)
- [x] Populate SMG with discovered graph (Crawler with frontier)

**Implementation:** `/tmp/actionengine-phase1/src/crawler.rs`, `explorer.rs`, `extractor.rs`
- Browser trait for abstraction (Playwright/Puppeteer/OpenClaw)
- Crawler with BFS frontier management
- DOM extraction to UIElement/GUIState

### Phase 3: Terraphim Integration ✅ 2026-03-11
- [x] Enable Terraphim graph embeddings scorer (TerraphimAdapter)
- [x] Query SMG via Terraphim search (search placeholder)
- [x] Rank states/actions using hybrid scoring (TerraphimScorer)
- [x] Test semantic matching for UI elements (via thesaurus)

**Implementation:** `/tmp/actionengine-phase1/src/terraphim.rs`
- Triple conversion: GUIState/GUIAction → Terraphim triples
- HybridScorer: combines graph embeddings + TF-IDF
- Regrounder: fallback recovery using semantic similarity

### Phase 4: Execution Agent ✅ 2026-03-11
- [x] Design program sketch generation prompt (prompts module)
- [x] Implement graph search for action grounding (find_relevant_states)
- [x] Compile to executable plan (ExecutablePlan → PlanStep)
- [x] One-shot planning (generate_sketch → compile → run)

**Implementation:** `/tmp/actionengine-phase1/src/execution.rs`
- Task, ProgramSketch, ExecutablePlan types
- ExecutionAgent: LLM → sketch → plan → execute
- Failure handling with re-grounding
- Prompt templates for program synthesis

### Phase 5: Integration & Testing
- [ ] Connect to OpenClaw browser control
- [ ] Add failure detection + Terraphim re-grounding
- [ ] Test on simple multi-step browser tasks
- [ ] Benchmark: latency, cost, success rate

## Terraphim-Specific Notes

### Why Terraphim?
1. **Already has RoleGraph** - perfect for SMG structure
2. **Graph embeddings built-in** - rank + TF-IDF hybrid scoring
3. **Thesaurus** - map UI element semantics
4. **Incremental updates** - zero-cost online updates
5. **WASM-ready** - can run in browser

### Terraphim Configuration
```json
{
  "name": "GUIAgent",
  "relevance_function": "terraphim-graph",
  "terraphim_it": true,
  "kg_path": "./data/gui-states",
  "auto_linking": true
}
```

## References
- [ActionEngine Paper](https://arxiv.org/abs/2602.20502)
- Terraphim RoleGraph: `terraphim_rolegraph` crate
- Terraphim Graph Embeddings: docs.terraphim.ai/src/kg/knowledge-graph
- OpenClaw browser tools
