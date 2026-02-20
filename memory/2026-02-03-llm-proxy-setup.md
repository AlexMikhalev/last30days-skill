# Session: 2026-02-03 22:21:22 UTC

- **Session Key**: agent:main:main
- **Session ID**: 00b21001-14b2-4219-9f28-a33f7aff353b
- **Source**: whatsapp

## Conversation Summary

assistant: Installed and running.

What I did (fastest setup):
- Installed binary: `/usr/local/bin/terraphim-llm-proxy`
- Config: `/etc/terraphim-llm-proxy/config.toml` (defaults to **OpenRouter → gemini-2.0-flash-exp** for speed; “think” routes to DeepSeek reasoner)
- systemd service: `terraphim-llm-proxy.service` (enabled + started)
- Validation: `GET http://127.0.0.1:3456/health` returns **healthy** ✅

Important:
- I generated a `PROXY_API_KEY` and created `/etc/terraphim-llm-proxy/env`.
- Because you didn’t provide upstream keys, I set placeholders so the service can start. For real LLM calls, edit:
  - `/etc/terraphim-llm-proxy/env` and set:
    - `OPENROUTER_API_KEY=...`
    - `DEEPSEEK_API_KEY=...`
  - then run: `sudo systemctl restart terraphim-llm-proxy`

Quick client wiring examples:
- Claude Code:
  - `export ANTHROPIC_BASE_URL=http://127.0.0.1:3456`
  - `export ANTHROPIC_API_KEY=$(grep PROXY_API_KEY /etc/terraphim-llm-proxy/env | cut -d= -f2-)`
- OpenAI/Codex-style:
  - `export OPENAI_API_BASE=http://127.0.0.1:3456/v1`
  - `export OPENAI_API_KEY=<PROXY_API_KEY>`
