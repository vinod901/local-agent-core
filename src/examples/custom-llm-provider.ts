/**
 * Example: Custom LLM Provider Integration
 * 
 * This example shows how to integrate a real LLM provider.
 * In this case, we'll create a template for OpenAI integration.
 */

import { BaseLLMProvider, LLMOptions, LLMResponse } from '../llm/provider';

/**
 * OpenAI LLM Provider (Template - requires API key)
 * 
 * To use this provider:
 * 1. Install the OpenAI SDK: npm install openai
 * 2. Set your API key: export OPENAI_API_KEY=your-key-here
 * 3. Use this provider with the Agent
 */
export class OpenAIProvider extends BaseLLMProvider {
  private apiKey: string;
  private model: string;

  constructor(apiKey?: string, model = 'gpt-3.5-turbo') {
    super('openai');
    this.apiKey = apiKey || process.env.OPENAI_API_KEY || '';
    this.model = model;

    if (!this.apiKey) {
      console.warn('Warning: No OpenAI API key provided');
    }
  }

  async complete(_prompt: string, _options?: LLMOptions): Promise<LLMResponse> {
    if (!this.apiKey) {
      throw new Error('OpenAI API key not configured');
    }

    // NOTE: This is a template. Uncomment and install 'openai' package to use.
    /*
    const OpenAI = require('openai');
    const client = new OpenAI({ apiKey: this.apiKey });

    const messages = [];
    
    if (options?.systemPrompt) {
      messages.push({ role: 'system', content: options.systemPrompt });
    }
    
    if (options?.conversationHistory) {
      messages.push(...options.conversationHistory);
    }
    
    messages.push({ role: 'user', content: prompt });

    const completion = await client.chat.completions.create({
      model: this.model,
      messages,
      temperature: options?.temperature || 0.7,
      max_tokens: options?.maxTokens || 500,
      stop: options?.stopSequences,
    });

    return {
      text: completion.choices[0].message.content,
      usage: {
        promptTokens: completion.usage.prompt_tokens,
        completionTokens: completion.usage.completion_tokens,
        totalTokens: completion.usage.total_tokens,
      },
      finishReason: completion.choices[0].finish_reason,
    };
    */

    // Placeholder for demonstration
    throw new Error('OpenAI provider not implemented. Uncomment the code above and install the openai package.');
  }

  async isAvailable(): Promise<boolean> {
    return !!this.apiKey;
  }
}

/**
 * Example usage:
 * 
 * import { Agent, AgentConfig } from 'local-agent-core';
 * import { OpenAIProvider } from './custom-llm-provider';
 * 
 * const config: AgentConfig = {
 *   userId: 'user-123',
 *   llmProvider: 'openai',
 *   voiceEnabled: false,
 *   privacyMode: 'strict',
 *   dataRetentionDays: 30,
 *   allowedModules: [],
 * };
 * 
 * const llmProvider = new OpenAIProvider(process.env.OPENAI_API_KEY);
 * const agent = new Agent(config, llmProvider);
 * 
 * const response = await agent.processText('Hello!');
 * console.log(response.text);
 */
