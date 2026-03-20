#!/bin/bash
# agent-nightwatch.sh - Per-agent watchdog wrapper with telemetry
set -euo pipefail

AGENT_NAME="${1:-unknown}"
AGENT_CMD="${2:-echo 'No command configured'}"
LOG_DIR="${LOG_DIR:-/home/alex/clawd/terraphim-factory/logs}"
MAX_FAILURES="${MAX_FAILURES:-3}"
RESTART_DELAY="${RESTART_DELAY:-5}"

mkdir -p "$LOG_DIR"

LOG_FILE="$LOG_DIR/${AGENT_NAME}.log"
TELEMETRY_FILE="$LOG_DIR/telemetry.jsonl"
ALERTS_FILE="$LOG_DIR/alerts.log"

failures=0

log() {
    echo "[$(date -Iseconds)] $*" | tee -a "$LOG_FILE"
}

emit() {
    local exit_code=$1
    local duration=$2
    local ts=$(date +%s)
    echo "{\"agent\":\"$AGENT_NAME\",\"exit\":$exit_code,\"duration\":$duration,\"ts\":$ts}" >> "$TELEMETRY_FILE"
}

log "=== Nightwatch started for $AGENT_NAME ==="
log "Command: $AGENT_CMD"

while true; do
    log "Starting agent: $AGENT_NAME"
    
    START=$(date +%s)
    $AGENT_CMD 2>&1 | tee -a "$LOG_FILE" || true
    EXIT_CODE=${PIPESTATUS[0]}
    END=$(date +%s)
    DURATION=$((END - START))
    
    emit "$EXIT_CODE" "$DURATION"
    
    if [ $EXIT_CODE -ne 0 ]; then
        failures=$((failures + 1))
        log "Agent failed with exit $EXIT_CODE (failure #$failures/$MAX_FAILURES)"
        
        if [ $failures -ge $MAX_FAILURES ]; then
            log "CRITICAL: $AGENT_NAME failed $MAX_FAILURES times — alerting"
            echo "[$(date -Iseconds)] CRITICAL: $AGENT_NAME failed $MAX_FAILURES times" >> "$ALERTS_FILE"
            failures=0
        fi
    else
        failures=0
        log "Agent completed successfully in ${DURATION}s"
    fi
    
    log "Restarting in ${RESTART_DELAY}s..."
    sleep "$RESTART_DELAY"
done
