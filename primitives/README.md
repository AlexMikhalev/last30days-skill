# Agent Autonomy Primitives (local)

This workspace uses **file-based primitives** (Markdown + YAML frontmatter) to support long-running, compound autonomy.

## Directories
- `primitives/templates/` — YAML schema-ish templates + examples
- `primitives/tasks/` — actionable work items
- `primitives/projects/` — durable project context
- `primitives/decisions/` — decisions + rationale
- `primitives/lessons/` — lessons learned (retrievable patterns)
- `primitives/people/` — people profiles (communication prefs, roles, etc.)

## Conventions
- Filenames are `kebab-case.md`.
- Every primitive has YAML frontmatter.
- Prefer linking between primitives with relative links.

## Status fields
Tasks use:
- `status: open | in-progress | blocked | done`
- `priority: critical | high | medium | low`

## Heartbeat loop
Heartbeat reads `HEARTBEAT.md` and will:
1) scan `primitives/tasks/` for `status: open|blocked|in-progress`
2) pick the next task (priority, due, age)
3) do the work (or ask for input if blocked)
4) append a note to the task's `log:` or body
5) mark done when complete

This is intentionally simple and malleable.
