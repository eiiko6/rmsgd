use clap::{Parser, Subcommand};

/// A CLI LAN chat server
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Whether to start the server or the client
    #[command(subcommand)]
    pub action: Action,

    /// Activate verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Start the server
    Server,

    /// Start the client
    Client,
}
