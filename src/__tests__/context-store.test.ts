import { ContextStore } from '../context/store';

describe('ContextStore', () => {
  let contextStore: ContextStore;

  beforeEach(() => {
    contextStore = new ContextStore('test-user');
  });

  describe('event management', () => {
    it('should add events', () => {
      const event = contextStore.addEvent({
        type: 'test_event',
        description: 'Test description',
        metadata: { key: 'value' },
      });

      expect(event.id).toBeDefined();
      expect(event.type).toBe('test_event');
      expect(event.timestamp).toBeInstanceOf(Date);
    });

    it('should retrieve recent events', () => {
      contextStore.addEvent({
        type: 'event1',
        description: 'First event',
        metadata: {},
      });

      contextStore.addEvent({
        type: 'event2',
        description: 'Second event',
        metadata: {},
      });

      const events = contextStore.getRecentEvents(2);
      expect(events.length).toBe(2);
    });

    it('should filter events by type', () => {
      contextStore.addEvent({
        type: 'user_input',
        description: 'Input 1',
        metadata: {},
      });

      contextStore.addEvent({
        type: 'agent_response',
        description: 'Response 1',
        metadata: {},
      });

      contextStore.addEvent({
        type: 'user_input',
        description: 'Input 2',
        metadata: {},
      });

      const userInputs = contextStore.getEventsByType('user_input');
      expect(userInputs.length).toBe(2);
    });

    it('should limit event history', () => {
      const store = new ContextStore('test-user', 5);

      for (let i = 0; i < 10; i++) {
        store.addEvent({
          type: 'test',
          description: `Event ${i}`,
          metadata: {},
        });
      }

      const events = store.getRecentEvents(100);
      expect(events.length).toBe(5);
    });
  });

  describe('habit tracking', () => {
    it('should add habits', () => {
      const habit = contextStore.addHabit({
        name: 'Morning exercise',
        frequency: 'daily',
      });

      expect(habit.id).toBeDefined();
      expect(habit.name).toBe('Morning exercise');
      expect(habit.completionCount).toBe(0);
      expect(habit.createdAt).toBeInstanceOf(Date);
    });

    it('should complete habits', () => {
      const habit = contextStore.addHabit({
        name: 'Reading',
        frequency: 'daily',
      });

      contextStore.completeHabit(habit.id);

      const habits = contextStore.getActiveHabits();
      const updated = habits.find((h) => h.id === habit.id);

      expect(updated?.completionCount).toBe(1);
      expect(updated?.lastCompleted).toBeInstanceOf(Date);
    });

    it('should retrieve active habits', () => {
      contextStore.addHabit({
        name: 'Exercise',
        frequency: 'daily',
      });

      contextStore.addHabit({
        name: 'Meditation',
        frequency: 'daily',
      });

      const habits = contextStore.getActiveHabits();
      expect(habits.length).toBe(2);
    });
  });

  describe('context management', () => {
    it('should set and get location', () => {
      contextStore.setLocation('home');
      const context = contextStore.getContext();

      expect(context.currentLocation).toBe('home');
    });

    it('should set and get activity', () => {
      contextStore.setActivity('working');
      const context = contextStore.getContext();

      expect(context.currentActivity).toBe('working');
    });

    it('should get full context', () => {
      contextStore.setLocation('office');
      contextStore.setActivity('meeting');
      contextStore.addEvent({
        type: 'test',
        description: 'Test event',
        metadata: {},
      });

      const context = contextStore.getContext();

      expect(context.userId).toBe('test-user');
      expect(context.currentLocation).toBe('office');
      expect(context.currentActivity).toBe('meeting');
      expect(context.recentEvents.length).toBeGreaterThan(0);
      expect(context.timestamp).toBeInstanceOf(Date);
    });
  });

  describe('privacy features', () => {
    it('should clear all context data', () => {
      contextStore.setLocation('home');
      contextStore.setActivity('working');
      contextStore.addEvent({
        type: 'test',
        description: 'Test',
        metadata: {},
      });
      contextStore.addHabit({
        name: 'Test habit',
        frequency: 'daily',
      });

      contextStore.clear();

      const context = contextStore.getContext();
      expect(context.currentLocation).toBeUndefined();
      expect(context.currentActivity).toBeUndefined();
      expect(context.recentEvents.length).toBe(0);
      expect(context.activeHabits.length).toBe(0);
    });
  });

  describe('event summarization', () => {
    it('should summarize events', () => {
      contextStore.addEvent({
        type: 'user_input',
        description: 'Hello',
        metadata: {},
      });

      contextStore.addEvent({
        type: 'agent_response',
        description: 'Hi there!',
        metadata: {},
      });

      const summary = contextStore.summarizeEvents();
      expect(summary).toContain('user_input');
      expect(summary).toContain('agent_response');
    });

    it('should return message for empty events', () => {
      const summary = contextStore.summarizeEvents();
      expect(summary).toBe('No recent events.');
    });
  });
});
