use std::{env, time::Duration};

use anyhow::{Context, Result, bail};
use reqwest::{Client, Method, Response, Url};
use serde::{Serialize, de::DeserializeOwned};

use crate::models::{
    ApiErrorResponse, CreateNodeResponse, Node, NodeUpdateResponse, NodesResponse,
};

const DEFAULT_API_ORIGIN: &str = "https://api.nodana.io";
const API_ORIGIN_ENV: &str = "NODANA_API_ORIGIN";
const API_KEY_ENV: &str = "NODANA_API_KEY";

pub struct NodanaClient {
    client: Client,
    base_url: Url,
    api_key: String,
}

impl NodanaClient {
    pub fn from_env() -> Result<Self> {
        let api_origin = env::var(API_ORIGIN_ENV).unwrap_or_else(|_| DEFAULT_API_ORIGIN.into());
        let api_key = env::var(API_KEY_ENV)
            .with_context(|| format!("Set {API_KEY_ENV} to a Nodana API key"))?;
        Self::new(&api_origin, api_key)
    }

    pub fn new(api_origin: &str, api_key: String) -> Result<Self> {
        let base_url = api_base_url(api_origin)?;
        let api_key = api_key.trim().to_owned();
        if api_key.is_empty() {
            bail!("{API_KEY_ENV} must not be empty");
        }
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;
        Ok(Self {
            client,
            base_url,
            api_key,
        })
    }

    pub async fn list_nodes(&self) -> Result<Vec<Node>> {
        Ok(self
            .request::<NodesResponse, ()>(Method::GET, self.node_url(None, None)?, None)
            .await?
            .nodes)
    }

    pub async fn create_node(
        &self,
        name: Option<&str>,
        auto_liquidity: &str,
        full: bool,
    ) -> Result<CreateNodeResponse> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct CreateRequest<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
            auto_liquidity: &'a str,
            access_mode: &'a str,
        }
        self.request(
            Method::POST,
            self.node_url(None, None)?,
            Some(&CreateRequest {
                name,
                auto_liquidity,
                access_mode: if full { "full" } else { "limited" },
            }),
        )
        .await
    }

    pub async fn get_node(&self, id: &str) -> Result<Node> {
        self.request::<Node, ()>(Method::GET, self.node_url(Some(id), None)?, None)
            .await
    }

    pub async fn get_node_update(&self, id: &str) -> Result<NodeUpdateResponse> {
        self.request::<NodeUpdateResponse, ()>(
            Method::GET,
            self.node_url(Some(id), Some("update"))?,
            None,
        )
        .await
    }

    pub async fn node_action(&self, id: &str, action: &str) -> Result<Node> {
        self.request::<Node, ()>(Method::POST, self.node_url(Some(id), Some(action))?, None)
            .await
    }

    pub async fn delete_node(&self, id: &str) -> Result<()> {
        let response = self
            .send::<()>(Method::DELETE, self.node_url(Some(id), None)?, None)
            .await?;
        if response.status() != reqwest::StatusCode::NO_CONTENT {
            bail!("Unexpected API response: {}", response.status());
        }
        Ok(())
    }

    fn node_url(&self, id: Option<&str>, action: Option<&str>) -> Result<Url> {
        let mut url = self.base_url.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| anyhow::anyhow!("Invalid API URL"))?;
            segments.pop_if_empty();
            segments.push("nodes");
            if let Some(id) = id {
                if id.is_empty() || id == "." || id == ".." || id.contains('/') || id.contains('\\')
                {
                    bail!("Invalid node ID");
                }
                segments.push(id);
            }
            if let Some(action) = action {
                segments.push(action);
            }
        }
        Ok(url)
    }

    async fn request<T: DeserializeOwned, B: Serialize + ?Sized>(
        &self,
        method: Method,
        url: Url,
        body: Option<&B>,
    ) -> Result<T> {
        let response = self.send(method, url, body).await?;
        response
            .json()
            .await
            .context("Could not read the API response")
    }

    async fn send<B: Serialize + ?Sized>(
        &self,
        method: Method,
        url: Url,
        body: Option<&B>,
    ) -> Result<Response> {
        let mut request = self.client.request(method, url).bearer_auth(&self.api_key);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request
            .send()
            .await
            .context("Could not reach the Nodana API")?;
        if response.status().is_success() {
            return Ok(response);
        }
        let status = response.status();
        let retry_after = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let bytes = response.bytes().await?;
        if let Ok(error) = serde_json::from_slice::<ApiErrorResponse>(&bytes) {
            if let Some(wait) = retry_after {
                bail!(
                    "API error {} ({}): {}. Retry after {wait} seconds",
                    status,
                    error.error.code,
                    error.error.message
                );
            }
            bail!(
                "API error {} ({}): {}",
                status,
                error.error.code,
                error.error.message
            );
        }
        bail!("API request failed with HTTP {status}")
    }
}

fn api_base_url(api_origin: &str) -> Result<Url> {
    let origin = Url::parse(api_origin.trim_end_matches('/'))?;
    if !matches!(origin.scheme(), "http" | "https")
        || origin.host().is_none()
        || !origin.username().is_empty()
        || origin.password().is_some()
        || !matches!(origin.path(), "" | "/")
        || origin.query().is_some()
        || origin.fragment().is_some()
    {
        bail!("{API_ORIGIN_ENV} must be a valid HTTP(S) origin");
    }
    if origin.scheme() == "http"
        && !matches!(origin.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"))
    {
        bail!("{API_ORIGIN_ENV} must use HTTPS outside localhost");
    }
    Ok(origin.join("/v1/")?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_node_urls_without_losing_v1_prefix() {
        let client = NodanaClient::new("http://localhost:3000/", "key".into()).unwrap();
        assert_eq!(
            client
                .node_url(Some("node-123"), Some("restart"))
                .unwrap()
                .as_str(),
            "http://localhost:3000/v1/nodes/node-123/restart"
        );
        assert!(client.node_url(Some("../other"), None).is_err());
    }

    #[test]
    fn rejects_origins_with_embedded_credentials() {
        assert!(api_base_url("https://user:pass@example.com").is_err());
        assert!(api_base_url("http://api.example.com").is_err());
        assert!(api_base_url("http://127.0.0.1:3000").is_ok());
    }

    #[tokio::test]
    async fn creates_node_with_bearer_key_and_parses_one_time_credentials() {
        use tokio::{
            io::{AsyncReadExt, AsyncWriteExt},
            net::TcpListener,
        };

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let header_end;
            loop {
                let mut chunk = [0_u8; 2048];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0);
                request.extend_from_slice(&chunk[..count]);
                if let Some(position) = request.windows(4).position(|window| window == b"\r\n\r\n")
                {
                    header_end = position + 4;
                    break;
                }
            }
            let headers = String::from_utf8_lossy(&request[..header_end]).to_ascii_lowercase();
            assert!(headers.starts_with("post /v1/nodes http/1.1"));
            assert!(headers.contains("authorization: bearer test-key\r\n"));
            let content_length: usize = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
                .unwrap()
                .parse()
                .unwrap();
            while request.len() - header_end < content_length {
                let mut chunk = [0_u8; 2048];
                let count = stream.read(&mut chunk).await.unwrap();
                assert!(count > 0);
                request.extend_from_slice(&chunk[..count]);
            }
            let body: serde_json::Value =
                serde_json::from_slice(&request[header_end..header_end + content_length]).unwrap();
            assert_eq!(
                body,
                serde_json::json!({"name":"Example","autoLiquidity":"5m","accessMode":"full"})
            );
            let response = r#"{"node":{"id":"node-123","name":"Example","status":"provisioning","endpointUrl":"https://example.com","failureMessage":null,"updateStatus":null},"credentials":{"seed":"words","password":"admin","restrictedPassword":"restricted"}}"#;
            stream.write_all(format!("HTTP/1.1 202 Accepted\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes()).await.unwrap();
        });
        let client = NodanaClient::new(&format!("http://{address}"), "test-key".into()).unwrap();
        let created = client
            .create_node(Some("Example"), "5m", true)
            .await
            .unwrap();
        assert_eq!(created.node.id, "node-123");
        assert_eq!(created.credentials.restricted_password, "restricted");
        server.await.unwrap();
    }
}
