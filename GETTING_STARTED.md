# Getting Started with Local Agent Core

This guide will help you get up and running with the Local Agent Core project quickly.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Using the Makefile](#using-the-makefile)
- [Setting up Local LLM](#setting-up-local-llm-ollama)
- [Running the Demo](#running-the-demo)
- [Understanding the Architecture](#understanding-the-architecture)
- [Next Steps](#next-steps)

## Prerequisites

Before you begin, ensure you have the following installed:

### Required
- **Rust** (1.70 or later): Install from [rustup.rs](https://rustup.rs)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Go** (1.21 or later): Install from [go.dev](https://go.dev/dl/)

### Optional
- **Docker & Docker Compose**: For containerized deployment
- **Ollama**: For local LLM support (recommended)

## Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/vinod901/local-agent-core.git
   cd local-agent-core
   ```

2. **Install dependencies**
   ```bash
   make deps
   ```

3. **Build the project**
   ```bash
   make build
   ```

## Quick Start

The fastest way to see the agent in action:

```bash
make run
```

This will:
1. Build the Rust agent core (if not already built)
2. Run the complete workflow demo
3. Show wake word detection, LLM interaction, intent generation, and policy enforcement

## Using the Makefile

The project includes a comprehensive Makefile for easy development. Here are the most useful commands:

### See all available commands
```bash
make help
```

### Building
```bash
make build          # Build all components
make build-rust     # Build only Rust agent core
make build-go       # Build only Go device agent
```

### Testing
```bash
make test           # Run all tests
make test-rust      # Run only Rust tests
make test-go        # Run only Go tests
```

### Running
```bash
make run            # Run the complete demo
make run-demo       # Same as make run
make run-rust       # Run Rust examples
make run-go         # Run Go device agent
```

### Maintenance
```bash
make clean          # Clean all build artifacts
make fmt            # Format all code
make lint           # Run linters
make check          # Format + lint + test
```

### Docker
```bash
make docker-build   # Build Docker containers
make docker-up      # Start containers
make docker-down    # Stop containers
make docker-logs    # View logs
```

## Setting up Local LLM (Ollama)

For the best experience with local AI, we recommend using Ollama:

### 1. Install Ollama

Visit [ollama.ai](https://ollama.ai) and follow the installation instructions for your platform:

**macOS / Linux:**
```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

**Windows:**
Download and run the installer from [ollama.ai](https://ollama.ai)

### 2. Pull a model

Start with a small, fast model:
```bash
ollama pull llama2
```

Or try other models:
```bash
ollama pull mistral      # Fast and capable
ollama pull llama2:13b   # Larger, more capable
ollama pull codellama    # Specialized for code
```

### 3. Verify Ollama is running

```bash
ollama list
```

You should see the models you've pulled.

### 4. Run the demo

The demo will automatically detect and use Ollama:
```bash
make run
```

You'll see:
```
Using Ollama LLM (local)
```

If Ollama is not available, it will fall back to a mock LLM.

## Running the Demo

The complete workflow demo showcases all major features:

```bash
make run-demo
```

### What the demo shows:

1. **Memory Management**: Storing and retrieving events and habits
2. **Habit Tracking**: Statistical modeling of user routines
3. **Wake Word Detection**: Simple energy-based detection (production would use OpenWakeWord)
4. **LLM Interaction**: Using local Ollama or mock provider
5. **Intent Generation**: Converting user input to structured intents
6. **Policy Enforcement**: Permission checking before actions
7. **Context-Aware Planning**: Reasoning about user context

### Sample output:

```
=== Local Agent Core Demo ===

1. Initializing memory store...
   Stored 2 events

2. Creating habit tracking...
   Habit: Morning exercise
   Variance: Some(0.0) (lower = more consistent)

3. Testing wake word detection...
   Wake words: ["hey agent"]
   Low energy audio detected: false
   High energy audio detected: true

4. Processing user input with LLM...
   Using Ollama LLM (local)
   User: What's the weather like today?
   Agent: [Response from LLM]

... [more output]
```

## Understanding the Architecture

The project follows a strict separation between thinking and acting:

```
┌─────────────────────────────────────┐
│     Rust Agent Core (Thinking)      │
│  • Memory                            │
│  • LLM reasoning                     │
│  • Intent generation                 │
│  • Policy enforcement                │
└────────────┬────────────────────────┘
             │ JSON intents
┌────────────▼────────────────────────┐
│    Go Device Agents (Acting)        │
│  • Intent validation                 │
│  • Action execution                  │
│  • OS integration                    │
└─────────────────────────────────────┘
```

**Key Principle:** The agent core never executes actions directly. It only emits structured intents (JSON) that device agents validate and execute.

### Components:

- **Rust Agent Core** (`rust-agent-core/`)
  - Memory layer (SQLite)
  - Habit modeling
  - LLM abstraction (Ollama support)
  - Wake word detection
  - Intent generation
  - Policy engine

- **Go Device Agents** (`go-device-agent/`)
  - Intent gateway
  - Action executors
  - OS integration

## Next Steps

### For Users

1. **Explore the code**
   - Look at `rust-agent-core/examples/complete_workflow.rs` for a complete example
   - Check `rust-agent-core/src/` for the core modules

2. **Try different LLM models**
   ```bash
   ollama pull mistral
   # The code automatically uses whatever model you specify
   ```

3. **Customize the agent**
   - Add your own intents in `rust-agent-core/src/intent/mod.rs`
   - Create custom executors in `go-device-agent/pkg/executor/`

### For Developers

1. **Read the architecture docs**
   - [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed architecture
   - [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines

2. **Run tests continuously**
   ```bash
   make dev    # Auto-reload on changes
   ```

3. **Add features**
   - The codebase is modular and extensible
   - Follow the principle: agent emits intents, never executes

## Common Issues

### Ollama not detected

**Problem:** Demo shows "Ollama not available"

**Solution:**
1. Ensure Ollama is installed and running
2. Pull a model: `ollama pull llama2`
3. Check Ollama is accessible: `curl http://localhost:11434/api/tags`

### Build errors

**Problem:** Rust compilation fails

**Solution:**
1. Update Rust: `rustup update`
2. Clean build: `make clean && make build`

**Problem:** Go compilation fails

**Solution:**
1. Update Go to 1.21+
2. Clean build: `cd go-device-agent && go clean && go build ./...`

## Getting Help

- **Documentation**: See [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md)
- **Examples**: Check `rust-agent-core/examples/`
- **Issues**: Report bugs on GitHub Issues

## Summary

You now have a working local AI agent with:
- ✅ Wake word detection
- ✅ Local LLM integration (Ollama)
- ✅ Memory and habit tracking
- ✅ Intent-based architecture
- ✅ Policy enforcement
- ✅ Easy development workflow (Makefile)

**Ready to build something amazing?** Start by modifying the demo or creating your own intents!
