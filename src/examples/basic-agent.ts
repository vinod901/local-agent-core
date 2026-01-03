/**
 * Basic example of using the local agent
 */

import { Agent, MockLLMProvider, MockVoiceOutputProvider, AgentConfig } from '../index';

async function main(): Promise<void> {
  console.log('=== Local Agent Core - Basic Example ===\n');

  // Configure the agent
  const config: AgentConfig = {
    userId: 'demo-user',
    llmProvider: 'mock',
    voiceEnabled: true,
    privacyMode: 'strict',
    dataRetentionDays: 30,
    allowedModules: ['time', 'weather', 'reminder'],
  };

  // Create LLM provider (using mock for demo)
  const llmProvider = new MockLLMProvider();

  // Create voice output provider (optional)
  const voiceOutput = new MockVoiceOutputProvider();

  // Initialize agent
  const agent = new Agent(config, llmProvider, undefined, voiceOutput);

  console.log('Agent initialized with configuration:');
  console.log(JSON.stringify(config, null, 2));
  console.log('\n---\n');

  // Example 1: Simple greeting
  console.log('Example 1: Simple greeting');
  const response1 = await agent.processText('Hello!');
  console.log(`User: Hello!`);
  console.log(`Agent: ${response1.text}`);
  console.log(`Intents detected: ${response1.intents.length}`);
  console.log('\n---\n');

  // Example 2: Query with context
  console.log('Example 2: Time query');
  const response2 = await agent.processText('What time is it?');
  console.log(`User: What time is it?`);
  console.log(`Agent: ${response2.text}`);
  if (response2.intents.length > 0) {
    console.log(`Intents detected:`);
    response2.intents.forEach((intent) => {
      console.log(`  - ${intent.type} (confidence: ${intent.confidence})`);
    });
  }
  console.log('\n---\n');

  // Example 3: Add context
  console.log('Example 3: Adding context');
  const contextStore = agent.getContextStore();
  contextStore.setLocation('home');
  contextStore.setActivity('working');
  contextStore.addHabit({
    name: 'Morning exercise',
    frequency: 'daily',
    schedule: '07:00',
  });

  const context = contextStore.getContext();
  console.log('Current context:');
  console.log(`  Location: ${context.currentLocation}`);
  console.log(`  Activity: ${context.currentActivity}`);
  console.log(`  Active habits: ${context.activeHabits.length}`);
  console.log('\n---\n');

  // Example 4: Query with enriched context
  console.log('Example 4: Query with context awareness');
  const response3 = await agent.processText('Should I take a break?');
  console.log(`User: Should I take a break?`);
  console.log(`Agent: ${response3.text}`);
  console.log('\n---\n');

  // Example 5: View recent events
  console.log('Example 5: Event summarization');
  const summary = contextStore.summarizeEvents(5);
  console.log('Recent events:');
  console.log(summary);
  console.log('\n---\n');

  console.log('Demo complete! All data remained local - nothing was sent externally.');
}

// Run the example
if (require.main === module) {
  main().catch(console.error);
}

export { main };
