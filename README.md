# MAX — Production CLI AI Agent

```
  ███╗   ███╗ █████╗ ██╗  ██╗
  ████╗ ████║██╔══██╗╚██╗██╔╝
  ██╔████╔██║███████║ ╚███╔╝
  ██║╚██╔╝██║██╔══██║ ██╔██╗
  ██║ ╚═╝ ██║██║  ██║██╔╝ ██╗
  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝
```

**Created by Ememzyvisuals (Emmanuel Ariyo)**

[![GitHub](https://img.shields.io/badge/GitHub-ememzyvisuals-blue?logo=github)](https://github.com/ememzyvisuals)
[![npm](https://img.shields.io/npm/v/@ememzyvisuals/max)](https://npmjs.com/package/@ememzyvisuals/max)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.72+-orange?logo=rust)](https://www.rust-lang.org)

> A high-end, developer-first CLI AI agent with multi-agent orchestration, offline LLM support, persistent memory, and an interactive AI companion — built in Rust for maximum performance.

---

## Features

- **Multi-Agent Orchestration** — ULTRAPLAN breaks any task into parallel agent workstreams and synthesizes a master plan
- **Offline LLM Inference** — Runs Mistral 7B, LLaMA 3, Code LLaMA, StarCoder via Ollama — no internet required
- **API Integration** — Groq (ultra-fast), OpenAI GPT-4 Turbo, Anthropic Claude 3.5 Sonnet, with automatic fallback
- **Persistent Memory** — SQLite-backed session memory with conversation recall
- **/buddy Companion** — Interactive AI pet with moods, XP, evolution stages (Sprite → Legend)
- **Code Generation** — AI-powered code gen in any language, with syntax highlighting and file output
- **Sandbox Execution** — Safe execution of Python, JS, Bash, Go, Ruby and more with timeout protection
- **Premium Terminal UX** — Animated spinners, typewriter effects, colorized output, splash screen

---

## Installation

### npm (Recommended — auto-downloads binary)

```bash
npm install -g @ememzyvisuals/max
max --version
```

### GitHub Clone (Build from source)

```bash
git clone https://github.com/ememzyvisuals/max
cd max
cargo build --release
./target/release/max --version

# Install globally
sudo cp target/release/max /usr/local/bin/max
```

### Manual Binary Download

Download prebuilt binaries from [GitHub Releases](https://github.com/ememzyvisuals/max/releases):

| Platform       | File                          |
|----------------|-------------------------------|
| Linux x86_64   | `max-linux-x86_64.tar.gz`    |
| Linux ARM64    | `max-linux-arm64.tar.gz`     |
| macOS Intel    | `max-macos-x86_64.tar.gz`    |
| macOS Silicon  | `max-macos-arm64.tar.gz`     |
| Windows x86_64 | `max-windows-x86_64.zip`     |

```bash
# Linux / macOS
tar -xzf max-linux-x86_64.tar.gz
chmod +x max-linux-x86_64
sudo mv max-linux-x86_64 /usr/local/bin/max
```

---

## Quick Start

```bash
# Interactive REPL (recommended)
max

# Single chat message
max chat "Explain async Rust in 3 sentences"

# Multi-agent task planning
max plan "Build a SaaS analytics dashboard"

# Generate code
max code "REST API with JWT auth" --lang rust --output api.rs

# Run a file in the sandbox
max run script.py

# Interact with your buddy
max buddy status
max buddy feed
max buddy play

# View available models
max models list
```

---

## Configuration

MAX automatically creates `~/.max/config.toml` on first run.

### API Keys (set via environment variables)

```bash
export GROQ_API_KEY=your_groq_key          # Fast, free tier available
export OPENAI_API_KEY=your_openai_key
export ANTHROPIC_API_KEY=your_anthropic_key
```

### Offline Mode with Ollama

```bash
# Install Ollama: https://ollama.ai
ollama pull llama3         # General purpose
ollama pull codellama      # Code generation
ollama pull mistral        # Fast inference
ollama pull starcoder2     # Code completion

# Set in ~/.max/config.toml
# model.active = "ollama/llama3"
```

### Config Options

```toml
[model]
active = "groq/llama3-70b-8192"   # Active model
temperature = 0.7
max_tokens = 2048

[ui]
animation_speed = "normal"         # slow | normal | fast | off
color_theme = "dark"               # dark | neon | minimal

[buddy]
enabled = true
name = "CHIP"
personality = "curious"

[memory]
enabled = true
depth = 50                         # Conversations to remember
```

---

## Commands

```
max                              Interactive REPL
max chat [message]               Chat with MAX
max plan <task> [--agents N]     ULTRAPLAN multi-agent orchestration
max code <prompt> [--lang L]     Generate code
max run <file>                   Execute file in sandbox
max buddy <action>               /buddy companion (status|feed|play|evolve|reset)
max memory <action>              Memory management (list|search|clear)
max models <action>              Model management (list|download|set|info)
max config <action>              Configuration (show|reset)
```

### REPL Slash Commands

```
/help              Show commands
/exit              Exit MAX
/clear             Clear terminal
/history           Show conversation history
/memory            Show recent memories
/buddy             Check your companion
/plan <task>       Run ULTRAPLAN
/code <prompt>     Generate code
```

---

## Models

### API Models (Recommended)

| Model ID                        | Provider   | Notes                  |
|---------------------------------|------------|------------------------|
| `groq/llama3-70b-8192`         | Groq       | Best quality, very fast |
| `groq/llama3-8b-8192`          | Groq       | Fast, lightweight      |
| `groq/mixtral-8x7b-32768`      | Groq       | Long context           |
| `openai/gpt-4-turbo`           | OpenAI     | Premium                |
| `anthropic/claude-3-5-sonnet`  | Anthropic  | Excellent reasoning    |

### Offline Models (via Ollama)

| Model ID              | Notes                    |
|-----------------------|--------------------------|
| `ollama/llama3`       | General purpose, offline |
| `ollama/mistral`      | Fast, offline            |
| `ollama/codellama`    | Code generation, offline |
| `ollama/starcoder2`   | Code completion, offline |

---

## Architecture

```
max/
├── src/
│   ├── main.rs           — Entry point & CLI loop
│   ├── cli/              — Command parser & dispatcher
│   │   ├── mod.rs        — Clap definitions
│   │   ├── repl.rs       — Interactive REPL
│   │   └── commands/     — Individual command handlers
│   ├── agent/            — Multi-agent orchestration
│   │   └── orchestrator.rs  — ULTRAPLAN engine
│   ├── model/            — LLM router (offline + APIs)
│   │   └── mod.rs        — Groq / OpenAI / Anthropic / Ollama
│   ├── memory/           — Persistent memory (SQLite)
│   ├── buddy/            — /buddy AI companion system
│   ├── tools/            — Sandbox execution layer
│   ├── ui/               — Terminal UX (spinner, splash, animations)
│   └── config/           — Settings manager (TOML)
├── npm-package/          — npm distribution wrapper
│   ├── package.json
│   ├── bin/max.js        — Entry point
│   └── scripts/install.js  — OS-aware binary installer
└── .github/workflows/    — CI/CD pipeline
    └── release.yml       — Build, test, release, npm publish
```

---

## Development

```bash
# Run in dev mode
cargo run

# Run with arguments
cargo run -- chat "Hello, MAX"
cargo run -- plan "Build an e-commerce platform"

# Run tests
cargo test

# Build optimized release
cargo build --release

# Check for issues
cargo clippy
```

---

## CI/CD Pipeline

MAX uses GitHub Actions for full automation:

1. **On push to `main`** — Run tests + Clippy linting
2. **On version tag (`v*`)** — Build binaries for all 5 platforms, create GitHub Release, publish to npm

To trigger a release:
```bash
git tag v1.0.0
git push origin v1.0.0
```

Required secrets in your GitHub repo:
- `NPM_TOKEN` — npm access token for publishing

---

## License

MIT © [Ememzyvisuals (Emmanuel Ariyo)](https://github.com/ememzyvisuals)

---

## Links

- **GitHub**: [github.com/ememzyvisuals](https://github.com/ememzyvisuals)
- **X (Twitter)**: [@ememzyvisuals](https://x.com/ememzyvisuals)
- **Kaggle**: [kaggle.com/ememzyvisuals](https://kaggle.com/ememzyvisuals)
- **npm**: [@ememzyvisuals/max](https://npmjs.com/package/@ememzyvisuals/max)
