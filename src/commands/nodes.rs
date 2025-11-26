use std::io::{self, Write};

use anyhow::{Result, bail};

use crate::{NodeCommands, api::NodanaClient, models::Node, output};

pub async fn run(client: &NodanaClient, command: NodeCommands) -> Result<()> {
    match command {
        NodeCommands::List => {
            let nodes = {
                let _spinner = output::Spinner::new("Loading nodes…");
                client.list_nodes().await?
            };
            if nodes.is_empty() {
                println!("No nodes found.");
            } else {
                output::nodes(&nodes);
            }
        }
        NodeCommands::Create {
            name,
            auto_liquidity,
            full,
        } => {
            let spinner = output::Spinner::new("Creating node…");
            let created = client
                .create_node(name.as_deref(), &auto_liquidity, full)
                .await?;
            drop(spinner);
            output::success("Node creation accepted");
            print_node(&created.node);
            output::heading("One-time node credentials");
            output::field("Admin password", &created.credentials.password);
            output::field(
                "Restricted password",
                &created.credentials.restricted_password,
            );
            output::field("Recovery seed", &created.credentials.seed);
            eprintln!(
                "Save these credentials now. They cannot be retrieved through the Nodana API."
            );
        }
        NodeCommands::Get { node_id } => {
            let spinner = output::Spinner::new("Loading node details…");
            let node = client.get_node(&node_id).await?;
            let update = client.get_node_update(&node_id).await?;
            drop(spinner);
            print_node(&node);
            output::field(
                "Phoenixd version",
                update.current_version.as_deref().unwrap_or("Unknown"),
            );
        }
        NodeCommands::Start { node_id } => action(client, &node_id, "start").await?,
        NodeCommands::Stop { node_id } => action(client, &node_id, "stop").await?,
        NodeCommands::Restart { node_id } => action(client, &node_id, "restart").await?,
        NodeCommands::Update { node_id } => action(client, &node_id, "update").await?,
        NodeCommands::Delete { node_id, force } => {
            if !force && !confirm_delete(&node_id)? {
                println!("Deletion cancelled.");
                return Ok(());
            }
            {
                let _spinner = output::Spinner::new("Deleting node…");
                client.delete_node(&node_id).await?;
            }
            output::success(&format!("Deleted node {}.", output::clean(&node_id)));
        }
    }
    Ok(())
}

fn print_node(node: &Node) {
    output::heading("Node details");
    output::field("ID", &node.id);
    output::field("Name", &node.name);
    output::status_field(&node.status);
    output::field("Endpoint", &node.endpoint_url);
    if let Some(message) = &node.failure_message {
        output::field("Failure", message);
    }
    if let Some(status) = &node.update_status {
        output::field("Update status", status);
    }
}

async fn action(client: &NodanaClient, node_id: &str, action: &str) -> Result<()> {
    let node = {
        let _spinner = output::Spinner::new(&format!("Sending {action} request…"));
        client.node_action(node_id, action).await?
    };
    output::success(&format!("{} request accepted", action));
    print_node(&node);
    Ok(())
}

fn confirm_delete(node_id: &str) -> Result<bool> {
    eprint!(
        "Permanently delete node {}? Type the node ID to confirm: ",
        output::clean(node_id)
    );
    io::stderr().flush()?;
    let mut answer = String::new();
    if io::stdin().read_line(&mut answer)? == 0 {
        bail!("No confirmation received; node was not deleted");
    }
    Ok(answer.trim() == node_id)
}
