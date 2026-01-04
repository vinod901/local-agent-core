# Local Agent Core - Architecture Documentation

## Overview

This is a local-first personal AI agent with strict separation between cognitive functions (thinking) and execution (acting).

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                       Audio Input (Microphone)                  │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                    Wake Word Detection                           │
│              (OpenWakeWord / Porcupine - TBD)                   │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│                  Speech-to-Text (STT)                           │
│                    (whisper.cpp - TBD)                          │
└────────────────────────┬────────────────────────────────────────┘
                         │ transcribed text
┌────────────────────────▼────────────────────────────────────────┐
│                    AGENT CORE (Rust)                            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Memory Layer (SQLite)                       │  │
│  │  • Events • Habits • Context • History                   │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Habit Modeling                              │  │
│  │  • Statistical pattern learning                          │  │
│  │  • Variance tracking                                     │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              LLM Abstraction                             │  │
│  │  • Local models (llama.cpp family)                       │  │
│  │  • Cloud providers (abstracted)                          │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Planner                                     │  │
│  │  • Context-aware reasoning                               │  │
│  │  • Action suggestions                                    │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Policy Engine                               │  │
│  │  • Permission enforcement                                │  │
│  │  • Safety boundaries                                     │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              Intent Generator                            │  │
│  │  • Structured JSON output                                │  │
│  │  • No direct execution                                   │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────────┘
                         │ Structured Intent (JSON)
┌────────────────────────▼────────────────────────────────────────┐
│                    Intent Gateway (Go)                           │
│                   Secure Boundary                                │
│  • Intent validation                                            │
│  • Permission checking                                          │
│  • Executor routing                                             │
└────────────────────────┬────────────────────────────────────────┘
                         │ Authorized Intent
┌────────────────────────▼────────────────────────────────────────┐
│                   Device Agents (Go)                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │
│  │  Device  │  │ Desktop  │  │  Robot   │  │   Custom     │   │
│  │  Control │  │   OS     │  │  Control │  │  Executor    │   │
│  │ Executor │  │Integrator│  │(Future)  │  │              │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────────┘   │
└────────────────────────┬────────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────────┐
│              Real-World Actions                                  │
│        (Digital Services / Physical Systems)                     │
└──────────────────────────────────────────────────────────────────┘
```

## Key Invariants

1. **Agent NEVER executes actions** - Only emits structured intents
2. **All cognition in Rust** - Memory safety, determinism, future robotics
3. **All execution in Go** - Simple, auditable, replaceable
4. **SQLite for memory** - SQL-first, auditable, deterministic
5. **Local-first** - No cloud dependency, privacy preserved

## Components

### Rust Agent Core (`rust-agent-core/`)

The cognitive engine that:
- Maintains persistent memory (SQLite)
- Models habits and routines statistically
- Compresses events and summarizes context
- Plans and reasons about actions
- Enforces policy and permissions
- **Emits structured intents** (JSON)

**Key Modules:**
- `memory` - SQLite-based persistent storage
- `habit` - Statistical habit modeling
- `planner` - Context-aware reasoning
- `policy` - Permission enforcement
- `intent` - Intent generation (JSON output)
- `llm` - LLM abstraction layer (with Ollama support)
- `voice` - Voice I/O interfaces (with simple wake word detection)

### Go Device Agents (`go-device-agent/`)

The execution layer that:
- Receives structured intents from agent core
- Validates and routes intents
- Executes authorized actions
- Integrates with OS/services/devices
- Provides observability

**Key Packages:**
- `intent` - Intent structure definitions
- `gateway` - Secure intent gateway
- `executor` - Action executors
- `cmd/agent` - Main device agent entry point

### Implemented Components

- ✅ Wake word detection (simple energy-based implementation)
- ✅ Local LLM integration (Ollama via HTTP API)
- ✅ Makefile for easy development workflow

### Future Components (To Be Implemented)

- Advanced wake word detection (OpenWakeWord/Porcupine)
- Speech-to-text (whisper.cpp integration)
- Text-to-speech (Piper TTS integration)
- Robot control executor (with safety constraints)

## Technology Stack

### Languages
- **Rust**: Agent core, voice I/O, memory
- **Go**: Device agents, OS integration, networking

### Dependencies
- **SQLite**: Persistent memory (via rusqlite)
- **Ollama**: Local LLM support (via HTTP API)
- **reqwest**: HTTP client for LLM API calls
- **whisper.cpp**: Speech-to-text (to be integrated)
- **Piper TTS**: Text-to-speech (to be integrated)

### Development
- **Docker**: Containerization
- **docker-compose**: Multi-service orchestration
- **Makefile**: Build automation and workflow management

## Core Principles

### 1. Purpose First (Prayojana-anivāryatā)
Accessibility, continuity, and trust — not novelty or automation hype.

### 2. Separation of Thinking and Acting
```
Thinking (Rust) → Intent (JSON) → Permissioned Execution (Go)
```
Never direct action.

### 3. Local-First & Privacy-First
All cognition runs locally. No data exhaust. No training on user data.

### 4. Agent ≠ Robot
The agent is cognitive. Robots are optional executors added later.

### 5. Bounded Agency
The agent prepares, suggests, summarizes — the human authorizes.

## Development Setup

### Prerequisites
- Rust (1.70+)
- Go (1.21+)
- Docker & docker-compose (for orchestration)
- Ollama (optional, for local LLM support)

### Building

```bash
# Using Makefile (recommended)
make build

# Or build components individually
make build-rust
make build-go

# Or manually
# Build Rust agent core
cd rust-agent-core
cargo build --release
cargo test

# Build Go device agents
cd go-device-agent
go build ./...
go test ./...

# Run device agent demo
go run cmd/agent/main.go
```

### Testing

```bash
# Using Makefile
make test

# Or manually
# Test Rust components
cd rust-agent-core
cargo test

# Test Go components
cd go-device-agent
go test ./...
```

### Running the Demo

```bash
# Using Makefile
make run

# Or manually
cd rust-agent-core
cargo run --release --example complete_workflow
```

### Local LLM Setup

1. Install Ollama from https://ollama.ai
2. Pull a model:
   ```bash
   ollama pull llama2
   # or
   ollama pull mistral
   ```
3. The demo will automatically detect and use Ollama if available

## Usage Example

### 1. Agent Core Emits Intent (Rust)

```rust
use rust_agent_core::intent::IntentGenerator;

let generator = IntentGenerator::new();
let intent = generator.generate(
    "device.control".to_string(),
    0.9,
    params,
    "User wants to turn on lights".to_string(),
)?;

// Serialize to JSON for device agent
let json = generator.to_json(&intent)?;
```

### 2. Device Agent Executes Intent (Go)

```go
// Gateway receives intent JSON from agent core
result, err := gateway.ProcessIntent(ctx, intentJSON)

// Result is logged and returned to agent core
```

## Security Model

### Permission System

1. Agent emits intent with `requires_permission: true`
2. Policy engine checks permissions
3. If not permitted, intent is queued for user approval
4. User grants permission (time-limited, scoped)
5. Intent is forwarded to device agent
6. Device agent validates and executes

### Safety Boundaries

- **Agent Core**: Cannot execute actions, only emit intents
- **Intent Gateway**: Validates, routes, enforces policy
- **Device Agents**: Sandboxed, limited scope, explicit capabilities

## Future: Robot Integration

When adding robotics:
1. Robot is just another device agent
2. Agent core remains unchanged
3. Robot agent enforces:
   - Speed limits
   - Collision detection
   - Emergency stop
   - Physics constraints

**Agent never knows about motors, sensors, or physics.**

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## License

MIT - See [LICENSE](../LICENSE)
