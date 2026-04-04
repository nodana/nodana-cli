use nod::api::NodanaClient;
use nod::cli::Cli;
use nod::run;
use std::env;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let api_key = env::var("NODANA_API_KEY").map_err(|_| {
        anyhow::anyhow!("API key not found. Set NODANA_API_KEY environment variable")
    })?;

    let client = NodanaClient::new(api_key);

    run::run(cli, &client).await
}
