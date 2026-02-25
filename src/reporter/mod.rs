use crate::framework::types::TestResult;

pub fn print(results: &[TestResult]) {
    for r in results {
        println!(
            "[{}] {} - {}",
            if r.passed { "PASS" } else { "FAIL" },
            r.name,
            r.message
        );
    }
}

pub fn write_json(results: &[TestResult]) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    std::fs::write("compliance_report.json", json)?;
    Ok(())
}
