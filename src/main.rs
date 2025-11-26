mod api;
mod auth;
mod commands;
mod models;
mod output;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use api::NodanaClient;

#[derive(Parser)]
#[command(name = "ndn", version, about = "Manage Nodana Phoenixd nodes")]
struct Cli {
    /// Keep terminal output plain, without colour, animation or screen clearing
    #[arg(long, global = true)]
    plain: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect the CLI through your browser
    Login,
    /// Show CLI connection status
    Status,
    /// Revoke this CLI session and remove its local credential
    Logout,
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
    output::configure(cli.plain);
    if let Err(error) = run(cli).await {
        output::error(&format!("{error:#}"));
        std::process::exit(1);
    }
    Ok(())
}

async fn run(cli: Cli) -> Result<()> {
    let mut client = NodanaClient::from_env()?;
    match cli.command {
        Commands::Login => auth::login(&mut client).await,
        Commands::Status => auth::status(&mut client).await,
        Commands::Logout => auth::logout(&mut client).await,
        Commands::Node(args) => {
            auth::connect(&mut client).await?;
            commands::nodes::run(&client, args.command).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_commands_match_the_documented_cli() {
        for args in [
            vec!["ndn", "login"],
            vec!["ndn", "status"],
            vec!["ndn", "logout"],
            vec!["ndn", "--plain", "node", "list"],
            vec!["ndn", "node", "list"],
            vec![
                "ndn",
                "node",
                "create",
                "--name",
                "Example",
                "--auto-liquidity",
                "5m",
            ],
            vec!["ndn", "node", "get", "node-123"],
            vec!["ndn", "node", "create", "--full"],
            vec!["ndn", "node", "start", "node-123"],
            vec!["ndn", "node", "stop", "node-123"],
            vec!["ndn", "node", "restart", "node-123"],
            vec!["ndn", "node", "update", "node-123"],
            vec!["ndn", "node", "delete", "node-123", "--force"],
        ] {
            assert!(Cli::try_parse_from(args).is_ok());
        }
        assert!(Cli::try_parse_from(["ndn", "projects", "list"]).is_err());
        assert!(Cli::try_parse_from(["ndn", "node", "create", "--auto-liquidity", "off"]).is_err());
    }
}
