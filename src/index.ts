/**
 * Local Agent Core
 * A privacy-first personal AI agent designed to run locally
 */

// Core exports
export { Agent } from './core/agent';
export { IntentParser } from './core/intent-parser';

// Type exports
export type {
  Intent,
  PermissionRequest,
  Permission,
  Context,
  ContextEvent,
  Habit,
  VoiceInput,
  VoiceOutput,
  AgentConfig,
  ActionResult,
  AgentResponse,
} from './core/types';

// LLM exports
export { BaseLLMProvider } from './llm/provider';
export type { LLMProvider, LLMOptions, LLMResponse, LLMStreamChunk } from './llm/provider';
export { MockLLMProvider } from './llm/mock-provider';

// Voice exports
export type {
  VoiceInputProvider,
  VoiceOutputProvider,
  VoiceTranscription,
  VoiceSpeechOptions,
} from './voice/provider';
export { MockVoiceInputProvider, MockVoiceOutputProvider } from './voice/provider';

// Context exports
export { ContextStore } from './context/store';

// Action exports
export { PermissionManager } from './actions/permission-manager';
export { ModuleRegistry, MockActionModule } from './actions/module';
export type { ActionModule, ModuleCapabilities, ActionDescriptor } from './actions/module';
