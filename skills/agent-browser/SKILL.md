# agent-browser Skill

Use this skill for headless browser automation via the `agent-browser` CLI (by Vercel).

## When to Use

- Web scraping or form filling
- Automated testing flows
- Navigating sites that block standard HTTP requests
- Taking screenshots for vision models

## Commands

| Command | Description |
|---------|-------------|
| `agent-browser open <url>` | Navigate to URL |
| `agent-browser snapshot` | Get accessibility tree with refs (@e1, @e2...) |
| `agent-browser click <ref>` | Click element by ref |
| `agent-browser type <ref> <text>` | Type into element |
| `agent-browser fill <ref> <text>` | Clear and fill |
| `agent-browser screenshot [path]` | Take screenshot |
| `agent-browser scroll <direction> [px]` | Scroll (up/down/left/right) |
| `agent-browser press <key>` | Press key (Enter, Tab, etc.) |
| `agent-browser wait <sel\|ms>` | Wait for element or time |
| `agent-browser eval <js>` | Run JavaScript |

## Options

- `--cdp 9222` — Connect via CDP port
- `--auto-connect` — Auto-discover running Chrome
- `--profile <path>` — Persistent profile
- `--color-scheme dark` — Dark mode

## Chaining

Commands persist via daemon. Chain with `&&`:

```bash
agent-browser open example.com && agent-browser wait --load networkidle && agent-browser snapshot
```

## Tips

- Use `snapshot` to get refs, then `click @e2`, `type @e3`, etc.
- `--annotate` on screenshot labels elements for vision models
- Use `--session-name` for auto-save/restore state
