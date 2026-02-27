use crate::framework::types::TestResult;
use femtoclaw_policy::{CapabilityRegistry, Capability};

pub async fn run() -> Vec<TestResult> {
    let mut results = vec![];
    let mut registry = CapabilityRegistry::new();

    // Test: Register shell capability
    let shell_cap = Capability::new("shell", "Execute shell commands");
    registry.register(shell_cap);
    
    results.push(TestResult {
        name: "Capability: Shell tool registered".into(),
        passed: registry.is_registered("shell"),
        message: "Shell capability registered".into(),
    });

    // Test: Register web.get capability
    let web_cap = Capability::new("web.get", "Fetch URLs");
    registry.register(web_cap);
    
    results.push(TestResult {
        name: "Capability: Web get tool registered".into(),
        passed: registry.is_registered("web.get"),
        message: "Web get capability registered".into(),
    });

    // Test: Unknown tool rejected
    results.push(TestResult {
        name: "Capability: Unknown tool rejected".into(),
        passed: !registry.is_registered("unknown_tool"),
        message: "Unknown tool is not registered".into(),
    });

    // Test: Registered tool is enabled
    results.push(TestResult {
        name: "Capability: Registered tool enabled".into(),
        passed: registry.is_enabled("shell"),
        message: "Shell capability is enabled".into(),
    });

    // Test: Disable capability
    registry.disable("shell");
    results.push(TestResult {
        name: "Capability: Disable capability".into(),
        passed: !registry.is_enabled("shell"),
        message: "Shell capability disabled".into(),
    });

    results
}
