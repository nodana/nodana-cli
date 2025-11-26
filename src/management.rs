//! Nodana account operations, authenticated independently of Phoenixd.
use std::io::{self, IsTerminal, Write};

use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use reqwest::{Method, Url};
use serde_json::{Value, json};

#[derive(Subcommand)]
pub enum Command {
    /// List hosted nodes in your account
    ListNodes,
    /// Create a node; prints one-time passwords and recovery seed
    CreateNode {
        #[arg(long, value_parser = node_name)]
        name: Option<String>,
        #[arg(long, default_value = "2m", value_parser = ["2m", "5m", "10m"])]
        auto_liquidity: String,
    },
    /// Inspect a hosted node
    GetNode {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Change a hosted node's display name
    RenameNode {
        #[arg(value_parser = identifier)]
        node_id: String,
        #[arg(long, value_parser = node_name)]
        name: String,
    },
    /// Start a stopped node
    StartNode {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Stop a node, retaining its data
    StopNode {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Restart a node
    RestartNode {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Check for an available node release
    CheckNodeUpdate {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Update a node to the selected release
    UpdateNode {
        #[arg(value_parser = identifier)]
        node_id: String,
        #[arg(long, value_parser = checksum)]
        release_sha256: String,
    },
    /// Permanently delete a node; save recovery information and handle funds first
    DeleteNode {
        #[arg(value_parser = identifier)]
        node_id: String,
        /// Skip the node ID confirmation prompt
        #[arg(long)]
        force: bool,
    },
    /// List a node's webhook receivers (currently zero or one)
    ListWebhooks {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Read a node's webhook settings
    GetWebhook {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// Create a receiver; prints its one-time signing secret
    CreateWebhook {
        #[arg(value_parser = identifier)]
        node_id: String,
        #[arg(long, value_parser = webhook_url)]
        url: String,
        #[arg(long, value_parser = description)]
        description: Option<String>,
    },
    /// Change receiver settings; omitted fields keep their values
    UpdateWebhook {
        #[arg(value_parser = identifier)]
        node_id: String,
        #[arg(long, value_parser = webhook_url)]
        url: Option<String>,
        /// Use an empty string to clear the description
        #[arg(long, value_parser = description)]
        description: Option<String>,
        #[arg(long, action = clap::ArgAction::Set)]
        enabled: Option<bool>,
    },
    /// Delete the receiver and its delivery history
    DeleteWebhook {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
    /// List the latest 50 webhook deliveries
    ListWebhookDeliveries {
        #[arg(value_parser = identifier)]
        node_id: String,
    },
}

struct Request {
    method: Method,
    path: String,
    body: Option<Value>,
}

impl Command {
    fn request(&self) -> Result<Request> {
        use Method as M;
        let (method, path, body) = match self {
            Self::ListNodes => (M::GET, "nodes".into(), None),
            Self::CreateNode {
                name,
                auto_liquidity,
            } => {
                let mut id = [0; 16];
                openssl::rand::rand_bytes(&mut id)
                    .context("Could not generate a creation request ID")?;
                let mut body =
                    json!({"requestId": hex::encode(id), "autoLiquidity": auto_liquidity});
                if let Some(name) = name {
                    body["name"] = json!(name);
                }
                (M::POST, "nodes".into(), Some(body))
            }
            Self::GetNode { node_id } => (M::GET, format!("nodes/{node_id}"), None),
            Self::RenameNode { node_id, name } => (
                M::PATCH,
                format!("nodes/{node_id}"),
                Some(json!({"name": name})),
            ),
            Self::StartNode { node_id } => (M::POST, format!("nodes/{node_id}/start"), None),
            Self::StopNode { node_id } => (M::POST, format!("nodes/{node_id}/stop"), None),
            Self::RestartNode { node_id } => (M::POST, format!("nodes/{node_id}/restart"), None),
            Self::CheckNodeUpdate { node_id } => (M::GET, format!("nodes/{node_id}/update"), None),
            Self::UpdateNode {
                node_id,
                release_sha256,
            } => (
                M::POST,
                format!("nodes/{node_id}/update"),
                Some(json!({"releaseSha256": release_sha256})),
            ),
            Self::DeleteNode { node_id, .. } => (M::DELETE, format!("nodes/{node_id}"), None),
            Self::ListWebhooks { node_id } | Self::GetWebhook { node_id } => {
                (M::GET, format!("nodes/{node_id}/webhook-endpoint"), None)
            }
            Self::CreateWebhook {
                node_id,
                url,
                description,
            } => {
                let mut body = json!({"url": url});
                if let Some(value) = description {
                    body["description"] = json!(value);
                }
                (
                    M::POST,
                    format!("nodes/{node_id}/webhook-endpoint"),
                    Some(body),
                )
            }
            Self::UpdateWebhook {
                node_id,
                url,
                description,
                enabled,
            } => {
                let mut fields = serde_json::Map::new();
                if let Some(value) = url {
                    fields.insert("url".into(), json!(value));
                }
                if let Some(value) = description {
                    fields.insert("description".into(), json!(value));
                }
                if let Some(value) = enabled {
                    fields.insert("enabled".into(), json!(value));
                }
                ensure!(
                    !fields.is_empty(),
                    "Supply --url, --description or --enabled"
                );
                (
                    M::PATCH,
                    format!("nodes/{node_id}/webhook-endpoint"),
                    Some(Value::Object(fields)),
                )
            }
            Self::DeleteWebhook { node_id } => {
                (M::DELETE, format!("nodes/{node_id}/webhook-endpoint"), None)
            }
            Self::ListWebhookDeliveries { node_id } => {
                (M::GET, format!("nodes/{node_id}/webhook-deliveries"), None)
            }
        };
        Ok(Request { method, path, body })
    }
}

pub async fn run(command: Command, origin: Option<&str>) -> Result<()> {
    let request = command.request()?;
    let origin = origin
        .map(str::to_owned)
        .or_else(|| std::env::var("NODANA_API_ORIGIN").ok())
        .unwrap_or_else(|| "https://api.nodana.io".into());
    let base = api_origin(&origin)?;
    let key = std::env::var("NODANA_API_KEY")
        .context("Set NODANA_API_KEY to a Nodana API key with the required node permissions")?;
    ensure!(
        !key.is_empty() && !key.chars().any(char::is_whitespace),
        "NODANA_API_KEY is empty or invalid"
    );
    if let Command::DeleteNode {
        node_id,
        force: false,
    } = &command
    {
        confirm_delete(node_id)?;
    }
    let client = crate::api::http_client()?;
    let creating = matches!(command, Command::CreateNode { .. });
    let mut builder = client
        .request(request.method, base.join(&request.path)?)
        .bearer_auth(key);
    if let Some(body) = request.body {
        builder = builder.json(&body);
    }
    // Never automatically repeat a mutation. Creation returns its credentials only once.
    let response = builder.send().await.map_err(|_| anyhow::anyhow!(
        if creating {
            "Could not complete the creation request. Check nd list-nodes before trying again; the node may already exist."
        } else {
            "Could not reach the Nodana API. Check your connection and inspect the node or webhook before repeating a change."
        }
    ))?;
    check_status(response.status())?;
    let bytes = crate::api::read_limited(response, 4 * 1024 * 1024)
        .await
        .with_context(|| {
            if creating {
                "Creation response was interrupted. Check nd list-nodes before trying again; the node may already exist."
            } else {
                "Could not read the Nodana response"
            }
        })?;
    ensure!(
        !bytes.is_empty()
            || !matches!(
                command,
                Command::CreateNode { .. } | Command::CreateWebhook { .. }
            ),
        "Nodana returned no creation details. Inspect the node or webhook before repeating the request"
    );
    let result = if bytes.is_empty() {
        json!({"accepted": true})
    } else {
        serde_json::from_slice(&bytes).context("Nodana returned invalid JSON")?
    };
    let result = if matches!(command, Command::ListWebhooks { .. }) {
        webhook_list(result)?
    } else {
        result
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    if creating {
        eprintln!(
            "Save the credentials now; they are returned only once. Use nd get-node <node-id> to check when provisioning is ready."
        );
    } else if matches!(command, Command::CreateWebhook { .. }) {
        eprintln!(
            "Save signingSecret in your receiver's secret settings; it is returned only once."
        );
    }
    Ok(())
}

fn api_origin(value: &str) -> Result<Url> {
    let mut url = Url::parse(value).context("Invalid Nodana API origin")?;
    ensure!(
        url.host_str().is_some()
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
            && url.path() == "/"
            && (url.scheme() == "https"
                || (url.scheme() == "http"
                    && matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]")))),
        "Use an HTTPS Nodana API origin without credentials, path, query or fragment"
    );
    url.set_path("/v1/");
    Ok(url)
}

fn check_status(status: reqwest::StatusCode) -> Result<()> {
    if status.is_success() {
        return Ok(());
    }
    match status.as_u16() {
        401 => bail!(
            "Nodana rejected the API key. Check NODANA_API_KEY and replace expired or revoked keys"
        ),
        402 => {
            bail!("Add account credit in the Nodana dashboard before creating or starting a node")
        }
        403 => bail!(
            "Nodana denied access. Check the API key's nodes:read or nodes:write permissions and account"
        ),
        404 => bail!("Node or webhook not found in the API key's account"),
        409 => bail!("Nodana reported a conflict. Inspect the node or webhook before trying again"),
        429 => bail!("Nodana rate limit reached. Wait before trying again"),
        503 => bail!("Nodana is temporarily unavailable or node capacity is full. Try again later"),
        _ => bail!("Nodana request failed with HTTP {status}"),
    }
}

fn webhook_list(value: Value) -> Result<Value> {
    match value.get("endpoint") {
        Some(Value::Null) => Ok(json!([])),
        Some(endpoint @ Value::Object(_)) => Ok(json!([endpoint])),
        _ => bail!("Invalid webhook response from Nodana"),
    }
}

fn confirm_delete(node: &str) -> Result<()> {
    ensure!(
        io::stdin().is_terminal(),
        "Node deletion requires confirmation. Use --force to skip the node ID prompt"
    );
    eprint!("Permanently delete {node}? Type its node ID to confirm: ");
    io::stderr().flush()?;
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    ensure!(answer.trim() == node, "Deletion cancelled");
    Ok(())
}

fn identifier(value: &str) -> Result<String, String> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
    {
        return Err("Use the node ID returned by Nodana".into());
    }
    Ok(value.into())
}
fn node_name(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > 16 {
        return Err("Use a name of 1–16 characters".into());
    }
    Ok(value.into())
}
fn description(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.chars().count() > 500 {
        return Err("Description must be at most 500 characters".into());
    }
    Ok(value.into())
}
fn checksum(value: &str) -> Result<String, String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err("Use a 64-character lowercase SHA-256 checksum".into());
    }
    Ok(value.into())
}
fn webhook_url(value: &str) -> Result<String, String> {
    let url = Url::parse(value).map_err(|_| "Use a public HTTPS webhook URL")?;
    if value.chars().count() > 2048
        || url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err("Use a public HTTPS webhook URL without credentials or fragment".into());
    }
    Ok(value.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_api_origin_and_does_not_learn_credentials_from_urls() {
        assert_eq!(
            api_origin("https://api.nodana.io").unwrap().as_str(),
            "https://api.nodana.io/v1/"
        );
        for origin in [
            "http://api.nodana.io",
            "https://secret@api.nodana.io",
            "https://api.nodana.io/v1/",
            "https://api.nodana.io?token=secret",
            "https://api.nodana.io#fragment",
        ] {
            assert!(api_origin(origin).is_err());
        }
    }
    #[test]
    fn lists_zero_or_one_webhook_without_changing_get_response() {
        assert_eq!(webhook_list(json!({"endpoint":null})).unwrap(), json!([]));
        let endpoint = json!({"id":"webhook-test","enabled":false});
        assert_eq!(
            webhook_list(json!({"endpoint":endpoint})).unwrap(),
            json!([endpoint])
        );
        assert!(webhook_list(json!({})).is_err());
    }
}
