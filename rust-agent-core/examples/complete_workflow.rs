//! Example demonstrating the complete agent workflow
//!
//! This example shows:
//! 1. Memory operations (storing events and habits)
//! 2. Intent generation from user input
//! 3. Policy enforcement
//! 4. Context-aware planning

use rust_agent_core::{
    error::Result,
    habit::HabitAnalyzer,
    intent::IntentGenerator,
    llm::{LlmProvider, MockLlmProvider},
    memory::MemoryStore,
    planner::Planner,
    policy::{Permission, PolicyEngine},
    types::{Event, Habit, HabitFrequency, LlmOptions},
};
use chrono::{Duration, Utc};
use std::collections::HashMap;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    println!("=== Local Agent Core Demo ===\n");

    // 1. Setup memory store
    println!("1. Initializing memory store...");
    let memory = MemoryStore::in_memory()?;
    
    // Store some events
    let event1 = Event::new(
        "user_input".to_string(),
        "User asked about the weather".to_string(),
        0.7,
    );
    memory.store_event(&event1)?;
    
    let event2 = Event::new(
        "agent_response".to_string(),
        "Provided weather information".to_string(),
        0.5,
    );
    memory.store_event(&event2)?;
    
    println!("   Stored {} events", memory.event_count()?);

    // 2. Create and track a habit
    println!("\n2. Creating habit tracking...");
    let habit = Habit::new(
        "Morning exercise".to_string(),
        "Exercise routine every morning".to_string(),
        HabitFrequency::Daily,
    );
    memory.store_habit(&habit)?;
    
    // Simulate some completions
    let completions = vec![
        Utc::now() - Duration::days(3),
        Utc::now() - Duration::days(2),
        Utc::now() - Duration::days(1),
    ];
    
    let analyzer = HabitAnalyzer::new();
    let variance = analyzer.calculate_variance(&completions);
    println!("   Habit: {}", habit.name);
    println!("   Variance: {:?} (lower = more consistent)", variance);

    // 3. LLM interaction
    println!("\n3. Processing user input with LLM...");
    let llm = MockLlmProvider::new();
    let options = LlmOptions::default();
    let response = llm.complete("What's the weather like today?", &options)?;
    println!("   User: What's the weather like today?");
    println!("   Agent: {}", response.text);

    // 4. Intent generation
    println!("\n4. Generating intents from user input...");
    let generator = IntentGenerator::new();
    
    // Example 1: Query intent (no permission required)
    let intents = generator.parse_from_text("what time is it?");
    if let Some(intent) = intents.first() {
        println!("   Intent: {}", intent.intent_type);
        println!("   Confidence: {}", intent.confidence);
        println!("   Requires permission: {}", intent.requires_permission);
        println!("   JSON output:");
        println!("{}", generator.to_json(intent)?);
    }

    // Example 2: Action intent (requires permission)
    println!("\n5. Demonstrating permission-required intent...");
    let mut params = HashMap::new();
    params.insert("device".to_string(), serde_json::json!("living_room_light"));
    params.insert("action".to_string(), serde_json::json!("on"));
    
    let device_intent = generator.generate(
        "device.control".to_string(),
        0.9,
        params,
        "User wants to turn on the living room light".to_string(),
    )?;
    
    println!("   Intent: {}", device_intent.intent_type);
    println!("   Requires permission: {}", device_intent.requires_permission);

    // 6. Policy enforcement
    println!("\n6. Testing policy enforcement...");
    let mut policy = PolicyEngine::new(vec!["device".to_string(), "notification".to_string()]);
    
    // Try without permission
    match policy.check_intent(&device_intent) {
        Ok(_) => println!("   ✗ Intent approved (unexpected)"),
        Err(e) => println!("   ✓ Intent blocked: {}", e),
    }
    
    // Grant permission
    let permission = Permission {
        module: "device".to_string(),
        actions: vec!["device.control".to_string()],
        scope: vec!["living_room".to_string()],
        granted_at: Utc::now(),
        expires_at: Some(Utc::now() + Duration::hours(1)),
    };
    policy.grant_permission(permission);
    
    // Try with permission
    match policy.check_intent(&device_intent) {
        Ok(_) => println!("   ✓ Intent approved with permission"),
        Err(e) => println!("   ✗ Intent blocked: {}", e),
    }

    // 7. Context-aware planning
    println!("\n7. Context-aware planning...");
    let planner = Planner::new();
    
    let recent_events = memory.get_recent_events(5)?;
    let summary = planner.compress_events(&recent_events);
    println!("   {}", summary);
    
    // Evaluate if intent is appropriate
    let (appropriate, reason) = planner.evaluate_intent(
        &device_intent,
        &rust_agent_core::types::Context::new("demo-user".to_string()),
    );
    println!("   Intent appropriate: {}", appropriate);
    println!("   Reason: {}", reason);

    println!("\n=== Demo Complete ===");
    println!("\nKey Takeaways:");
    println!("1. Agent core maintains memory (events, habits) in SQLite");
    println!("2. Statistical habit modeling tracks patterns without judgment");
    println!("3. LLM abstraction allows any provider (local or cloud)");
    println!("4. Intents are structured JSON - agent never executes directly");
    println!("5. Policy engine enforces permissions before any action");
    println!("6. Planner provides context-aware reasoning");
    println!("\nNext step: Send intent JSON to Go device agent for execution");

    Ok(())
}
