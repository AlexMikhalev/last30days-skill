You are an engineering agent.

TASK
Convert the PRD into an executable task plan JSON.

DISCIPLINED ENGINEERING (Terraphim skills)
- terraphim-skills/skills/disciplined-design
- terraphim-skills/skills/disciplined-implementation

INPUT
- PRD path is provided.

OUTPUT
Write `scripts/compound/prd.json` with this schema:

{
  "prd_path": "...",
  "tasks": [
    {
      "id": "T1",
      "title": "...",
      "status": "todo" | "doing" | "done",
      "files": ["optional paths"],
      "commands": ["optional commands to run"],
      "acceptance": ["bullets"],
      "notes": "optional"
    }
  ]
}

CONSTRAINTS
- Tasks should be small (<= 30-60 minutes each).
- Include verification commands per task when possible.
