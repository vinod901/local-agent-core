# local-agent-core

A privacy-first local AI agent with strict separation between cognitive functions (thinking) and execution (acting).

## 🎯 What This Is

A **local-first personal AI agent** that:

- 🎤 Listens and speaks naturally (voice-first)
- 🧠 Understands routines, habits, preferences, and context
- 💾 Maintains long-term personal knowledge and continuity
- 🌍 Observes relevant world events selectively
- 🔄 Reasons locally using interchangeable LLMs
- 🔒 **Does NOT act directly** - delegates actions safely
- 🤖 Can later control physical systems (robotics) without changing its core

**This is NOT:**
- A chatbot
- A cloud assistant
- Autonomous AI

**This IS:**
- A cognitive delegate with strict safety boundaries
- Local-first, privacy-first
- Thinking and acting are strictly separated

## 🏗️ Architecture

```
Audio Input → Wake Word → STT (whisper.cpp)
                           ↓
        ┌──────────────────────────────────┐
        │    Agent Core (Rust)              │
        │  • Memory (SQLite)                │
        │  • Habit modeling                 │
        │  • Planner                        │
        │  • Policy engine                  │
        │  • Intent generator               │
        └──────────────────────────────────┘
                           ↓ Structured Intent (JSON)
        ┌──────────────────────────────────┐
        │   Intent Gateway (Go)             │
        │   Secure Boundary                 │
        └──────────────────────────────────┘
                           ↓
        ┌──────────────────────────────────┐
        │   Device Agents (Go)              │
        │  • Device control                 │
        │  • OS integration                 │
        │  • Robot control (future)         │
        └──────────────────────────────────┘
                           ↓
              Real-world Actions
```

**Key Invariant:** Agent emits intent, never executes actions.

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

## 🔑 Core Principles (Non-Negotiable)

### 1. Purpose First (Prayojana-anivāryatā)
Accessibility, continuity, and trust — not novelty or automation hype.

### 2. Separation of Thinking and Acting
```
Thinking → Intent → Permissioned Execution
```
Never direct action.

### 3. Local-First & Privacy-First
All cognition runs locally. No data exhaust. No training on user data.

### 4. Agent ≠ Robot
The agent is cognitive. Robots are optional executors added later.

### 5. Bounded Agency
The agent prepares, suggests, summarizes — the human authorizes.

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ (`cargo`)
- Go 1.21+ (`go`)
- Docker & docker-compose (optional)

### Build Rust Agent Core

```bash
cd rust-agent-core
cargo build --release
cargo test
```

### Build Go Device Agent

```bash
cd go-device-agent
go build ./...
go test ./...
```

### Run Device Agent Demo

```bash
cd go-device-agent
go run cmd/agent/main.go
```

### Using Docker Compose

```bash
docker-compose up --build
```

## 📚 Components

### Rust Agent Core (`rust-agent-core/`)

The **cognitive engine** implemented in Rust for:
- Memory safety and determinism
- Future robotics compatibility
- High performance

**Modules:**
- `memory` - SQLite-based persistent storage
- `habit` - Statistical habit/routine modeling
- `planner` - Context-aware reasoning
- `policy` - Permission enforcement
- `intent` - Intent generation (outputs JSON)
- `llm` - LLM abstraction layer
- `voice` - Voice I/O interfaces

### Go Device Agents (`go-device-agent/`)

The **execution layer** implemented in Go for:
- Simple, auditable code
- OS integration
- Easy replacement/extension

**Packages:**
- `intent` - Intent structure definitions
- `gateway` - Secure intent gateway
- `executor` - Action executors
- `cmd/agent` - Main device agent

### TypeScript Reference (`src/`)

Legacy TypeScript implementation kept for reference. The production system uses Rust + Go.

## 💡 Usage Example

### Agent Core (Rust) - Emits Intent

```rust
use rust_agent_core::intent::IntentGenerator;
use std::collections::HashMap;

let generator = IntentGenerator::new();
let mut params = HashMap::new();
params.insert("device".to_string(), serde_json::json!("light"));
params.insert("action".to_string(), serde_json::json!("on"));

let intent = generator.generate(
    "device.control".to_string(),
    0.9,
    params,
    "User wants to turn on lights".to_string(),
)?;

// Serialize to JSON for device agent
let json = generator.to_json(&intent)?;
// Send to device agent via gateway
```

### Device Agent (Go) - Executes Intent

```go
import (
    "context"
    "github.com/vinod901/local-agent-core/go-device-agent/pkg/gateway"
    "github.com/vinod901/local-agent-core/go-device-agent/pkg/executor"
)

// Create gateway
gw := gateway.NewGateway(logger)

// Register executors
gw.RegisterExecutor(executor.NewDeviceExecutor())

// Process intent from agent core
result, err := gw.ProcessIntent(ctx, intentJSON)
```

## 🛡️ Security & Privacy

1. **Local-First**: All data processing happens on your device
2. **No Tracking**: No telemetry, analytics, or data collection
3. **Explicit Permissions**: Every action requires user approval
4. **Scoped Access**: Permissions are limited in scope and time
5. **Sandboxed Execution**: Actions run in isolated environments
6. **Clear Logs**: All actions are logged for transparency
7. **Separation of Concerns**: Agent thinks, device agents act

## 🧪 Testing

### Rust Tests
```bash
cd rust-agent-core
cargo test
```

### Go Tests
```bash
cd go-device-agent
go test ./...
```

### TypeScript Tests (Legacy)
```bash
npm test
```

## 🗺️ Roadmap

- [x] Rust agent core foundation
- [x] Go device agent framework
- [x] Memory layer (SQLite)
- [x] Habit modeling
- [x] Intent generation
- [x] Policy engine
- [ ] Wake word detection (OpenWakeWord/Porcupine)
- [ ] Speech-to-text (whisper.cpp integration)
- [ ] Text-to-speech (Piper TTS integration)
- [ ] Local LLM integration (llama.cpp)
- [ ] HTTP API for agent ↔ device communication
- [ ] Robot control executor (with safety)

## 🎯 Technology Choices (Locked)

### Language Split
- **Rust**: Agent core (cognition, memory, planning, policy, voice I/O)
  - Memory safety, determinism, future robotics
- **Go**: Device agents (OS integration, networking, UI, observability)
  - Simple, auditable, replaceable executors

### Assembled Components (No Reinvention)
- Speech → Text: whisper.cpp
- Text → Speech: Piper TTS
- Wake word: OpenWakeWord / Porcupine
- LLMs: local (llama.cpp family) or cloud — behind abstraction
- Memory: SQLite (SQL-first, auditable, deterministic)
- Dev & simulation: Docker + docker-compose

**No vectors or embeddings by default.** Those can be layered later.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Key guidelines:**
- Respect the architecture boundaries
- Agent core in Rust, device agents in Go
- No direct action execution in agent core
- All actions through intent → gateway → executor
- Privacy-first, local-first always

## 📄 License

MIT - See [LICENSE](LICENSE)

## 🙏 Acknowledgments

This project is built on the principle that AI agents should respect user autonomy and data sovereignty. The strict separation between thinking and acting ensures safety while enabling powerful assistance.

---

**Remember:** The value is in composition + boundaries, not raw ML.
