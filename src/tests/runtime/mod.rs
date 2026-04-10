use crate::framework::types::TestResult;
use femtoclaw::{Agent, Config};

pub async fn run() -> Vec<TestResult> {
    let mut results = vec![];

    // Test: Agent instantiation
    match Agent::new(Config::default()) {
        Ok(_) => {
            results.push(TestResult {
                name: "Runtime: Agent instantiation".into(),
                passed: true,
                message: "Agent created successfully".into(),
            });
        }
        Err(e) => {
            results.push(TestResult {
                name: "Runtime: Agent instantiation".into(),
                passed: false,
                message: format!("Failed to create agent: {}", e),
            });
        }
    }

    // Test: Agent can process input
    if let Ok(agent) = Agent::new(Config::default()) {
        match agent.run("test input").await {
            Ok(response) => {
                results.push(TestResult {
                    name: "Runtime: Agent processes input".into(),
                    passed: true,
                    message: format!("Agent responded: {}", response),
                });
            }
            Err(e) => {
                results.push(TestResult {
                    name: "Runtime: Agent processes input".into(),
                    passed: false,
                    message: format!("Agent failed: {}", e),
                });
            }
        }
    }

    // Test: Agent history
    if let Ok(agent) = Agent::new(Config::default()) {
        let _ = agent.run("hello").await;
        let history = agent.history().await;
        results.push(TestResult {
            name: "Runtime: Memory management".into(),
            passed: !history.is_empty(),
            message: format!("History has {} messages", history.len()),
        });
    }

    // Test: Agent reset
    if let Ok(agent) = Agent::new(Config::default()) {
        let _ = agent.run("test").await;
        agent.reset().await;
        let history = agent.history().await;
        results.push(TestResult {
            name: "Runtime: Agent reset".into(),
            passed: history.is_empty(),
            message: format!("After reset, history has {} messages", history.len()),
        });
    }

    results
}
