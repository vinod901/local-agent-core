//! Memory module - SQLite-based persistent storage
//!
//! This module implements the memory layer for the agent:
//! - Stores events, habits, and context
//! - Supports querying and summarization
//! - Privacy-first: all data stays local
//! - SQL-first: auditable and deterministic

use crate::error::{AgentError, Result};
use crate::types::{Event, Habit, HabitFrequency};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::Uuid;

pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    /// Create a new memory store with the given database path
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let store = Self { conn };
        store.initialize_schema()?;
        Ok(store)
    }

    /// Create an in-memory database for testing
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.initialize_schema()?;
        Ok(store)
    }

    /// Initialize database schema
    fn initialize_schema(&self) -> Result<()> {
        // Events table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                event_type TEXT NOT NULL,
                description TEXT NOT NULL,
                importance REAL NOT NULL,
                metadata TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        // Habits table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS habits (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                frequency TEXT NOT NULL,
                schedule TEXT,
                completion_count INTEGER NOT NULL,
                last_completed TEXT,
                created_at TEXT NOT NULL,
                variance REAL
            )",
            [],
        )?;

        // Habit completions table (for statistical tracking)
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS habit_completions (
                id TEXT PRIMARY KEY,
                habit_id TEXT NOT NULL,
                completed_at TEXT NOT NULL,
                FOREIGN KEY(habit_id) REFERENCES habits(id)
            )",
            [],
        )?;

        // Create indices for common queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type)",
            [],
        )?;

        Ok(())
    }

    /// Store an event
    pub fn store_event(&self, event: &Event) -> Result<()> {
        let metadata_json = serde_json::to_string(&event.metadata)?;
        
        self.conn.execute(
            "INSERT INTO events (id, event_type, description, importance, metadata, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                event.id.to_string(),
                event.event_type,
                event.description,
                event.importance,
                metadata_json,
                event.timestamp.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    /// Get recent events
    pub fn get_recent_events(&self, limit: u32) -> Result<Vec<Event>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, event_type, description, importance, metadata, timestamp
             FROM events
             ORDER BY timestamp DESC
             LIMIT ?1",
        )?;

        let events = stmt
            .query_map([limit], |row| {
                let id: String = row.get(0)?;
                let metadata_json: String = row.get(4)?;
                let timestamp_str: String = row.get(5)?;

                Ok(Event {
                    id: Uuid::parse_str(&id).unwrap(),
                    event_type: row.get(1)?,
                    description: row.get(2)?,
                    importance: row.get(3)?,
                    metadata: serde_json::from_str(&metadata_json).unwrap_or_default(),
                    timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                        .unwrap()
                        .with_timezone(&Utc),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(events)
    }

    /// Get events by type
    pub fn get_events_by_type(&self, event_type: &str, limit: u32) -> Result<Vec<Event>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, event_type, description, importance, metadata, timestamp
             FROM events
             WHERE event_type = ?1
             ORDER BY timestamp DESC
             LIMIT ?2",
        )?;

        let events = stmt
            .query_map(params![event_type, limit], |row| {
                let id: String = row.get(0)?;
                let metadata_json: String = row.get(4)?;
                let timestamp_str: String = row.get(5)?;

                Ok(Event {
                    id: Uuid::parse_str(&id).unwrap(),
                    event_type: row.get(1)?,
                    description: row.get(2)?,
                    importance: row.get(3)?,
                    metadata: serde_json::from_str(&metadata_json).unwrap_or_default(),
                    timestamp: DateTime::parse_from_rfc3339(&timestamp_str)
                        .unwrap()
                        .with_timezone(&Utc),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(events)
    }

    /// Store a habit
    pub fn store_habit(&self, habit: &Habit) -> Result<()> {
        let frequency_str = match &habit.frequency {
            HabitFrequency::Daily => "daily".to_string(),
            HabitFrequency::Weekly => "weekly".to_string(),
            HabitFrequency::Monthly => "monthly".to_string(),
            HabitFrequency::Custom(s) => format!("custom:{}", s),
        };

        self.conn.execute(
            "INSERT OR REPLACE INTO habits 
             (id, name, description, frequency, schedule, completion_count, last_completed, created_at, variance)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                habit.id.to_string(),
                habit.name,
                habit.description,
                frequency_str,
                habit.schedule,
                habit.completion_count,
                habit.last_completed.map(|dt| dt.to_rfc3339()),
                habit.created_at.to_rfc3339(),
                habit.variance,
            ],
        )?;

        Ok(())
    }

    /// Get all active habits
    pub fn get_active_habits(&self) -> Result<Vec<Habit>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, frequency, schedule, completion_count, 
                    last_completed, created_at, variance
             FROM habits
             ORDER BY created_at DESC",
        )?;

        let habits = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let frequency_str: String = row.get(3)?;
                let last_completed_str: Option<String> = row.get(6)?;
                let created_at_str: String = row.get(7)?;

                let frequency = if frequency_str == "daily" {
                    HabitFrequency::Daily
                } else if frequency_str == "weekly" {
                    HabitFrequency::Weekly
                } else if frequency_str == "monthly" {
                    HabitFrequency::Monthly
                } else if let Some(custom) = frequency_str.strip_prefix("custom:") {
                    HabitFrequency::Custom(custom.to_string())
                } else {
                    HabitFrequency::Custom(frequency_str)
                };

                Ok(Habit {
                    id: Uuid::parse_str(&id).unwrap(),
                    name: row.get(1)?,
                    description: row.get(2)?,
                    frequency,
                    schedule: row.get(4)?,
                    completion_count: row.get(5)?,
                    last_completed: last_completed_str.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                    }),
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)
                        .unwrap()
                        .with_timezone(&Utc),
                    variance: row.get(8)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(habits)
    }

    /// Record habit completion
    pub fn complete_habit(&self, habit_id: Uuid) -> Result<()> {
        let completion_id = Uuid::new_v4();
        let now = Utc::now();

        // Insert completion record
        self.conn.execute(
            "INSERT INTO habit_completions (id, habit_id, completed_at)
             VALUES (?1, ?2, ?3)",
            params![
                completion_id.to_string(),
                habit_id.to_string(),
                now.to_rfc3339(),
            ],
        )?;

        // Update habit
        self.conn.execute(
            "UPDATE habits 
             SET completion_count = completion_count + 1,
                 last_completed = ?1
             WHERE id = ?2",
            params![now.to_rfc3339(), habit_id.to_string()],
        )?;

        Ok(())
    }

    /// Clear old events (privacy/retention policy)
    pub fn clear_events_before(&self, before: DateTime<Utc>) -> Result<usize> {
        let deleted = self.conn.execute(
            "DELETE FROM events WHERE timestamp < ?1",
            params![before.to_rfc3339()],
        )?;

        Ok(deleted)
    }

    /// Get event count
    pub fn event_count(&self) -> Result<usize> {
        let count: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))?;
        Ok(count)
    }

    /// Get habit count
    pub fn habit_count(&self) -> Result<usize> {
        let count: usize = self
            .conn
            .query_row("SELECT COUNT(*) FROM habits", [], |row| row.get(0))?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_store_creation() {
        let store = MemoryStore::in_memory().unwrap();
        assert_eq!(store.event_count().unwrap(), 0);
        assert_eq!(store.habit_count().unwrap(), 0);
    }

    #[test]
    fn test_store_and_retrieve_event() {
        let store = MemoryStore::in_memory().unwrap();
        let event = Event::new("test".to_string(), "test event".to_string(), 0.5);

        store.store_event(&event).unwrap();
        assert_eq!(store.event_count().unwrap(), 1);

        let events = store.get_recent_events(10).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "test");
    }

    #[test]
    fn test_store_and_retrieve_habit() {
        let store = MemoryStore::in_memory().unwrap();
        let habit = Habit::new(
            "Morning exercise".to_string(),
            "Exercise every morning".to_string(),
            HabitFrequency::Daily,
        );

        store.store_habit(&habit).unwrap();
        assert_eq!(store.habit_count().unwrap(), 1);

        let habits = store.get_active_habits().unwrap();
        assert_eq!(habits.len(), 1);
        assert_eq!(habits[0].name, "Morning exercise");
    }

    #[test]
    fn test_complete_habit() {
        let store = MemoryStore::in_memory().unwrap();
        let mut habit = Habit::new(
            "Morning exercise".to_string(),
            "Exercise every morning".to_string(),
            HabitFrequency::Daily,
        );

        store.store_habit(&habit).unwrap();
        store.complete_habit(habit.id).unwrap();

        let habits = store.get_active_habits().unwrap();
        assert_eq!(habits[0].completion_count, 1);
        assert!(habits[0].last_completed.is_some());
    }
}
