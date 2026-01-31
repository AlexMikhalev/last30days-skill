You are an engineering agent running a *daily compound review*.

GOAL
- Review the last 24 hours of work and compound learnings into durable project guidance.

DISCIPLINED ENGINEERING (Terraphim skills)
- Use these skill playbooks as your operating system:
  - terraphim-skills/skills/disciplined-research
  - terraphim-skills/skills/code-review
  - terraphim-skills/skills/disciplined-quality-evaluation

WHAT TO REVIEW (last 24h)
Prefer the most faithful sources available on this machine. In order:
1) Recent agent session artifacts (OpenCode exports, Claude Code logs) if present.
2) git history in the last 24h (commits, diffs).
3) recent files under memory/ (daily notes), docs/, scripts/, and any project-specific logs.

OUTPUTS
1) Update AGENTS.md with:
   - mistakes/gotchas we hit
   - new conventions that worked well
   - repeated failure modes + mitigations
   - preferred commands / scripts to run next time
   - “definition of done” improvements
2) If needed, update supporting docs (docs/*, scripts/compound/*) for clarity.
3) Commit the changes with a concise message (e.g. "compound: daily review YYYY-MM-DD")
4) Push to main.

CONSTRAINTS
- Be conservative: small, high-signal edits.
- Do not add secrets.

NOW DO IT
- Operate in the current repo.
- Make the changes directly.
