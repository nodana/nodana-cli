use crate::api::NodanaClient;
use crate::cli::{Cli, Commands};
use crate::commands;

pub async fn run(cli: Cli, client: &NodanaClient) -> Result<(), anyhow::Error> {
    match cli.command {
        Commands::Project(args) => commands::projects::run(client, args.command).await,
        Commands::App(args) => commands::apps::run(client, args.command).await,
    }
}
