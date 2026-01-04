# Go Device Agent

The execution layer for the local-first personal AI agent. This service receives structured intents from the Rust agent core and executes real-world actions.

## Features

- **Intent Gateway**: Secure boundary between thinking and acting
- **Executor Framework**: Pluggable action executors
- **Permission Validation**: Double-checks permissions before execution
- **OS Integration**: Native system integration capabilities
- **Observability**: Logging and monitoring
- **Extensible**: Easy to add new executors

## Core Principle

**Device agents execute intents, never make decisions.**

The agent receives structured intents (JSON) from the agent core, validates permissions, and executes the requested actions. All intelligence resides in the agent core.

## Quick Start

```bash
# Build
go build ./...

# Run tests
go test ./...

# Run device agent
go run cmd/agent/main.go
```

## Architecture

```
┌────────────────────────────────────┐
│   Agent Core (Rust)                │
│   Emits Intent (JSON)              │
└────────────┬───────────────────────┘
             ↓ Intent JSON
┌────────────▼───────────────────────┐
│   Intent Gateway (Go)              │
│   • Parse intent                   │
│   • Validate structure             │
│   • Check permissions              │
│   • Route to executor              │
└────────────┬───────────────────────┘
             ↓
┌────────────▼───────────────────────┐
│   Executors                        │
│   ┌──────────┐  ┌──────────┐      │
│   │  Device  │  │  Notify  │      │
│   │ Control  │  │  System  │      │
│   └──────────┘  └──────────┘      │
│   ┌──────────┐  ┌──────────┐      │
│   │  Desktop │  │  Robot   │      │
│   │    OS    │  │ Control  │      │
│   └──────────┘  └──────────┘      │
└────────────┬───────────────────────┘
             ↓
     Real-World Actions
```

## Usage

### Running the Device Agent

```bash
cd go-device-agent
go run cmd/agent/main.go
```

### Sample Intent Processing

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
gw.RegisterExecutor(executor.NewNotificationExecutor())

// Process intent from agent core
intentJSON := []byte(`{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "intent_type": "device.control",
  "confidence": 0.9,
  "parameters": {
    "device": "living_room_light",
    "action": "on"
  },
  "reasoning": "User wants to turn on the living room light",
  "requires_permission": true,
  "target_module": "device",
  "created_at": "2026-01-03T15:00:00Z"
}`)

result, err := gw.ProcessIntent(context.Background(), intentJSON)
```

## Packages

### `pkg/intent`
Intent structure definitions matching the Rust agent core:
- `Intent` - Structured intent from agent
- `ParseIntent()` - Parse JSON intent
- `Validate()` - Validate intent structure

### `pkg/gateway`
Secure intent gateway:
- `Gateway` - Main gateway struct
- `RegisterExecutor()` - Register action executors
- `ProcessIntent()` - Process intent JSON
- Permission validation
- Executor routing

### `pkg/executor`
Action executors:
- `Executor` interface
- `DeviceExecutor` - Device control
- `NotificationExecutor` - System notifications
- `MockExecutor` - Testing

### `cmd/agent`
Main device agent application:
- Initializes gateway
- Registers executors
- Demonstrates intent processing

## Creating Custom Executors

Implement the `Executor` interface:

```go
type MyExecutor struct {}

func (e *MyExecutor) Name() string {
    return "mymodule"
}

func (e *MyExecutor) SupportedActions() []string {
    return []string{"mymodule.action1", "mymodule.action2"}
}

func (e *MyExecutor) Execute(ctx context.Context, intent *intent.Intent) (*gateway.ExecutionResult, error) {
    // Your execution logic here
    return &gateway.ExecutionResult{
        Success: true,
        IntentID: intent.ID,
        Module: e.Name(),
        Action: intent.IntentType,
        Result: map[string]interface{}{
            "message": "Action completed",
        },
        Timestamp: time.Now().Format(time.RFC3339),
    }, nil
}

func (e *MyExecutor) IsAvailable() bool {
    return true // Check if your service is available
}

// Register it
gateway.RegisterExecutor(&MyExecutor{})
```

## Security

### Intent Validation
All intents are validated before execution:
1. Parse JSON structure
2. Check required fields
3. Validate confidence threshold
4. Verify target module exists
5. Check executor availability

### Permission Enforcement
Double-check permissions:
1. Agent core enforces permissions
2. Gateway validates intent structure
3. Executor performs action safely

### Sandboxing
Each executor runs isolated:
- Limited scope
- Error boundaries
- Resource limits (future)

## Why Go?

- **Simplicity**: Easy to understand and audit
- **OS Integration**: Great system integration capabilities
- **Performance**: Fast startup, low overhead
- **Concurrency**: Built-in goroutines
- **Cross-platform**: Works on all major platforms
- **Replacement**: Easy to replace/extend

## Examples

### Device Control
```json
{
  "intent_type": "device.control",
  "parameters": {
    "device": "living_room_light",
    "action": "on"
  }
}
```

### Notification
```json
{
  "intent_type": "notification.send",
  "parameters": {
    "message": "Time for your meeting"
  }
}
```

### Query
```json
{
  "intent_type": "time.query",
  "parameters": {}
}
```

## Testing

```bash
# Run all tests
go test ./...

# Run with verbose output
go test -v ./...

# Test specific package
go test ./pkg/gateway

# Build
go build ./...
```

## Roadmap

- [x] Intent structure definitions
- [x] Intent gateway
- [x] Executor framework
- [x] Device executor
- [x] Notification executor
- [ ] HTTP server for agent core communication
- [ ] OS-specific integrations
- [ ] Robot control executor (with safety)
- [ ] Observability package
- [ ] Configuration management

## Contributing

Contributions welcome! Please ensure:
- Tests pass: `go test ./...`
- Code is formatted: `go fmt ./...`
- Linting passes: `golangci-lint run`

## License

MIT - See [LICENSE](../LICENSE)
