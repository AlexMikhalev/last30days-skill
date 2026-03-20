# Terraphim AI Self-Learning Agent System

## Executive Summary

Build a **self-learning, autonomous agent swarm** for terraphim-ai repository deployable on bigbox with tmux. Each agent has a **nightwatch pattern** (self-monitoring + external observer), and a central **Observer/Manager agent** keeps the swarm running, corrects drift, and handles failures.

Inspired by **AI Dark Factory** principles: continuous operation, self-correction, minimal human intervention.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     NIGHTWATCH PATTERN (per agent)              │
│  ┌─────────────┐    ┌──────────────┐    ┌───────────────────┐  │
│  │   AGENT     │◄──►│  SELF-MONITOR│◄──►│  TELEMETRY EMITTER│  │
│  │ (Codex/     │    │  (logs,      │    │  (to Observer     │  │
│  │  Claude/    │    │   metrics)    │    │   Agent)          │  │
│  │  OpenCode)  │    │              │    │                   │  │
│  └─────────────┘    └──────────────┘    └───────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▲
                              │ Telemetry
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    OBSERVER/MANAGER AGENT                        │
│  ┌─────────────┐    ┌──────────────┐    ┌───────────────────┐  │
│  │  HEALTH     │    │   DRIFT      │    │   CORRECTION      │  │
│  │  MONITOR    │───►│   DETECTOR   │───►│   ENGINE          │  │
│  │  (checks    │    │  (behavior  │    │  (restart/kill/   │  │
│  │   pings)    │    │   anomalies) │    │   retrain)        │  │
│  └─────────────┘    └──────────────┘    └───────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              KNOWLEDGE BASE (Terraphim KG)                │  │
│  │    - Agent performance history                            │  │
│  │    - Learned patterns                                    │  │
│  │    - Corrections applied                                  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Agent Types & Roles

| Agent | Primary Tool | Specialty | Nightwatch Focus |
|-------|-------------|-----------|------------------|
| **Rust Engineer** | Codex/Claude | Rust code, crates, APIs | Cargo build success, clippy, tests |
| **Svelte/TUI Engineer** | Claude | UI, frontend | Build success, svelte-check |
| **DevOps Engineer** | OpenCode | CI/CD, Docker, configs | CI pass rate, container health |
| **Documentation Engineer** | OpenCode | Docs, README, API | Link checking, completeness |
| **Research Agent** | Claude | Investigation, patterns | Quality of findings |
| **Observer Manager** | Native (Rust/Shell) | Orchestration, health | All agents alive, behavior correct |

---

## Phase 1: Infrastructure (Week 1)

### 1.1 TMUX Session Structure

```bash
# Main session: terraphim-factory
tmux new-session -s terraphim-factory -d

# Agent panes (horizontal split)
tmux split-window -h -t terraphim-factory
tmux split-window -h -t terraphim-factory  
tmux split-window -h -t terraphim-factory
tmux split-window -h -t terraphim-factory
tmux split-window -h -t terraphim-factory

# Layout: 2x3 grid
tmux select-layout -t terraphim-factory tiled
```

**Pane Mapping:**
- Pane 0: Observer Manager (always visible)
- Pane 1: Rust Engineer
- Pane 2: Svelte/TUI Engineer
- Pane 3: DevOps Engineer
- Pane 4: Documentation Engineer
- Pane 5: Research Agent

### 1.2 Self-Monitoring per Agent

Each agent runs with a **watchdog wrapper**:

```bash
#!/bin/bash
# agent-nightwatch.sh
AGENT_NAME=$1
AGENT_CMD=$2
MAX_FAILURES=3
LOG_FILE="/home/alex/clawd/terraphim-ai/logs/${AGENT_NAME}.log"

while true; do
    START=$(date +%s)
    $AGENT_CMD 2>> $LOG_FILE
    EXIT_CODE=$?
    END=$(date +%s)
    DURATION=$((END - START))
    
    # Emit telemetry
    echo "{\"agent\":\"$AGENT_NAME\",\"exit\":$EXIT_CODE,\"duration\":$DURATION,\"ts\":$END}" \
        >> /home/alex/clawd/terraphim-ai/logs/telemetry.jsonl
    
    if [ $EXIT_CODE -ne 0 ]; then
        FAILURES=$((FAILURES + 1))
        if [ $FAILURES -ge $MAX_FAILURES ]; then
            echo "CRITICAL: $AGENT_NAME failed $MAX_FAILURES times" \
                >> /home/alex/clawd/terraphim-ai/logs/alerts.log
            # Signal observer
            pkill -f "observer-manager" || true
        fi
    else
        FAILURES=0
    fi
    
    sleep 5
done
```

### 1.3 Telemetry Collection

- **Location:** `/home/alex/clawd/terraphim-ai/logs/telemetry.jsonl`
- **Format:** JSON Lines with agent, exit_code, duration, timestamp
- **Metrics tracked:**
  - Execution count
  - Success/failure rate
  - Average duration
  - Last run timestamp

---

## Phase 2: Observer Manager Agent (Week 2)

### 2.1 Core Responsibilities

1. **Health Monitoring** — Ping each agent pane, check process alive
2. **Drift Detection** — Compare agent behavior against baseline
3. **Correction** — Restart failed agents, adjust parameters
4. **Learning** — Log corrections to Terraphim KG for future reference

### 2.2 Observer Implementation (Rust)

```rust
// Observer Agent - watches other agents and corrects behavior
struct ObserverAgent {
    telemetry_path: PathBuf,
    knowledge_graph: TerraphimClient,
    alert_threshold: f64, // e.g., 0.3 = 30% failure rate
}

impl ObserverAgent {
    fn check_health(&self) -> Vec<Alert> {
        // Read telemetry, check for stale agents
        // Return list of agents needing attention
    }
    
    fn detect_drift(&self, agent_id: &str) -> DriftReport {
        // Compare current behavior vs baseline
        // - Are success rates dropping?
        // - Is duration increasing?
        // - Are error patterns changing?
    }
    
    fn correct(&self, agent_id: &str, correction: Correction) -> Result<()> {
        // Options:
        // - Restart agent process
        // - Clear agent state
        // - Adjust max failures threshold
        // - Notify human if unrecoverable
    }
    
    fn learn(&self, correction: &Correction) {
        // Write to Terraphim KG:
        // "Agent X failed because Y. Correction Z was applied."
    }
}
```

### 2.3 Drift Detection Rules

| Metric | Threshold | Action |
|--------|-----------|--------|
| Failure rate | > 20% in 1 hour | Restart agent |
| Avg duration | > 2x baseline | Cancel current task, restart |
| No telemetry | > 5 min | Force restart |
| Same error | 3x repeat | Log to KG, try alternative |

---

## Phase 3: Self-Learning Loop (Week 3)

### 3.1 Learning Pipeline

```
Agent Failure → Observer Detects → Correction Applied → Log to KG
                                                            ▼
                                              ┌─────────────────────┐
                                              │ Terraphim Knowledge │
                                              │ Graph                │
                                              │ - Error patterns    │
                                              │ - Successful fixes   │
                                              │ - Agent preferences │
                                              └─────────────────────┘
                                                            ▼
                                              Next Agent Run →查询KG → Apply learned fix
```

### 3.2 KG Schema for Agent Memory

```json
{
  "agent_memory": {
    "agent_type": "rust_engineer",
    "error_patterns": [
      {
        "pattern": "clippy warnings",
        "frequency": 15,
        "successful_fixes": [
          "run cargo clippy --fix",
          "update dependencies"
        ]
      }
    ],
    "behavioral_baseline": {
      "avg_duration_seconds": 120,
      "success_rate": 0.92,
      "common_tasks": ["fix-bug", "add-feature", "refactor"]
    }
  }
}
```

### 3.3 Observer Queries KG Before Corrections

```rust
// Before applying correction, check if similar issue was seen before
let similar_issues = kg.query(&format!(
    "agent:{} error:{} past_fixes",
    agent_type, error_category
)).await?;

if !similar_issues.is_empty() {
    // Apply known fix from history
    apply_fix(similar_issues[0].fix);
} else {
    // Try default correction, log result for future
    apply_default_correction();
    kg.insert(LearnedPattern {
        agent: agent_type,
        error: error_category,
        fix_applied: default_fix,
        success: true, // will be updated after verification
    });
}
```

---

## Phase 4: Deployment on Bigbox (Week 4)

### 4.1 Startup Script

```bash
#!/bin/bash
# start-factory.sh

SESSION="terraphim-factory"
LOG_DIR="/home/alex/clawd/terraphim-ai/logs"

# Kill existing
tmux kill-session -t $SESSION 2>/dev/null || true

# Create session
tmux new-session -s $SESSION -d -c /home/alex/clawd/terraphim-ai

# Start Observer Manager (Pane 0)
tmux send-keys -t $SESSION:0.0 "cd /home/alex/clawd/terraphim-ai && cargo run --bin observer-manager" C-m

sleep 2

# Start Agent Wrappers (Panes 1-5)
for AGENT in rust-engineer svelte-engineer devops-engineer docs-engineer research-agent; do
    tmux send-keys -t $SESSION:0.1 "cd /home/alex/clawd/terraphim-ai && ./scripts/agent-nightwatch.sh $AGENT" C-m
done

# Attach for monitoring
tmux attach -t $SESSION
```

### 4.2 Agent Configuration

Each agent needs a config file:

```toml
# .agents/rust-engineer.toml
name = "rust-engineer"
tool = "codex"  # or "claude", "opencode"
model = "claude-sonnet-4-20250514"
working_dir = "/home/alex/clawd/terraphim-ai"

[nightwatch]
max_failures = 3
restart_delay_seconds = 10
telemetry_interval_seconds = 30

[baseline]
avg_duration_seconds = 120
success_rate = 0.90
```

### 4.3 Environment Variables

```bash
export TERRAPHIM_KG_URL="http://localhost:2288"
export TERRAPHIM_KG_TOKEN="your-token"
export OBSERVER_LOG_LEVEL="debug"
export TELEMETRY_DIR="/home/alex/clawd/terraphim-ai/logs"
```

---

## Phase 5: Nightwatch Pattern Details

### 5.1 Per-Agent Self-Monitoring

Each agent wrapper includes:

1. **Health Check** — Is the process alive?
2. **Output Capture** — stdout/stderr to log file
3. **Metric Emission** — Success/failure/duration to telemetry
4. **Signal Handler** — Graceful shutdown on SIGTERM

### 5.2 Observer Pattern Implementation

```bash
# Observer polls telemetry in a loop
while true; do
    # Check each agent's last heartbeat
    for AGENT in rust-engineer svelte-engineer ...; do
        LAST=$(tail -1 telemetry.jsonl | jq -r ".ts // 0")
        NOW=$(date +%s)
        if (( NOW - LAST > 300 )); then
            # Agent silent > 5 min, restart
            tmux send-keys -t $SESSION:0.$PANE "C-c" 
            tmux send-keys -t $SESSION:0.$PANE "./start-agent.sh $AGENT" C-m
        fi
    done
    sleep 30
done
```

---

## Implementation Checklist

- [ ] **Phase 1:** TMUX session setup + basic agent wrappers
- [ ] **Phase 1:** Telemetry collection to JSONL
- [ ] **Phase 2:** Observer Manager binary (Rust)
- [ ] **Phase 2:** Health monitoring loop
- [ ] **Phase 2:** Drift detection (baseline comparison)
- [ ] **Phase 3:** Terraphim KG schema for agent memory
- [ ] **Phase 3:** Learning loop (log corrections → query before fix)
- [ ] **Phase 4:** bigbox deployment scripts
- [ ] **Phase 4:** Agent config files
- [ ] **Phase 5:** Nightwatch wrapper for each agent type
- [ ] **Phase 5:** Alerting (Slack/WhatsApp on critical failure)

---

## Key Files to Create

| Path | Purpose |
|------|---------|
| `scripts/agent-nightwatch.sh` | Per-agent watchdog wrapper |
| `scripts/start-factory.sh` | Factory startup script |
| `crates/observer-agent/` | Observer Manager Rust crate |
| `.agents/config/*.toml` | Per-agent configuration |
| `logs/telemetry.jsonl` | Telemetry log file |
| `logs/alerts.log` | Critical alerts |

---

## Success Metrics

- **Uptime:** > 99% (agents auto-restart on failure)
- **Self-correction:** Observer fixes > 80% of failures without human help
- **Learning:** New agent failures decrease over time as KG grows
- **Latency:** Observer detects issues within 60 seconds
