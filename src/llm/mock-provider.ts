import { BaseLLMProvider, LLMOptions, LLMResponse } from './provider';

/**
 * Mock LLM provider for testing and development
 * Returns predefined responses based on patterns
 */
export class MockLLMProvider extends BaseLLMProvider {
  private responses: Map<string, string>;

  constructor() {
    super('mock-llm');
    this.responses = new Map();
    this.initializeDefaultResponses();
  }

  private initializeDefaultResponses(): void {
    // Default responses for common queries
    this.responses.set('default', 'I understand. How can I help you with that?');
    this.responses.set(
      'greeting',
      'Hello! I am your local AI agent. How can I assist you today?'
    );
    this.responses.set('time', 'Let me check the current time for you.');
    this.responses.set('weather', 'I can help you check the weather.');
    this.responses.set('reminder', 'I will set a reminder for you.');
  }

  async complete(prompt: string, _options?: LLMOptions): Promise<LLMResponse> {
    const lowerPrompt = prompt.toLowerCase();

    let responseText = this.responses.get('default') || 'I understand.';

    // Pattern matching for common queries
    if (lowerPrompt.includes('hello') || lowerPrompt.includes('hi')) {
      responseText = this.responses.get('greeting')!;
    } else if (lowerPrompt.includes('time')) {
      responseText = this.responses.get('time')!;
    } else if (lowerPrompt.includes('weather')) {
      responseText = this.responses.get('weather')!;
    } else if (lowerPrompt.includes('remind')) {
      responseText = this.responses.get('reminder')!;
    }

    // Simulate processing delay
    await new Promise((resolve) => setTimeout(resolve, 100));

    return {
      text: responseText,
      usage: {
        promptTokens: prompt.length / 4,
        completionTokens: responseText.length / 4,
        totalTokens: (prompt.length + responseText.length) / 4,
      },
      finishReason: 'stop',
    };
  }

  addResponse(key: string, response: string): void {
    this.responses.set(key, response);
  }

  async isAvailable(): Promise<boolean> {
    return true;
  }
}
