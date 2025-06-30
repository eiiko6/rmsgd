use anyhow::Result;
use clap::Parser;
use rmsgd::cli::{Action, Cli};
use rmsgd::core::server::start_server;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.action {
        Action::Server => {
            start_server().await?;
        }
        Action::Client => {
            eprintln!("Not implemented.");
        }
    }

    Ok(())
}
