# HEARTBEAT.md

# Agent loop (lightweight): autonomy primitives
# On heartbeat:
# 1) Scan primitives/tasks for open/in-progress/blocked
# 2) Pick next task (priority > due > oldest)
# 3) Execute if possible; if blocked, ask alex a crisp question
# 4) Log progress to the task file; mark done when complete
#
# Task roots:
# - /home/alex/clawd/primitives/tasks
# - /home/alex/clawd/tasks (legacy; optional)

- Triage: list top 5 open tasks from `primitives/tasks/` with priority/due and one-line next action.
- If any task is actionable without external messaging, start on the top one and record progress.
- If all tasks are blocked, ask me the single highest-leverage unblock question.

