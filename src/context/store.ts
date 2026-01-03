import { Context, ContextEvent, Habit } from '../core/types';

/**
 * Context store for managing user context and history
 * Privacy-first: data stored locally, never shared without permission
 */
export class ContextStore {
  private userId: string;
  private events: ContextEvent[] = [];
  private habits: Map<string, Habit> = new Map();
  private currentLocation?: string;
  private currentActivity?: string;
  private maxEvents: number;

  constructor(userId: string, maxEvents = 1000) {
    this.userId = userId;
    this.maxEvents = maxEvents;
  }

  /**
   * Add a context event
   */
  addEvent(event: Omit<ContextEvent, 'id' | 'timestamp'>): ContextEvent {
    const contextEvent: ContextEvent = {
      id: this.generateId(),
      ...event,
      timestamp: new Date(),
    };

    this.events.push(contextEvent);

    // Limit event history for privacy and performance
    if (this.events.length > this.maxEvents) {
      this.events.shift();
    }

    return contextEvent;
  }

  /**
   * Get recent events
   */
  getRecentEvents(limit = 10): ContextEvent[] {
    return this.events.slice(-limit);
  }

  /**
   * Search events by type
   */
  getEventsByType(type: string, limit = 10): ContextEvent[] {
    return this.events.filter((e) => e.type === type).slice(-limit);
  }

  /**
   * Add or update a habit
   */
  addHabit(habit: Omit<Habit, 'id' | 'createdAt' | 'completionCount'>): Habit {
    const newHabit: Habit = {
      id: this.generateId(),
      ...habit,
      completionCount: 0,
      createdAt: new Date(),
    };

    this.habits.set(newHabit.id, newHabit);
    return newHabit;
  }

  /**
   * Mark habit as completed
   */
  completeHabit(habitId: string): void {
    const habit = this.habits.get(habitId);
    if (habit) {
      habit.lastCompleted = new Date();
      habit.completionCount++;
    }
  }

  /**
   * Get all active habits
   */
  getActiveHabits(): Habit[] {
    return Array.from(this.habits.values());
  }

  /**
   * Update current location
   */
  setLocation(location: string): void {
    this.currentLocation = location;
  }

  /**
   * Update current activity
   */
  setActivity(activity: string): void {
    this.currentActivity = activity;
  }

  /**
   * Get current context
   */
  getContext(): Context {
    return {
      userId: this.userId,
      currentLocation: this.currentLocation,
      currentActivity: this.currentActivity,
      recentEvents: this.getRecentEvents(),
      activeHabits: this.getActiveHabits(),
      timestamp: new Date(),
    };
  }

  /**
   * Clear all context data (for privacy)
   */
  clear(): void {
    this.events = [];
    this.habits.clear();
    this.currentLocation = undefined;
    this.currentActivity = undefined;
  }

  /**
   * Summarize recent events
   */
  summarizeEvents(limit = 10): string {
    const recent = this.getRecentEvents(limit);
    if (recent.length === 0) {
      return 'No recent events.';
    }

    return recent.map((e) => `- ${e.type}: ${e.description} (${e.timestamp.toLocaleString()})`).join('\n');
  }

  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }
}
