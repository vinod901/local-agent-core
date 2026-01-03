/**
 * Abstract interface for LLM providers
 * This allows the agent to work with any LLM (OpenAI, Anthropic, local models, etc.)
 */
export interface LLMProvider {
  /**
   * Generate a completion from the LLM
   */
  complete(prompt: string, options?: LLMOptions): Promise<LLMResponse>;

  /**
   * Generate a streaming completion
   */
  streamComplete?(
    prompt: string,
    options?: LLMOptions
  ): AsyncGenerator<LLMStreamChunk, void, unknown>;

  /**
   * Get provider name
   */
  getName(): string;

  /**
   * Check if provider is available
   */
  isAvailable(): Promise<boolean>;
}

/**
 * Options for LLM completion
 */
export interface LLMOptions {
  temperature?: number;
  maxTokens?: number;
  stopSequences?: string[];
  systemPrompt?: string;
  conversationHistory?: Array<{ role: string; content: string }>;
}

/**
 * Response from LLM
 */
export interface LLMResponse {
  text: string;
  usage?: {
    promptTokens: number;
    completionTokens: number;
    totalTokens: number;
  };
  finishReason?: string;
}

/**
 * Streaming chunk from LLM
 */
export interface LLMStreamChunk {
  text: string;
  done: boolean;
}

/**
 * Base class for LLM provider implementations
 */
export abstract class BaseLLMProvider implements LLMProvider {
  protected name: string;

  constructor(name: string) {
    this.name = name;
  }

  abstract complete(prompt: string, options?: LLMOptions): Promise<LLMResponse>;

  getName(): string {
    return this.name;
  }

  async isAvailable(): Promise<boolean> {
    try {
      await this.complete('test', { maxTokens: 1 });
      return true;
    } catch {
      return false;
    }
  }
}
