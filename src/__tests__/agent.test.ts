import { Agent, MockLLMProvider, AgentConfig } from '../index';

describe('Agent', () => {
  let agent: Agent;
  let config: AgentConfig;

  beforeEach(() => {
    config = {
      userId: 'test-user',
      llmProvider: 'mock',
      voiceEnabled: false,
      privacyMode: 'strict',
      dataRetentionDays: 30,
      allowedModules: ['test'],
    };

    agent = new Agent(config, new MockLLMProvider());
  });

  describe('processText', () => {
    it('should process text input and return a response', async () => {
      const response = await agent.processText('Hello');

      expect(response).toBeDefined();
      expect(response.text).toBeDefined();
      expect(response.intents).toBeDefined();
      expect(Array.isArray(response.intents)).toBe(true);
      expect(response.timestamp).toBeInstanceOf(Date);
    });

    it('should detect intents from user input', async () => {
      const response = await agent.processText('What time is it?');

      expect(response.intents.length).toBeGreaterThan(0);
      const timeIntent = response.intents.find((i) => i.type === 'time.query');
      expect(timeIntent).toBeDefined();
    });

    it('should log interaction to context store', async () => {
      await agent.processText('Test message');

      const contextStore = agent.getContextStore();
      const events = contextStore.getRecentEvents();

      expect(events.length).toBeGreaterThan(0);
      const userInputEvent = events.find((e) => e.type === 'user_input');
      expect(userInputEvent).toBeDefined();
    });
  });

  describe('context awareness', () => {
    it('should include context in prompts', async () => {
      const contextStore = agent.getContextStore();
      contextStore.setLocation('home');
      contextStore.setActivity('working');

      const response = await agent.processText('Should I take a break?');

      expect(response).toBeDefined();
    });

    it('should track habits', () => {
      const contextStore = agent.getContextStore();
      const habit = contextStore.addHabit({
        name: 'Exercise',
        frequency: 'daily',
      });

      expect(habit.id).toBeDefined();
      expect(habit.name).toBe('Exercise');
      expect(habit.completionCount).toBe(0);

      const habits = contextStore.getActiveHabits();
      expect(habits.length).toBe(1);
    });
  });

  describe('configuration', () => {
    it('should return agent configuration', () => {
      const returnedConfig = agent.getConfig();

      expect(returnedConfig.userId).toBe(config.userId);
      expect(returnedConfig.privacyMode).toBe(config.privacyMode);
    });
  });
});
