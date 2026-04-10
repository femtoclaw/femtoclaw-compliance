use crate::framework::types::TestResult;
use crate::tests;
use crate::reporter;

pub async fn run_all() -> anyhow::Result<()> {
    let mut results = vec![];

    results.extend(tests::protocol::run().await);
    results.extend(tests::runtime::run().await);
    results.extend(tests::capability::run().await);
    results.extend(tests::security::run().await);
    results.extend(tests::performance::run().await);

    reporter::print(&results);
    reporter::write_json(&results)?;

    Ok(())
}

pub async fn run_domain(domain: &str) -> anyhow::Result<()> {
    let results = match domain {
        "protocol" => tests::protocol::run().await,
        "runtime" => tests::runtime::run().await,
        "capability" => tests::capability::run().await,
        "security" => tests::security::run().await,
        "performance" => tests::performance::run().await,
        _ => vec![],
    };

    reporter::print(&results);
    reporter::write_json(&results)?;

    Ok(())
}
