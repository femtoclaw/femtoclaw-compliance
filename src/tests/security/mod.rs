use crate::framework::types::TestResult;
use femtoclaw::{Agent, Config};
use femtoclaw_protocol::Validator;

pub async fn run() -> Vec<TestResult> {
    let mut results = vec![];

    // Test: Protocol validation enforced
    let validator = Validator::new();
    let valid_msg = r#"{"message":{"content":"hello"}}"#;
    match validator.validate_str(valid_msg) {
        Ok(_) => {
            results.push(TestResult {
                name: "Security: Protocol validation accepts valid".into(),
                passed: true,
                message: "Valid protocol accepted".into(),
            });
        }
        Err(e) => {
            results.push(TestResult {
                name: "Security: Protocol validation accepts valid".into(),
                passed: false,
                message: format!("Valid rejected: {}", e),
            });
        }
    }

    // Test: Protocol rejects invalid
    let invalid_msg = r#"{"unknown":{}}"#;
    match validator.validate_str(invalid_msg) {
        Err(_) => {
            results.push(TestResult {
                name: "Security: Protocol validation rejects invalid".into(),
                passed: true,
                message: "Invalid protocol rejected".into(),
            });
        }
        Ok(_) => {
            results.push(TestResult {
                name: "Security: Protocol validation rejects invalid".into(),
                passed: false,
                message: "Invalid protocol was accepted".into(),
            });
        }
    }

    // Test: Protocol rejects ambiguous (both message and tool_call)
    let ambiguous = r#"{"message":{"content":"x"},"tool_call":{"tool":"y","args":{}}}"#;
    match validator.validate_str(ambiguous) {
        Err(_) => {
            results.push(TestResult {
                name: "Security: Protocol rejects ambiguous".into(),
                passed: true,
                message: "Ambiguous protocol rejected".into(),
            });
        }
        Ok(_) => {
            results.push(TestResult {
                name: "Security: Protocol rejects ambiguous".into(),
                passed: false,
                message: "Ambiguous protocol was accepted".into(),
            });
        }
    }

    // Test: Capability gating enforced
    if let Ok(agent) = Agent::new(Config::default()) {
        // Try to run an unknown tool through the agent
        let result = agent.run(r#"{"tool_call":{"tool":"unknown_tool","args":{}}}"#).await;
        match result {
            Err(_) => {
                results.push(TestResult {
                    name: "Security: Capability gating enforced".into(),
                    passed: true,
                    message: "Unknown capability denied".into(),
                });
            }
            Ok(resp) => {
                results.push(TestResult {
                    name: "Security: Capability gating enforced".into(),
                    passed: false,
                    message: format!("Unknown capability executed: {}", resp),
                });
            }
        }
    }

    results
}
