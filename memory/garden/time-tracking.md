# Time tracking

## Goal
Track work time with minimal friction and accurate reporting.

## Primary system
- System of record: **Jibble**
- Interaction surface: **WhatsApp** commands to the assistant

## Commands (proposed)
- `Start: <project> / <activity>`
- `Stop`
- `Switch: <project> / <activity>`
- `Status`
- `Log: <duration> <when> <project> / <activity> [note]`

## Rules
- Confirm before starting a timer if another timer appears to be running.
- Always echo what will be logged (project/activity) before committing.

## Next steps
- Define your canonical project/activity list.
- Choose implementation: browser automation now, API later.
