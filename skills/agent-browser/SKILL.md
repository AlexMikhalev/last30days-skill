# agent-browser Skill

Use this skill for headless browser automation via the `agent-browser` CLI (by Vercel).

## Setup

agent-browser uses Playwright under the hood. On first run it will prompt to install browsers via `npx playwright install`.

**Chrome binary issue:** The bundled Playwright may not find the right browser. Use `--executable-path`:
```bash
agent-browser close  # kill existing daemon first
agent-browser --executable-path "/home/alex/.cache/ms-playwright/chromium_headless_shell-1223/chrome-headless-shell-linux64/chrome-headless-shell" open <url>
```

Or set persistently via config (`~/.config/agent-browser/config.json`):
```json
{
  "browser": {
    "executablePath": "/home/alex/.cache/ms-playwright/chromium_headless_shell-1223/chrome-headless-shell-linux64/chrome-headless-shell"
  }
}
```

Or via environment variable:
```bash
export AGENT_BROWSER_EXECUTABLE_PATH="/home/alex/.cache/ms-playwright/chromium_headless_shell-1223/chrome-headless-shell-linux64/chrome-headless-shell"
```

**Important:** The `--executable-path` flag is ignored if the daemon is already running. Run `agent-browser close` first, then restart with new options.

## When to Use

- Web scraping or form filling
- Automated testing flows
- Navigating sites that block standard HTTP requests
- Taking screenshots for vision models
- JavaScript extraction from live pages (via `eval`)

## Commands

| Command | Description |
|---------|-------------|
| `agent-browser open <url>` | Navigate to URL |
| `agent-browser snapshot` | Get accessibility tree with refs (e1, e2...) |
| `agent-browser click <ref>` | Click element by ref |
| `agent-browser type <ref> <text>` | Type into element |
| `agent-browser fill <ref> <text>` | Clear and fill |
| `agent-browser screenshot [path]` | Take screenshot (saved to config screenshotDir by default) |
| `agent-browser scroll <dir> [px]` | Scroll (up/down/left/right) |
| `agent-browser press <key>` | Press key (Enter, Tab, etc.) |
| `agent-browser wait <sel\|ms>` | Wait for element or time |
| `agent-browser eval <js>` | Run JavaScript — return value is printed to stdout |
| `agent-browser close` | Close browser/daemon |
| `agent-browser install` | Download Chromium browser |

## Chaining

Commands persist via daemon. Chain with `&&`:

```bash
agent-browser open example.com && agent-browser wait --load networkidle && agent-browser snapshot
```

## Options

- `--executable-path <path>` — Custom browser executable (set in config for persistence)
- `--cdp <port>` — Connect via CDP port (alternative to built-in browser)
- `--auto-connect` — Auto-discover and connect to running Chrome
- `--allow-file-access` — Allow file:// URLs
- `--color-scheme dark` — Dark mode

## Useful Patterns

**Extract page content with JavaScript:**
```bash
agent-browser eval "document.querySelector('.markdown-body').innerText.substring(0, 500)"
```

**Click a link by ref:**
```bash
agent-browser snapshot  # get refs
agent-browser click e5  # click element with ref=e5
```

**Wait for page load then screenshot:**
```bash
agent-browser open <url> && agent-browser wait --load networkidle && agent-browser screenshot
```

## What it can do that built-in browser can't

- Download files via click
- Save pages as PDF
- Keyboard type without needing a selector (`keyboard type "text"`)
- Full JavaScript evaluation and extraction from live DOM
- `--session-name` for session save/restore

## Screenshot output

Screenshots saved to: `/home/alex/clawd/screenshots/`
