use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::{Client, Method, Response, Url};

use crate::commands::Request;

pub fn node_url(value: Option<&str>) -> Result<Url> {
    let value = value
        .filter(|v| !v.is_empty())
        .context("Set NODANA_PHOENIXD_URL or pass --url")?;
    let mut url = Url::parse(value).context("Invalid node URL")?;
    if url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !(url.scheme() == "https"
            || (url.scheme() == "http"
                && matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"))))
    {
        bail!(
            "Use an HTTPS node URL, or HTTP for localhost, without credentials, query or fragment"
        );
    }
    if !url.path().ends_with('/') {
        url.set_path(&format!("{}/", url.path()));
    }
    Ok(url)
}

pub fn http_client() -> Result<Client> {
    Ok(Client::builder()
        .timeout(Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::none())
        .tls_info(true)
        .build()?)
}

pub struct PhoenixClient {
    client: Client,
    url: Url,
    password: String,
}

impl PhoenixClient {
    pub fn new(url: Url, password: String) -> Result<Self> {
        Ok(Self {
            client: http_client()?,
            url,
            password,
        })
    }

    pub async fn execute(&self, request: Request) -> Result<String> {
        let mut url = self.url.clone();
        {
            let mut path = url
                .path_segments_mut()
                .map_err(|_| anyhow::anyhow!("Invalid node URL"))?;
            path.pop_if_empty();
            for segment in request.path {
                if segment.is_empty()
                    || segment == "."
                    || segment == ".."
                    || segment.contains(['/', '\\'])
                {
                    bail!("Invalid payment identifier");
                }
                path.push(&segment);
            }
        }
        let builder = self
            .client
            .request(request.method.clone(), url)
            .basic_auth("", Some(&self.password));
        let builder = if request.method == Method::GET {
            builder.query(&request.params)
        } else {
            builder.form(&request.params)
        };
        let response = builder.send().await.context("Could not reach Phoenixd")?;
        check_status(&response)?;
        let body = read_limited(response, 16 * 1024 * 1024).await?;
        let text = String::from_utf8(body).context("Phoenixd returned invalid text")?;
        // Preserve Phoenixd's response, including payment_failed and text action results.
        Ok(match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(value) => serde_json::to_string_pretty(&value)?,
            Err(_) => text,
        })
    }
}

pub fn check_status(response: &Response) -> Result<()> {
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    match status.as_u16() {
        401 => {
            bail!("Phoenixd rejected the password. Check NODANA_PHOENIXD_PASSWORD or --password")
        }
        403 => bail!("Phoenixd denied access. Use the full password for commands that require it"),
        // Do not echo arbitrary error bodies or request URLs (which may contain secrets).
        _ => bail!("Phoenixd request failed with HTTP {status}"),
    }
}

pub async fn read_limited(mut response: Response, maximum: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .context("Could not read the response")?
    {
        if chunk.len() > maximum.saturating_sub(bytes.len()) {
            bail!("Response is too large");
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    #[test]
    fn requires_a_safe_connection_url() {
        for value in [
            None,
            Some(""),
            Some("http://node.example.com"),
            Some("https://:secret@node.example.com"),
            Some("https://node.example.com?password=x"),
            Some("https://node.example.com#fragment"),
        ] {
            assert!(node_url(value).is_err(), "{value:?}");
        }
        assert_eq!(
            node_url(Some("http://127.0.0.1:9740")).unwrap().as_str(),
            "http://127.0.0.1:9740/"
        );
        assert_eq!(
            node_url(Some("https://node.example.com/prefix"))
                .unwrap()
                .path(),
            "/prefix/"
        );
    }

    async fn server(response: &'static str) -> (Url, tokio::task::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("http://{}/", listener.local_addr().unwrap())).unwrap();
        let task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut received = Vec::new();
            let (header_end, content_length) = loop {
                let mut buffer = [0; 2048];
                let count = stream.read(&mut buffer).await.unwrap();
                assert!(count > 0);
                received.extend_from_slice(&buffer[..count]);
                if let Some(end) = received.windows(4).position(|v| v == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&received[..end]).to_ascii_lowercase();
                    let length = headers
                        .lines()
                        .find_map(|line| line.strip_prefix("content-length: "))
                        .unwrap_or("0")
                        .parse::<usize>()
                        .unwrap();
                    break (end + 4, length);
                }
            };
            while received.len() < header_end + content_length {
                let mut buffer = [0; 2048];
                let count = stream.read(&mut buffer).await.unwrap();
                assert!(count > 0);
                received.extend_from_slice(&buffer[..count]);
            }
            stream.write_all(response.as_bytes()).await.unwrap();
            String::from_utf8(received).unwrap()
        });
        (url, task)
    }

    #[tokio::test]
    async fn sends_basic_auth_and_url_encoded_forms_without_changing_results() {
        let (url, task) =
            server("HTTP/1.1 200 OK\r\nContent-Length: 25\r\n\r\n{\"type\":\"payment_failed\"}")
                .await;
        let result = PhoenixClient::new(url, "private".into())
            .unwrap()
            .execute(Request {
                method: Method::POST,
                path: vec!["createinvoice".into()],
                params: vec![
                    ("description".into(), "Order & coffee + tip".into()),
                    ("amountSat".into(), "123".into()),
                ],
            })
            .await
            .unwrap();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&result).unwrap()["type"],
            "payment_failed"
        );
        let request = task.await.unwrap();
        assert!(request.starts_with("POST /createinvoice HTTP/1.1\r\n"));
        let headers = request.to_ascii_lowercase();
        assert!(headers.contains("authorization: basic onbyaxzhdgu=\r\n"));
        assert!(headers.contains("content-type: application/x-www-form-urlencoded\r\n"));
        assert!(request.ends_with("description=Order+%26+coffee+%2B+tip&amountSat=123"));
    }

    #[tokio::test]
    async fn sends_history_filters_as_query_and_preserves_text() {
        let (url, task) = server("HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nplain text").await;
        let result = PhoenixClient::new(url, "private".into())
            .unwrap()
            .execute(Request {
                method: Method::GET,
                path: vec!["payments".into(), "incoming".into()],
                params: vec![
                    ("all".into(), "true".into()),
                    ("externalId".into(), "order & 1".into()),
                ],
            })
            .await
            .unwrap();
        assert_eq!(result, "plain text");
        assert!(
            task.await
                .unwrap()
                .starts_with("GET /payments/incoming?all=true&externalId=order+%26+1 HTTP/1.1\r\n")
        );
    }

    #[tokio::test]
    async fn refuses_redirects_and_does_not_echo_error_secrets() {
        let (url, task) = server("HTTP/1.1 302 Found\r\nLocation: https://other.example\r\nContent-Length: 6\r\n\r\nsecret").await;
        let error = PhoenixClient::new(url, "private".into())
            .unwrap()
            .execute(Request {
                method: Method::GET,
                path: vec!["getinfo".into()],
                params: vec![],
            })
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("302"));
        assert!(!error.contains("secret") && !error.contains("private"));
        task.await.unwrap();
    }
}
