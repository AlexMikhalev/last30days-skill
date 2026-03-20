#!/bin/bash
# start-factory.sh - Start terraphim-factory TMUX session
set -euo pipefail

SESSION="terraphim-factory"
FACTORY_DIR="/home/alex/clawd/terraphim-factory"
LOG_DIR="$FACTORY_DIR/logs"

mkdir -p "$LOG_DIR"
touch "$LOG_DIR/telemetry.jsonl" "$LOG_DIR/alerts.log"

# Kill existing session
tmux kill-session -t "$SESSION" 2>/dev/null || true

# Create new session with 6 panes
tmux new-session -s "$SESSION" -d -c "$FACTORY_DIR"
tmux split-window -h -t "$SESSION"
tmux split-window -h -t "$SESSION"
tmux split-window -h -t "$SESSION"
tmux split-window -h -t "$SESSION"
tmux split-window -h -t "$SESSION"
tmux select-layout -t "$SESSION" tiled

# Pane 0: Observer Manager (Phase 2)
tmux send-keys -t "$SESSION:0.0" "echo 'Observer Manager — Phase 2'" C-m

# Pane 1: Rust Engineer
tmux send-keys -t "$SESSION:0.1" "cd $FACTORY_DIR && ./scripts/agent-nightwatch.sh rust-engineer 'echo rust-engineer-ready'" C-m

# Pane 2: Svelte/TUI Engineer
tmux send-keys -t "$SESSION:0.2" "cd $FACTORY_DIR && ./scripts/agent-nightwatch.sh svelte-engineer 'echo svelte-engineer-ready'" C-m

# Pane 3: DevOps Engineer
tmux send-keys -t "$SESSION:0.3" "cd $FACTORY_DIR && ./scripts/agent-nightwatch.sh devops-engineer 'echo devops-engineer-ready'" C-m

# Pane 4: Documentation Engineer
tmux send-keys -t "$SESSION:0.4" "cd $FACTORY_DIR && ./scripts/agent-nightwatch.sh docs-engineer 'echo docs-engineer-ready'" C-m

# Pane 5: Research Agent
tmux send-keys -t "$SESSION:0.5" "cd $FACTORY_DIR && ./scripts/agent-nightwatch.sh research-agent 'echo research-agent-ready'" C-m

echo "Factory started. Attach with: tmux attach -t $SESSION"
echo "Logs: tail -f $LOG_DIR/telemetry.jsonl"
