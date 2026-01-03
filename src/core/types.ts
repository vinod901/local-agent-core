/**
 * Core type definitions for the local agent
 */

/**
 * Structured intent emitted by the agent
 */
export interface Intent {
  type: string;
  confidence: number;
  parameters: Record<string, unknown>;
  requiresPermission: boolean;
  targetModule?: string;
  timestamp: Date;
}

/**
 * Permission request for action delegation
 */
export interface PermissionRequest {
  intentId: string;
  action: string;
  module: string;
  scope: string[];
  reasoning: string;
}

/**
 * Permission grant
 */
export interface Permission {
  module: string;
  actions: string[];
  scope: string[];
  expiresAt?: Date;
  grantedAt: Date;
}

/**
 * Context data structure
 */
export interface Context {
  userId: string;
  currentLocation?: string;
  currentActivity?: string;
  recentEvents: ContextEvent[];
  activeHabits: Habit[];
  timestamp: Date;
}

/**
 * Context event
 */
export interface ContextEvent {
  id: string;
  type: string;
  description: string;
  timestamp: Date;
  metadata: Record<string, unknown>;
}

/**
 * Habit tracking
 */
export interface Habit {
  id: string;
  name: string;
  frequency: 'daily' | 'weekly' | 'monthly';
  schedule?: string;
  lastCompleted?: Date;
  completionCount: number;
  createdAt: Date;
}

/**
 * Voice input
 */
export interface VoiceInput {
  text: string;
  confidence: number;
  language: string;
  timestamp: Date;
  audioMetadata?: Record<string, unknown>;
}

/**
 * Voice output
 */
export interface VoiceOutput {
  text: string;
  language: string;
  voice?: string;
  speed?: number;
}

/**
 * Agent configuration
 */
export interface AgentConfig {
  userId: string;
  llmProvider: string;
  voiceEnabled: boolean;
  privacyMode: 'strict' | 'balanced' | 'permissive';
  dataRetentionDays: number;
  allowedModules: string[];
}

/**
 * Action execution result
 */
export interface ActionResult {
  success: boolean;
  intentId: string;
  module: string;
  action: string;
  result?: unknown;
  error?: string;
  timestamp: Date;
}

/**
 * Agent response
 */
export interface AgentResponse {
  text: string;
  intents: Intent[];
  requiresPermission: boolean;
  timestamp: Date;
}
