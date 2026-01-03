# Rust Agent Core

The cognitive engine for the local-first personal AI agent. This crate implements the "thinking" part of the agent, maintaining strict separation from execution.

## Features

- **Memory Layer**: SQLite-based persistent storage for events and habits
- **Habit Modeling**: Statistical learning of user routines (descriptive, not moralizing)
- **Event Compression**: Summarizes what changed that matters
- **Planning**: Context-aware reasoning and action suggestions
- **Policy Engine**: Permission enforcement and safety boundaries
- **Intent Generation**: Outputs structured JSON intents (never executes)
- **LLM Abstraction**: Works with any LLM provider (local or cloud)
- **Voice I/O Interfaces**: Abstract interfaces for STT/TTS/wake-word

## Core Principle

**The agent emits intents, never executes actions.**

This is enforced at the architectural level - the agent core has no capability to execute real-world actions. All intents are serialized to JSON and passed to device agents for execution.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-agent-core = { path = "../rust-agent-core" }
```

### Basic Usage

```rust
use rust_agent_core::{
    memory::MemoryStore,
    intent::IntentGenerator,
    policy::PolicyEngine,
    types::{Event, Habit, HabitFrequency},
};

// Initialize memory
let memory = MemoryStore::new("agent.db")?;

// Store events
let event = Event::new(
    "user_input".to_string(),
    "User asked about weather".to_string(),
    0.7,
);
memory.store_event(&event)?;

// Track habits
let habit = Habit::new(
    "Morning exercise".to_string(),
    "Daily exercise routine".to_string(),
    HabitFrequency::Daily,
);
memory.store_habit(&habit)?;

// Generate intents
let generator = IntentGenerator::new();
let intents = generator.parse_from_text("turn on the lights");

// Serialize to JSON for device agent
let json = generator.to_json(&intents[0])?;
// Send to device agent...
```

## Architecture

```
┌────────────────────────────────────┐
│      Memory (SQLite)               │
│  • Events                          │
│  • Habits                          │
│  • Context                         │
└────────────────────────────────────┘
             ↓
┌────────────────────────────────────┐
│      Habit Modeling                │
│  • Statistical analysis            │
│  • Pattern recognition             │
└────────────────────────────────────┘
             ↓
┌────────────────────────────────────┐
│      LLM Abstraction               │
│  • Local models (llama.cpp)        │
│  • Cloud providers                 │
└────────────────────────────────────┘
             ↓
┌────────────────────────────────────┐
│      Planner                       │
│  • Context-aware reasoning         │
│  • Action suggestions              │
└────────────────────────────────────┘
             ↓
┌────────────────────────────────────┐
│      Intent Generator              │
│  • Parse user input                │
│  • Generate structured intents     │
└────────────────────────────────────┘
             ↓
┌────────────────────────────────────┐
│      Policy Engine                 │
│  • Permission checks               │
│  • Safety enforcement              │
└────────────────────────────────────┘
             ↓
        Intent (JSON)
         ↓
   [Device Agent]
```

## Modules

### `memory`
SQLite-based persistent storage with full CRUD operations:
- Store and retrieve events
- Track habits and completions
- Privacy-first data retention

### `habit`
Statistical habit modeling:
- Pattern recognition
- Variance analysis
- Frequency suggestions
- No moral judgments

### `planner`
Context-aware reasoning:
- Build context summaries
- Compress events
- Suggest actions
- Evaluate intents

### `policy`
Safety and permission enforcement:
- Grant/revoke permissions
- Check intent permissions
- Time-limited access
- Scope restrictions

### `intent`
Intent generation and validation:
- Parse natural language
- Generate structured intents
- Validate intent structure
- Serialize to JSON

### `llm`
LLM abstraction layer:
- Provider interface
- Mock provider for testing
- Placeholders for local models

### `voice`
Voice I/O interfaces:
- Wake word detection (interface)
- Speech-to-text (interface)
- Text-to-speech (interface)

## Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_memory_store_creation

# Run example
cargo run --example complete_workflow
```

## Examples

See `examples/` directory for complete examples:
- `complete_workflow.rs` - Full agent workflow demonstration

## Privacy & Security

1. **Local-first**: All data stored locally in SQLite
2. **No cloud dependency**: Works entirely offline
3. **Auditable**: SQL queries are transparent
4. **Deterministic**: No hidden behavior
5. **Separation of concerns**: Agent thinks, device agents act
6. **Permission-based**: All actions require explicit permission

## Why Rust?

- **Memory safety**: No undefined behavior
- **Determinism**: Predictable performance
- **Future robotics**: Can run on embedded systems
- **Concurrency**: Safe parallelism when needed
- **Performance**: Efficient resource usage

## Roadmap

- [x] Memory layer (SQLite)
- [x] Habit modeling
- [x] Intent generation
- [x] Policy engine
- [x] Planner
- [ ] Whisper.cpp integration (STT)
- [ ] Piper TTS integration
- [ ] llama.cpp integration (local LLM)
- [ ] HTTP API for agent ↔ device communication

## Contributing

Contributions welcome! Please ensure:
- Tests pass: `cargo test`
- Code is formatted: `cargo fmt`
- No warnings: `cargo clippy`

## License

MIT - See [LICENSE](../LICENSE)
