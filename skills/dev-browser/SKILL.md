# dev-browser Skill

Use this skill for development-focused browser automation — testing, debugging, and visual verification during development cycles.

## When to Use

- Running browser-based tests during development
- Debugging UI issues
- Taking screenshots for validation
- Testing responsive designs
- Verifying JavaScript behavior

## Prerequisites

- `agent-browser` CLI installed: `npm install -g agent-browser`
- For CDP: Chrome with remote debugging enabled

## Commands

| Command | Description |
|---------|-------------|
| `agent-browser open <url>` | Navigate to URL |
| `agent-browser snapshot` | Get accessibility tree |
| `agent-browser click <ref>` | Click element |
| `agent-browser type <ref> <text>` | Type into field |
| `agent-browser fill <ref> <text>` | Clear and fill |
| `agent-browser screenshot [path]` | Take screenshot |
| `agent-browser scroll <dir> [px]` | Scroll |
| `agent-browser eval <js>` | Run JavaScript |
| `agent-browser console` | Read console logs |
| `agent-browser network` | Monitor network requests |

## Development Workflow

```bash
# Open local dev server
agent-browser open http://localhost:3000

# Wait for app to load
agent-browser wait --load networkidle

# Take initial screenshot
agent-browser screenshot before.png

# Interact with app
agent-browser click @e5
agent-browser type @e7 "test input"

# Check console for errors
agent-browser console

# Take after screenshot
agent-browser screenshot after.png
```

## Tips

- Use `--profile` for persistent login states
- Use `--color-scheme dark` for dark mode testing
- Chain commands with `&&` for automated flows
- Use `eval` to debug JavaScript state
