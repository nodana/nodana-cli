mod api;
mod commands;
mod models;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use api::NodanaClient;

#[derive(Parser)]
#[command(name = "nod", version, about = "Manage Nodana Phoenixd nodes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and operate Phoenixd nodes
    Node(NodeArgs),
}

#[derive(Args)]
struct NodeArgs {
    #[command(subcommand)]
    command: NodeCommands,
}

#[derive(Subcommand)]
enum NodeCommands {
    /// List nodes in your account
    List,
    /// Create a node and show its one-time credentials
    Create {
        #[arg(long)]
        name: Option<String>,
        #[arg(long, default_value = "2m", value_parser = ["2m", "5m", "10m"])]
        auto_liquidity: String,
        /// Store the full Phoenixd password for dashboard payments
        #[arg(long)]
        full: bool,
    },
    /// Inspect a node
    Get { node_id: String },
    /// Start a stopped node
    Start { node_id: String },
    /// Stop a running node
    Stop { node_id: String },
    /// Restart a running node
    Restart { node_id: String },
    /// Apply the next available Phoenixd update
    Update { node_id: String },
    /// Permanently delete a node
    Delete {
        node_id: String,
        /// Skip the confirmation prompt
        #[arg(long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = NodanaClient::from_env()?;
    match cli.command {
        Commands::Node(args) => commands::nodes::run(&client, args.command).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_commands_match_the_documented_cli() {
        for args in [
            vec!["nod", "node", "list"],
            vec![
                "nod",
                "node",
                "create",
                "--name",
                "Example",
                "--auto-liquidity",
                "5m",
            ],
            vec!["nod", "node", "get", "node-123"],
            vec!["nod", "node", "create", "--full"],
            vec!["nod", "node", "start", "node-123"],
            vec!["nod", "node", "stop", "node-123"],
            vec!["nod", "node", "restart", "node-123"],
            vec!["nod", "node", "update", "node-123"],
            vec!["nod", "node", "delete", "node-123", "--force"],
        ] {
            assert!(Cli::try_parse_from(args).is_ok());
        }
        assert!(Cli::try_parse_from(["nod", "projects", "list"]).is_err());
        assert!(Cli::try_parse_from(["nod", "node", "create", "--auto-liquidity", "off"]).is_err());
    }
}
