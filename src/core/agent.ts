import { AgentConfig, AgentResponse, Intent, ActionResult } from '../core/types';
import { LLMProvider } from '../llm/provider';
import { VoiceInputProvider, VoiceOutputProvider } from '../voice/provider';
import { ContextStore } from '../context/store';
import { PermissionManager } from '../actions/permission-manager';
import { ModuleRegistry } from '../actions/module';
import { IntentParser } from './intent-parser';

/**
 * Main Agent class - orchestrates all components
 * Privacy-first: runs locally, no data leaves the system without permission
 */
export class Agent {
  private config: AgentConfig;
  private llm: LLMProvider;
  private voiceInput?: VoiceInputProvider;
  private voiceOutput?: VoiceOutputProvider;
  private contextStore: ContextStore;
  private permissionManager: PermissionManager;
  private moduleRegistry: ModuleRegistry;
  private intentParser: IntentParser;
  private conversationHistory: Array<{ role: string; content: string }> = [];

  constructor(
    config: AgentConfig,
    llm: LLMProvider,
    voiceInput?: VoiceInputProvider,
    voiceOutput?: VoiceOutputProvider
  ) {
    this.config = config;
    this.llm = llm;
    this.voiceInput = voiceInput;
    this.voiceOutput = voiceOutput;
    this.contextStore = new ContextStore(config.userId);
    this.permissionManager = new PermissionManager();
    this.moduleRegistry = new ModuleRegistry();
    this.intentParser = new IntentParser();
  }

  /**
   * Process text input and generate response
   */
  async processText(input: string): Promise<AgentResponse> {
    // Add to conversation history
    this.conversationHistory.push({ role: 'user', content: input });

    // Log the interaction as a context event
    this.contextStore.addEvent({
      type: 'user_input',
      description: input,
      metadata: { mode: 'text' },
    });

    // Build prompt with context
    const prompt = this.buildPrompt(input);

    // Get LLM response
    const llmResponse = await this.llm.complete(prompt, {
      temperature: 0.7,
      maxTokens: 500,
      conversationHistory: this.conversationHistory,
      systemPrompt: this.getSystemPrompt(),
    });

    // Add to conversation history
    this.conversationHistory.push({ role: 'assistant', content: llmResponse.text });

    // Parse intents from response
    const intents = this.intentParser.parseIntents(input);

    // Check if any intent requires permission
    const requiresPermission = intents.some((intent) => intent.requiresPermission);

    const response: AgentResponse = {
      text: llmResponse.text,
      intents,
      requiresPermission,
      timestamp: new Date(),
    };

    // Log the response
    this.contextStore.addEvent({
      type: 'agent_response',
      description: llmResponse.text,
      metadata: { intentCount: intents.length },
    });

    return response;
  }

  /**
   * Process voice input
   */
  async processVoice(audioData: Buffer): Promise<AgentResponse> {
    if (!this.voiceInput) {
      throw new Error('Voice input not configured');
    }

    // Transcribe audio
    const transcription = await this.voiceInput.transcribe(audioData);

    // Log voice input
    this.contextStore.addEvent({
      type: 'user_input',
      description: transcription.text,
      metadata: { mode: 'voice', confidence: transcription.confidence },
    });

    // Process as text
    const response = await this.processText(transcription.text);

    // Speak response if voice output is configured
    if (this.voiceOutput && this.config.voiceEnabled) {
      await this.voiceOutput.speak(response.text);
    }

    return response;
  }

  /**
   * Execute an intent
   */
  async executeIntent(intent: Intent): Promise<ActionResult> {
    const module = this.intentParser.getTargetModule(intent);

    // Check permission
    if (intent.requiresPermission) {
      const permitted = this.permissionManager.isPermitted(
        module,
        intent.type,
        Object.keys(intent.parameters)
      );

      if (!permitted) {
        return {
          success: false,
          intentId: intent.timestamp.toISOString(),
          module,
          action: intent.type,
          error: 'Permission denied',
          timestamp: new Date(),
        };
      }
    }

    // Get module
    const actionModule = this.moduleRegistry.getModule(module);
    if (!actionModule) {
      return {
        success: false,
        intentId: intent.timestamp.toISOString(),
        module,
        action: intent.type,
        error: `Module '${module}' not found`,
        timestamp: new Date(),
      };
    }

    // Execute action in sandboxed manner
    try {
      const result = await actionModule.execute(intent);

      // Log execution
      this.contextStore.addEvent({
        type: 'action_executed',
        description: `Executed ${intent.type}`,
        metadata: { module, success: result.success },
      });

      return result;
    } catch (error) {
      return {
        success: false,
        intentId: intent.timestamp.toISOString(),
        module,
        action: intent.type,
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: new Date(),
      };
    }
  }

  /**
   * Get module registry for registering action modules
   */
  getModuleRegistry(): ModuleRegistry {
    return this.moduleRegistry;
  }

  /**
   * Get permission manager
   */
  getPermissionManager(): PermissionManager {
    return this.permissionManager;
  }

  /**
   * Get context store
   */
  getContextStore(): ContextStore {
    return this.contextStore;
  }

  /**
   * Build prompt with context
   */
  private buildPrompt(input: string): string {
    const context = this.contextStore.getContext();
    const recentEvents = this.contextStore.summarizeEvents(5);

    let prompt = `User input: ${input}\n\n`;

    if (context.currentActivity) {
      prompt += `Current activity: ${context.currentActivity}\n`;
    }

    if (context.currentLocation) {
      prompt += `Current location: ${context.currentLocation}\n`;
    }

    if (recentEvents !== 'No recent events.') {
      prompt += `\nRecent context:\n${recentEvents}\n`;
    }

    if (context.activeHabits.length > 0) {
      prompt += `\nActive habits:\n`;
      context.activeHabits.forEach((habit) => {
        prompt += `- ${habit.name} (${habit.frequency})\n`;
      });
    }

    return prompt;
  }

  /**
   * Get system prompt for LLM
   */
  private getSystemPrompt(): string {
    return `You are a privacy-first personal AI assistant running locally on the user's device.
Your role is to:
1. Understand user intent and context
2. Provide helpful, concise responses
3. Respect user privacy - all data stays local
4. Be aware of user habits and routines
5. Emit structured intents for actions that require them
6. Never take actions without explicit permission

Be conversational, helpful, and respectful of the user's privacy and preferences.`;
  }

  /**
   * Clear conversation history
   */
  clearHistory(): void {
    this.conversationHistory = [];
  }

  /**
   * Get agent configuration
   */
  getConfig(): AgentConfig {
    return { ...this.config };
  }
}
