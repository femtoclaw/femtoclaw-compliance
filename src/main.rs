use clap::{Parser, Subcommand};
use femtoclaw_compliance::framework::runner;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    RunAll,
    Run { domain: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::RunAll => runner::run_all().await,
        Commands::Run { domain } => runner::run_domain(&domain).await,
    }
}
