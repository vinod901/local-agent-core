# local-agent-core

A privacy-first personal AI agent core designed to run locally with voice I/O, habit and context awareness, event summarization, and controlled action delegation.

## 🔒 Privacy-First Design

This agent operates in a sandboxed environment and emits structured intents, while real-world actions—digital or physical—are executed by explicit, permissioned modules or device agents. All data stays local unless explicitly permitted otherwise.

## 🌟 Features

- **Local Execution**: Runs entirely on your device - your data never leaves without permission
- **Voice I/O**: Abstracted voice input (speech-to-text) and output (text-to-speech) interfaces
- **Context Awareness**: Tracks user habits, routines, and events for personalized assistance
- **Event Summarization**: Automatically summarizes recent interactions and context
- **Replaceable LLMs**: Abstract LLM interface works with any provider (OpenAI, Anthropic, local models, etc.)
- **Sandboxed Execution**: All actions run in isolated, controlled environments
- **Permission System**: Explicit, scoped permissions required for all real-world actions
- **Module Architecture**: Extensible system for digital and physical action delegation

## 📦 Installation

```bash
npm install local-agent-core
```

## 🚀 Quick Start

```typescript
import { Agent, MockLLMProvider, AgentConfig } from 'local-agent-core';

// Configure the agent
const config: AgentConfig = {
  userId: 'your-user-id',
  llmProvider: 'mock', // or your preferred provider
  voiceEnabled: true,
  privacyMode: 'strict',
  dataRetentionDays: 30,
  allowedModules: ['time', 'weather', 'reminder'],
};

// Create an LLM provider (mock for demo, replace with real provider)
const llmProvider = new MockLLMProvider();

// Initialize the agent
const agent = new Agent(config, llmProvider);

// Process text input
const response = await agent.processText('Hello! What time is it?');
console.log(response.text);
console.log('Intents detected:', response.intents);
```

## 📖 Core Concepts

### Agent
The main orchestrator that coordinates all components:
- Processes user input (text or voice)
- Manages conversation context
- Parses intents from user requests
- Enforces permissions for actions

### LLM Provider
Abstract interface for language models:
- Replaceable - works with any LLM
- Supports streaming and standard completion
- Includes mock provider for testing

### Context Store
Privacy-first local storage:
- Tracks user habits and routines
- Records events for context awareness
- Summarizes recent interactions
- Respects data retention policies

### Permission Manager
Security layer for action delegation:
- Explicit permission requests
- Scoped permissions
- Time-based expiration
- Easy revocation

### Module Registry
Extensible action system:
- Register custom action modules
- Sandboxed execution
- Clear capability declarations
- Module discovery and management

## 💡 Examples

### Basic Usage

```typescript
import { Agent, MockLLMProvider, AgentConfig } from 'local-agent-core';

const config: AgentConfig = {
  userId: 'demo-user',
  llmProvider: 'mock',
  voiceEnabled: false,
  privacyMode: 'strict',
  dataRetentionDays: 30,
  allowedModules: ['time', 'weather'],
};

const agent = new Agent(config, new MockLLMProvider());

// Simple interaction
const response = await agent.processText('Hello!');
console.log(response.text);
```

### Context Awareness

```typescript
// Add context
const contextStore = agent.getContextStore();
contextStore.setLocation('home');
contextStore.setActivity('working');

// Add a habit
contextStore.addHabit({
  name: 'Morning exercise',
  frequency: 'daily',
  schedule: '07:00',
});

// Agent uses this context in responses
const response = await agent.processText('Should I take a break?');
```

### Action Delegation with Permissions

```typescript
import { MockActionModule } from 'local-agent-core';

// Register an action module
const deviceModule = new MockActionModule('device', [
  'device.control',
  'device.query'
]);
agent.getModuleRegistry().register(deviceModule);

// Create an intent
const intent = {
  type: 'device.control',
  confidence: 0.9,
  parameters: { device: 'light', action: 'on' },
  requiresPermission: true,
  targetModule: 'device',
  timestamp: new Date(),
};

// Request permission
const permManager = agent.getPermissionManager();
const requestId = permManager.requestPermission({
  intentId: 'intent-1',
  action: 'device.control',
  module: 'device',
  scope: ['living room'],
  reasoning: 'User wants to control lights',
});

// Grant permission (expires in 1 hour)
permManager.grantPermission(requestId, { expiresIn: 3600000 });

// Execute action (now permitted)
const result = await agent.executeIntent(intent);
console.log('Action result:', result);
```

### Custom LLM Provider

```typescript
import { BaseLLMProvider, LLMOptions, LLMResponse } from 'local-agent-core';

class MyLLMProvider extends BaseLLMProvider {
  constructor() {
    super('my-custom-llm');
  }

  async complete(prompt: string, options?: LLMOptions): Promise<LLMResponse> {
    // Implement your LLM integration here
    // This could call OpenAI, Anthropic, a local model, etc.
    const response = await myLLMAPI.generate(prompt, options);
    
    return {
      text: response.text,
      usage: {
        promptTokens: response.promptTokens,
        completionTokens: response.completionTokens,
        totalTokens: response.totalTokens,
      },
      finishReason: 'stop',
    };
  }
}

// Use your custom provider
const agent = new Agent(config, new MyLLMProvider());
```

### Custom Action Module

```typescript
import { ActionModule, Intent, ActionResult, ModuleCapabilities } from 'local-agent-core';

class SmartHomeModule implements ActionModule {
  getName(): string {
    return 'smarthome';
  }

  getSupportedActions(): string[] {
    return ['smarthome.light.on', 'smarthome.light.off', 'smarthome.thermostat.set'];
  }

  async execute(intent: Intent): Promise<ActionResult> {
    // Implement your action logic here
    // This could control real IoT devices
    
    return {
      success: true,
      intentId: intent.timestamp.toISOString(),
      module: this.getName(),
      action: intent.type,
      result: { status: 'executed' },
      timestamp: new Date(),
    };
  }

  async isAvailable(): Promise<boolean> {
    // Check if hardware/services are available
    return true;
  }

  getCapabilities(): ModuleCapabilities {
    return {
      name: this.getName(),
      description: 'Smart home device control',
      actions: [
        {
          name: 'smarthome.light.on',
          description: 'Turn on a light',
          parameters: [
            { name: 'device', type: 'string', required: true, description: 'Device name' }
          ],
          riskLevel: 'low',
        },
      ],
    };
  }
}

// Register your module
agent.getModuleRegistry().register(new SmartHomeModule());
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         User Input                          │
│                    (Text/Voice/Events)                      │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                        Agent Core                           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              LLM Provider (Replaceable)              │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Context Store (Local)                   │  │
│  │  • User habits    • Events    • Current context      │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Intent Parser                           │  │
│  │  Extracts structured intents from responses          │  │
│  └──────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │ Structured Intents
┌────────────────────────▼────────────────────────────────────┐
│                  Permission Manager                         │
│           (Explicit, Scoped, Time-limited)                  │
└────────────────────────┬────────────────────────────────────┘
                         │ Authorized Actions
┌────────────────────────▼────────────────────────────────────┐
│                   Module Registry                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  Device  │  │ Message  │  │   File   │  │  Custom  │  │
│  │  Module  │  │  Module  │  │  Module  │  │  Module  │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
              Real-world Actions
        (Digital/Physical, Sandboxed)
```

## 🔐 Security & Privacy

1. **Local-First**: All data processing happens on your device
2. **No Tracking**: No telemetry, analytics, or data collection
3. **Explicit Permissions**: Every action requires user approval
4. **Scoped Access**: Permissions are limited in scope and time
5. **Sandboxed Modules**: Actions run in isolated environments
6. **Data Retention**: Configurable data retention policies
7. **Clear Logs**: All actions are logged for transparency

## 🧪 Testing

Run the test suite:

```bash
npm test
```

Run with coverage:

```bash
npm test -- --coverage
```

## 🛠️ Development

Build the project:

```bash
npm run build
```

Run the examples:

```bash
npm run dev
```

Lint the code:

```bash
npm run lint
```

Format the code:

```bash
npm run format
```

## 📝 API Documentation

### Agent

Main agent class for orchestrating all components.

**Methods:**
- `processText(input: string): Promise<AgentResponse>` - Process text input
- `processVoice(audioData: Buffer): Promise<AgentResponse>` - Process voice input
- `executeIntent(intent: Intent): Promise<ActionResult>` - Execute an intent
- `getModuleRegistry(): ModuleRegistry` - Access the module registry
- `getPermissionManager(): PermissionManager` - Access the permission manager
- `getContextStore(): ContextStore` - Access the context store
- `clearHistory(): void` - Clear conversation history

### ContextStore

Manages user context, habits, and events locally.

**Methods:**
- `addEvent(event): ContextEvent` - Add a context event
- `getRecentEvents(limit?): ContextEvent[]` - Get recent events
- `addHabit(habit): Habit` - Add or update a habit
- `completeHabit(habitId): void` - Mark habit as completed
- `getContext(): Context` - Get current context
- `clear(): void` - Clear all context data

### PermissionManager

Manages permissions for action delegation.

**Methods:**
- `requestPermission(request): string` - Request a permission
- `grantPermission(requestId, options?): Permission | null` - Grant a permission
- `isPermitted(module, action, scope?): boolean` - Check if action is permitted
- `revokeModule(module): void` - Revoke all permissions for a module

### ModuleRegistry

Manages action modules.

**Methods:**
- `register(module): void` - Register an action module
- `getModule(name): ActionModule | undefined` - Get a module by name
- `getAllModules(): ActionModule[]` - Get all registered modules

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT

## 🙏 Acknowledgments

This project is designed with privacy and security as first principles, inspired by the need for AI agents that respect user autonomy and data sovereignty.
