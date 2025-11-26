use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use keyring::Entry;
use serde::{Deserialize, Serialize};

use crate::{api::NodanaClient, output};

#[derive(Serialize, Deserialize)]
struct Credential {
    token: String,
    account_id: String,
    expires_at: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginRequest {
    request_id: String,
    #[serde(rename = "requestSecret")]
    request_secret: String,
    verification_code: String,
    verification_uri: String,
    expires_in: u64,
}

#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum CollectionResponse {
    Denied,
    Consumed,
    Authorized {
        #[serde(rename = "accountId")]
        account_id: String,
        token: String,
        #[serde(rename = "expiresAt")]
        expires_at: u64,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetails {
    account_id: String,
    expires_at: u64,
}

fn entry(client: &NodanaClient) -> Result<Entry> {
    Entry::new("io.nodana.cli", &client.credential_identity())
        .context("Could not open the OS credential store")
}

fn load(entry: &Entry) -> Result<Option<Credential>> {
    match entry.get_password() {
        Ok(value) => Ok(Some(serde_json::from_str(&value).context("Stored CLI session is invalid. Remove the io.nodana.cli entry from your OS credential store, then run `ndn login`")?)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(error) => Err(error).context("Could not read the OS credential store. Unlock your keychain or Secret Service and try again"),
    }
}

fn remove(entry: &Entry) -> Result<()> {
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(error).context("Could not remove the local CLI session"),
    }
}

fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

pub async fn connect(client: &mut NodanaClient) -> Result<()> {
    let credential =
        load(&entry(client)?)?.context("CLI is not connected. Run `ndn login` first")?;
    if credential.expires_at <= now()? {
        if output::interactive() {
            return login(client).await;
        }
        bail!("CLI session expired. Run `ndn login` to reconnect");
    }
    client.set_token(Some(credential.token));
    Ok(())
}

pub async fn login(client: &mut NodanaClient) -> Result<()> {
    let store = entry(client)?;
    login_with_store(client, &store, true).await
}

async fn login_with_store(
    client: &mut NodanaClient,
    store: &Entry,
    open_browser: bool,
) -> Result<()> {
    // Check store availability before granting another session; keep the old session until success.
    let previous = load(store)?;
    client.set_token(None);
    output::clear();
    output::heading("Connect CLI");
    let request: LoginRequest = {
        let _spinner = output::Spinner::new("Preparing browser sign-in…");
        client
            .auth_request("auth/cli/requests", &serde_json::json!({}))
            .await?
    };
    let url = browser_url(&request)?;
    output::field(
        "Verification code",
        &output::clean(&request.verification_code),
    );
    println!(
        "\nOpen this page and confirm the code matches your terminal:\n{}\n",
        url.as_str()
    );
    if open_browser && open::that(url.as_str()).is_err() {
        eprintln!("Could not open your browser automatically. Open the link above.");
    }
    let deadline = tokio::time::Instant::now() + Duration::from_secs(request.expires_in.min(300));
    let spinner = output::Spinner::new("Waiting for browser approval…");
    tokio::select! {
            result = tokio::time::timeout_at(deadline, wait_for_approval(client, &request)) => result.context("Login request expired. Run `ndn login` again")??,
            _ = tokio::signal::ctrl_c() => bail!("Login cancelled"),
    }
    let response: CollectionResponse = tokio::time::timeout_at(deadline, client.auth_request("auth/cli/collect", &serde_json::json!({"requestId":request.request_id,"requestSecret":request.request_secret})))
        .await.context("Login request expired. Run `ndn login` again")??;
    let credential = match response {
        CollectionResponse::Denied => {
            bail!("CLI access was declined. Run `ndn login` to try again")
        }
        CollectionResponse::Consumed => {
            bail!("This login request was already collected. Run `ndn login` again")
        }
        CollectionResponse::Authorized {
            account_id,
            token,
            expires_at,
        } => {
            if !valid_token(&token) || expires_at <= now()? {
                bail!("The API returned an invalid CLI session");
            }
            Credential {
                token,
                account_id,
                expires_at,
            }
        }
    };
    drop(spinner);
    client.set_token(Some(credential.token.clone()));
    if let Err(error) = store.set_password(&serde_json::to_string(&credential)?) {
        let revoked = client.revoke().await;
        client.set_token(None);
        if revoked.is_err() {
            bail!(
                "Could not save the CLI session in the OS credential store: {error}. Could not revoke the unsaved session; it will expire automatically. Run `ndn login` again after fixing credential storage"
            );
        }
        return Err(error)
            .context("Could not save the CLI session. The unsaved session was revoked");
    }
    if let Some(previous) = previous {
        client.set_token(Some(previous.token));
        if client.revoke().await.is_err() {
            eprintln!("Previous CLI session could not be revoked; it will expire automatically.");
        }
        client.set_token(Some(credential.token));
    }
    output::clear();
    output::heading("CLI connected");
    output::success("Signed in successfully");
    output::field("Account", &output::clean(&credential.account_id));
    output::field("Access", "Read and write nodes");
    output::field("Session", "Expires in 30 days");
    println!("\nNext: ndn node list");
    Ok(())
}

async fn wait_for_approval(client: &NodanaClient, request: &LoginRequest) -> Result<()> {
    let body =
        serde_json::json!({"requestId":request.request_id,"requestSecret":request.request_secret});
    let mut retry = 1;
    loop {
        let response = match client.authorization_events(&body).await {
            Ok(response) => response,
            Err(error) if error.downcast_ref::<reqwest::Error>().is_some() => {
                tokio::time::sleep(Duration::from_secs(retry)).await;
                retry = (retry * 2).min(10);
                continue;
            }
            Err(error) => return Err(error),
        };
        let mut events = response.bytes_stream().eventsource();
        while let Some(event) = events.next().await {
            let event = match event {
                Ok(event) => event,
                Err(eventsource_stream::EventStreamError::Transport(_)) => break,
                Err(error) => return Err(error).context("Could not read CLI authorization events"),
            };
            if event.event == "cli-error" {
                bail!("Login request is no longer available. Run `ndn login` again");
            }
            if event.event != "cli-authorization" {
                continue;
            }
            let status: serde_json::Value =
                serde_json::from_str(&event.data).context("Invalid CLI authorization event")?;
            match status["status"].as_str() {
                Some("approved") => return Ok(()),
                Some("pending") => {}
                Some("denied") => bail!("CLI access was declined. Run `ndn login` to try again"),
                Some("consumed") => {
                    bail!("This login request was already collected. Run `ndn login` again")
                }
                _ => bail!("Invalid CLI authorization status"),
            }
        }
        // Only reconnect a broken stream; pending events never cause another request.
        tokio::time::sleep(Duration::from_secs(retry)).await;
        retry = (retry * 2).min(10);
    }
}

fn valid_token(token: &str) -> bool {
    token.strip_prefix("nodana_cli_").is_some_and(|secret| {
        secret.len() == 64
            && secret
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    })
}

fn browser_url(request: &LoginRequest) -> Result<reqwest::Url> {
    let url = reqwest::Url::parse(&request.verification_uri)?;
    if (url.scheme() != "https"
        && !(url.scheme() == "http"
            && matches!(url.host_str(), Some("localhost" | "127.0.0.1" | "[::1]"))))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
        || url.path() != "/login"
        || url.query_pairs().collect::<Vec<_>>()
            != vec![("cli_request".into(), request.request_id.as_str().into())]
    {
        bail!("The API returned an unsafe browser sign-in URL");
    }
    Ok(url)
}

pub async fn status(client: &mut NodanaClient) -> Result<()> {
    output::heading("Connection status");
    let Some(credential) = load(&entry(client)?)? else {
        println!("Not connected. Run `ndn login`.");
        return Ok(());
    };
    client.set_token(Some(credential.token));
    let details = {
        let _spinner = output::Spinner::new("Checking CLI session…");
        client.session().await?
    };
    output::success("Connected");
    output::field("Account", &output::clean(&details.account_id));
    let hours = details.expires_at.saturating_sub(now()?) / 3600;
    output::field(
        "Expires in",
        &format!("{} days, {} hours", hours / 24, hours % 24),
    );
    Ok(())
}

pub async fn logout(client: &mut NodanaClient) -> Result<()> {
    let store = entry(client)?;
    if let Some(credential) = load(&store)? {
        client.set_token(Some(credential.token));
        let _spinner = output::Spinner::new("Revoking CLI session…");
        client.revoke().await.context("Logout could not reach the API. The local credential has been retained so you can retry `ndn logout`")?;
    }
    remove(&store)?;
    client.set_token(None);
    output::success("CLI disconnected");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_are_origin_scoped_and_never_include_an_api_key_override() {
        let production = NodanaClient::new("https://api.nodana.io", None).unwrap();
        let local = NodanaClient::new("http://localhost:3000", None).unwrap();
        assert_ne!(
            production.credential_identity(),
            local.credential_identity()
        );
        let entry = mock_entry();
        assert!(load(&entry).unwrap().is_none());
        let credential = Credential {
            token: "nodana_cli_test".into(),
            account_id: "account-test".into(),
            expires_at: 123,
        };
        entry
            .set_password(&serde_json::to_string(&credential).unwrap())
            .unwrap();
        assert_eq!(load(&entry).unwrap().unwrap().expires_at, 123);
        remove(&entry).unwrap();
        remove(&entry).unwrap();
        assert!(load(&entry).unwrap().is_none());
    }

    fn mock_entry() -> Entry {
        Entry::new_with_credential(
            keyring::mock::default_credential_builder()
                .build(None, "test", "test")
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn browser_login_collects_and_stores_a_session_without_bearer_auth() {
        let token = format!("nodana_cli_{}", "a".repeat(64));
        let expires = now().unwrap() + 2_592_000;
        let (origin, server) = login_server(serde_json::json!({"status":"authorized", "accountId":"account-example", "sessionId":"cli-session-example", "token":token, "expiresAt":expires})).await;
        let store = mock_entry();
        let mut client = NodanaClient::new(&origin, None).unwrap();
        login_with_store(&mut client, &store, false).await.unwrap();
        let saved = load(&store).unwrap().unwrap();
        assert_eq!(saved.token, token);
        assert_eq!(saved.account_id, "account-example");
        assert_eq!(saved.expires_at, expires);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn declined_login_preserves_the_previous_credential() {
        let (origin, server) = login_server(serde_json::json!({"status":"denied"})).await;
        let store = mock_entry();
        store
            .set_password(
                &serde_json::to_string(&Credential {
                    token: "previous".into(),
                    account_id: "account-old".into(),
                    expires_at: 123,
                })
                .unwrap(),
            )
            .unwrap();
        let mut client = NodanaClient::new(&origin, None).unwrap();
        let error = login_with_store(&mut client, &store, false)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("declined"));
        assert_eq!(load(&store).unwrap().unwrap().token, "previous");
        server.await.unwrap();
    }

    async fn login_server(result: serde_json::Value) -> (String, tokio::task::JoinHandle<()>) {
        login_server_with_reconnect(result, false).await
    }

    #[tokio::test]
    async fn interrupted_event_stream_reconnects_and_collects_once() {
        let token = format!("nodana_cli_{}", "a".repeat(64));
        let expires = now().unwrap() + 2_592_000;
        let (origin, server) = login_server_with_reconnect(serde_json::json!({"status":"authorized", "accountId":"account-example", "sessionId":"cli-session-example", "token":token, "expiresAt":expires}), true).await;
        let store = mock_entry();
        let mut client = NodanaClient::new(&origin, None).unwrap();
        login_with_store(&mut client, &store, false).await.unwrap();
        assert_eq!(load(&store).unwrap().unwrap().token, token);
        server.await.unwrap();
    }

    async fn login_server_with_reconnect(
        result: serde_json::Value,
        reconnect: bool,
    ) -> (String, tokio::task::JoinHandle<()>) {
        use tokio::{
            io::{AsyncReadExt, AsyncWriteExt},
            net::TcpListener,
        };
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let request_id = "b".repeat(64);
        let secret = "c".repeat(64);
        let created = serde_json::json!({"requestId":request_id,"requestSecret":secret,"verificationCode":"AB12-CD34","verificationUri":format!("http://localhost:5174/login?cli_request={request_id}"),"expiresIn":300});
        let status = if result["status"] == "authorized" {
            "approved"
        } else {
            "denied"
        };
        let mut responses = vec![
            (
                "/v1/auth/cli/requests",
                created.to_string(),
                "application/json",
            ),
            (
                "/v1/auth/cli/events",
                format!(
                    "event: cli-authorization\ndata: {{\"status\":\"pending\"}}\n\n: keep-alive\n\nevent: cli-authorization\ndata: {{\"status\":\"{status}\"}}\n\n"
                ),
                "text/event-stream",
            ),
        ];
        if reconnect {
            responses.insert(
                1,
                (
                    "/v1/auth/cli/events",
                    "event: cli-authorization\ndata: {\"status\":\"pending\"}\n\n".into(),
                    "text/event-stream",
                ),
            );
        }
        if status == "approved" {
            responses.push((
                "/v1/auth/cli/collect",
                result.to_string(),
                "application/json",
            ));
        }
        let server = tokio::spawn(async move {
            for (path, body, content_type) in responses {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut received = Vec::new();
                let end = loop {
                    let mut bytes = [0_u8; 4096];
                    let count = stream.read(&mut bytes).await.unwrap();
                    assert!(count > 0);
                    received.extend_from_slice(&bytes[..count]);
                    if let Some(end) = received.windows(4).position(|b| b == b"\r\n\r\n") {
                        break end + 4;
                    }
                };
                let headers = String::from_utf8_lossy(&received[..end]).to_ascii_lowercase();
                assert!(headers.starts_with(&format!("post {path} http/1.1")));
                assert!(!headers.contains("authorization:"));
                let length: usize = headers
                    .lines()
                    .find_map(|line| line.strip_prefix("content-length: "))
                    .unwrap()
                    .parse()
                    .unwrap();
                while received.len() < end + length {
                    let mut bytes = [0_u8; 4096];
                    let count = stream.read(&mut bytes).await.unwrap();
                    assert!(count > 0);
                    received.extend_from_slice(&bytes[..count]);
                }
                if path.ends_with("events") || path.ends_with("collect") {
                    let input: serde_json::Value =
                        serde_json::from_slice(&received[end..end + length]).unwrap();
                    assert_eq!(input["requestId"], request_id);
                    assert_eq!(input["requestSecret"], secret);
                }
                stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nConnection: close\r\nContent-Length: {}\r\n\r\n",body.len()).as_bytes()).await.unwrap();
                for part in body.as_bytes().chunks(17) {
                    stream.write_all(part).await.unwrap();
                    tokio::task::yield_now().await;
                }
            }
        });
        (format!("http://{address}"), server)
    }

    #[test]
    fn rejects_insecure_browser_urls_and_api_key_shaped_credentials() {
        let mut request = LoginRequest {
            request_id: "a".repeat(64),
            request_secret: "b".repeat(64),
            verification_code: "AB12-CD34".into(),
            verification_uri: format!("https://app.nodana.io/login?cli_request={}", "a".repeat(64)),
            expires_in: 300,
        };
        assert!(browser_url(&request).is_ok());
        for uri in [
            "http://app.nodana.io/login",
            "https://user:password@app.nodana.io/login",
            "https://app.nodana.io/dashboard",
            "file:///tmp/login",
        ] {
            request.verification_uri = uri.into();
            assert!(browser_url(&request).is_err());
        }
        assert!(!valid_token("nodana_key_abcd1234"));
        assert!(!valid_token("nodana_cli_short"));
    }
}
