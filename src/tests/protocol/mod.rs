use crate::framework::types::TestResult;

pub async fn run() -> Vec<TestResult> {
    let mut results = vec![];
    
    // Test: Valid message form
    let msg = r#"{"message":{"content":"hello"}}"#;
    if let Ok(_) = parse_strict(msg) {
        results.push(TestResult {
            name: "Protocol: Valid message form".into(),
            passed: true,
            message: "Valid message accepted".into(),
        });
    } else {
        results.push(TestResult {
            name: "Protocol: Valid message form".into(),
            passed: false,
            message: "Valid message rejected".into(),
        });
    }

    // Test: Valid tool call form
    let tc = r#"{"tool_call":{"tool":"test","args":{}}}"#;
    if let Ok(_) = parse_strict(tc) {
        results.push(TestResult {
            name: "Protocol: Valid tool_call form".into(),
            passed: true,
            message: "Valid tool_call accepted".into(),
        });
    } else {
        results.push(TestResult {
            name: "Protocol: Valid tool_call form".into(),
            passed: false,
            message: "Valid tool_call rejected".into(),
        });
    }

    // Test: Reject both forms
    let both = r#"{"message":{"content":"x"},"tool_call":{"tool":"y","args":{}}}"#;
    if let Err(_) = parse_strict(both) {
        results.push(TestResult {
            name: "Protocol: Reject ambiguous (both)".into(),
            passed: true,
            message: "Ambiguous protocol rejected".into(),
        });
    } else {
        results.push(TestResult {
            name: "Protocol: Reject ambiguous (both)".into(),
            passed: false,
            message: "Ambiguous protocol accepted".into(),
        });
    }

    // Test: Reject neither form
    let neither = r#"{"unknown":{}}"#;
    if let Err(_) = parse_strict(neither) {
        results.push(TestResult {
            name: "Protocol: Reject invalid structure".into(),
            passed: true,
            message: "Invalid structure rejected".into(),
        });
    } else {
        results.push(TestResult {
            name: "Protocol: Reject invalid structure".into(),
            passed: false,
            message: "Invalid structure accepted".into(),
        });
    }

    results
}

fn parse_strict(s: &str) -> Result<(), String> {
    let v: serde_json::Value = serde_json::from_str(s)
        .map_err(|e| format!("invalid json: {}", e))?;

    let has_msg = v.get("message").is_some();
    let has_tool = v.get("tool_call").is_some();
    
    if has_msg == has_tool {
        return Err("must contain exactly one of message or tool_call".into());
    }

    if let Some(m) = v.get("message") {
        if m.get("content").and_then(|x| x.as_str()).is_none() {
            return Err("message.content missing".into());
        }
        return Ok(());
    }

    let tc = v.get("tool_call").ok_or("tool_call missing")?;
    if tc.get("tool").and_then(|x| x.as_str()).is_none() {
        return Err("tool_call.tool missing".into());
    }

    Ok(())
}
