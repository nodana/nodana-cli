use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use reqwest::{Client, Method, Url};
use zeroize::Zeroizing;

pub struct PhoenixdClient {
    client: Client,
    base_url: Url,
    password: Zeroizing<String>,
}

pub enum Reply {
    Json(serde_json::Value),
    Text(String),
}

impl PhoenixdClient {
    pub fn new(origin: &str, password: Zeroizing<String>) -> Result<Self> {
        let base_url = base_url(origin)?;
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .context("Could not initialize the Phoenixd client")?;
        Ok(Self {
            client,
            base_url,
            password,
        })
    }

    pub async fn request(
        &self,
        method: Method,
        parts: &[&str],
        params: &[(String, String)],
        query: bool,
    ) -> Result<Reply> {
        let mut url = self.base_url.clone();
        {
            let mut path = url
                .path_segments_mut()
                .map_err(|_| anyhow::anyhow!("Invalid node URL"))?;
            path.pop_if_empty();
            for part in parts {
                path.push(part);
            }
        }
        let mut request = self
            .client
            .request(method, url)
            .basic_auth("", Some(self.password.as_str()));
        if query {
            request = request.query(params);
        } else {
            request = request.form(params);
        }
        // Never replay a payment or follow a redirect with the node password.
        let mut response = request.send().await.map_err(|_| anyhow::anyhow!("Could not complete the Phoenixd request. Check the node URL and connectivity. If this request could move funds, check payment or transaction status before trying again."))?;
        let status = response.status();
        if !status.is_success() {
            match status.as_u16() {
                401 | 403 => bail!(
                    "Phoenixd rejected authentication (HTTP {status}). Check the node password and whether this command requires full access."
                ),
                404 => bail!(
                    "Phoenixd returned HTTP 404. Check the node URL, endpoint and payment identifier."
                ),
                429 => {
                    let wait = response
                        .headers()
                        .get(reqwest::header::RETRY_AFTER)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok());
                    if let Some(wait) = wait {
                        bail!("Phoenixd rate limit exceeded. Retry after {wait} seconds.");
                    }
                    bail!("Phoenixd rate limit exceeded. Pause requests before retrying.");
                }
                _ => bail!(
                    "Phoenixd request failed with HTTP {status}. Check this command's parameters. For a payment or channel action, reconcile its outcome before retrying."
                ),
            }
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| {
            anyhow::anyhow!(
                "Phoenixd response was interrupted. Check the operation's outcome before retrying."
            )
        })? {
            ensure!(
                bytes.len() + chunk.len() <= 16 * 1024 * 1024,
                "Phoenixd response is too large; request a smaller page of results"
            );
            bytes.extend_from_slice(&chunk);
        }
        if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes)
            && (value.is_object() || value.is_array())
        {
            return Ok(Reply::Json(value));
        }
        let text = String::from_utf8(bytes).context("Phoenixd returned a non-UTF-8 response")?;
        Ok(Reply::Text(text))
    }
}

fn base_url(value: &str) -> Result<Url> {
    let url =
        Url::parse(value).map_err(|_| anyhow::anyhow!("--url must be a valid HTTP(S) origin"))?;
    ensure!(
        matches!(url.scheme(), "http" | "https")
            && url.host().is_some()
            && url.username().is_empty()
            && url.password().is_none()
            && matches!(url.path(), "" | "/")
            && url.query().is_none()
            && url.fragment().is_none(),
        "--url must be an HTTP(S) origin without embedded credentials, paths, queries or fragments"
    );
    if url.scheme() == "http" {
        let local = url.host_str().is_some_and(|host| {
            host == "localhost"
                || host
                    .trim_matches(['[', ']'])
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        });
        ensure!(local, "Use HTTPS for remote Phoenixd nodes");
    }
    Ok(url)
}
