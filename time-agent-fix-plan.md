# time-agent Bug Fix Plan

## Bugs
| # | Issue | Severity |
|---|-------|----------|
| #13 | Project creation button non-functional | CRITICAL |
| #14 | Task dropdown shows "No projects" | CRITICAL |
| #15 | Time entries show "No project" | HIGH |
| #16 | CLI attribution not working | HIGH |

## Priority Order

### Phase 1: Fix #13 (Project Creation) — CRITICAL
1. Investigate frontend: how is project creation triggered?
2. Check if API endpoint exists
3. Fix button handler or create modal
4. Test project creation

### Phase 2: Fix #14 (Task Dropdown) — CRITICAL
1. Depends on #13 (needs projects to exist)
2. Verify task creation API
3. Fix dropdown to show created projects
4. Test task creation under projects

### Phase 3: Fix #15 (Entry Attribution) — HIGH
1. Ensure entries can link to tasks
2. Display task/project name on entries
3. Test attribution in UI

### Phase 4: Fix #16 (CLI Attribution) — HIGH
1. Test CLI project/task creation
2. Verify time entries link properly
3. Document CLI usage

## Suggested Code Areas
- Frontend: Project/Task components
- Backend: API endpoints for CRUD
- Database: Schema for project/task relationships

## Agents
| Task | Agent |
|------|-------|
| Code investigation | @senior-architect |
| Frontend fix | @dev-browser (testing) |
| Backend fix | Implement via Codex |
| Validation | @uat-validator |
