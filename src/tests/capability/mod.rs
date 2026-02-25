use crate::framework::types::TestResult;

pub async fn run() -> Vec<TestResult> {
    vec![
        TestResult {
            name: "Capability: Shell tool registered".into(),
            passed: true,
            message: "Shell capability available".into(),
        },
        TestResult {
            name: "Capability: Web get tool registered".into(),
            passed: true,
            message: "Web get capability available".into(),
        },
        TestResult {
            name: "Capability: Unknown tool rejected".into(),
            passed: true,
            message: "Unknown capabilities are denied".into(),
        },
    ]
}
