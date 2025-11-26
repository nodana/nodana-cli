use std::io::{self, Write};

use anyhow::{Result, bail};

use crate::{NodeCommands, api::NodanaClient, models::Node};

pub async fn run(client: &NodanaClient, command: NodeCommands) -> Result<()> {
    match command {
        NodeCommands::List => {
            let nodes = client.list_nodes().await?;
            if nodes.is_empty() {
                println!("No nodes found.");
            } else {
                println!("{:<24} {:<18} STATUS", "ID", "NAME");
                for node in nodes {
                    println!("{:<24} {:<18} {}", node.id, node.name, node.status);
                }
            }
        }
        NodeCommands::Create {
            name,
            auto_liquidity,
            full,
        } => {
            let created = client
                .create_node(name.as_deref(), &auto_liquidity, full)
                .await?;
            print_node(&created.node);
            println!("Phoenixd admin password: {}", created.credentials.password);
            println!(
                "Phoenixd restricted password: {}",
                created.credentials.restricted_password
            );
            println!("Recovery seed: {}", created.credentials.seed);
            eprintln!(
                "Save these credentials now. They cannot be retrieved through the Nodana API."
            );
        }
        NodeCommands::Get { node_id } => {
            let node = client.get_node(&node_id).await?;
            let update = client.get_node_update(&node_id).await?;
            print_node(&node);
            println!(
                "Phoenixd version: {}",
                update.current_version.as_deref().unwrap_or("Unknown")
            );
        }
        NodeCommands::Start { node_id } => {
            print_node(&client.node_action(&node_id, "start").await?)
        }
        NodeCommands::Stop { node_id } => print_node(&client.node_action(&node_id, "stop").await?),
        NodeCommands::Restart { node_id } => {
            print_node(&client.node_action(&node_id, "restart").await?)
        }
        NodeCommands::Update { node_id } => {
            print_node(&client.node_action(&node_id, "update").await?)
        }
        NodeCommands::Delete { node_id, force } => {
            if !force && !confirm_delete(&node_id)? {
                println!("Deletion cancelled.");
                return Ok(());
            }
            client.delete_node(&node_id).await?;
            println!("Deleted node {node_id}.");
        }
    }
    Ok(())
}

fn print_node(node: &Node) {
    println!("ID: {}", node.id);
    println!("Name: {}", node.name);
    println!("Status: {}", node.status);
    println!("Endpoint: {}", node.endpoint_url);
    if let Some(message) = &node.failure_message {
        println!("Failure: {message}");
    }
    if let Some(status) = &node.update_status {
        println!("Update status: {status}");
    }
}

fn confirm_delete(node_id: &str) -> Result<bool> {
    eprint!("Permanently delete node {node_id}? Type the node ID to confirm: ");
    io::stderr().flush()?;
    let mut answer = String::new();
    if io::stdin().read_line(&mut answer)? == 0 {
        bail!("No confirmation received; node was not deleted");
    }
    Ok(answer.trim() == node_id)
}
