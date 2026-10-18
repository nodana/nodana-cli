use std::time::Duration;

use base64::{Engine, engine::general_purpose::STANDARD};
use clap::Parser;
use serde_json::json;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::timeout,
};
use zeroize::Zeroizing;

use crate::{
    Cli,
    api::{PhoenixdClient, Reply},
    commands,
};

async fn read_request(stream: &mut TcpStream) -> (String, String) {
    let mut bytes = Vec::new();
    loop {
        let mut buf = [0; 4096];
        let count = stream.read(&mut buf).await.unwrap();
        assert!(count > 0);
        bytes.extend_from_slice(&buf[..count]);
        if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
            let headers = String::from_utf8(bytes[..end].to_vec()).unwrap();
            let length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().unwrap())
                })
                .unwrap_or(0);
            if bytes.len() >= end + 4 + length {
                return (
                    headers,
                    String::from_utf8(bytes[end + 4..end + 4 + length].to_vec()).unwrap(),
                );
            }
        }
    }
}

fn command(args: &[&str]) -> crate::Commands {
    Cli::try_parse_from(std::iter::once("ndn").chain(args.iter().copied()))
        .unwrap()
        .command
}

#[tokio::test]
async fn every_documented_command_uses_native_routes_authentication_and_encoding() {
    let hash = "a".repeat(64);
    type Case<'a> = (Vec<&'a str>, &'a str, String, Vec<(&'a str, &'a str)>);
    let cases: Vec<Case<'_>> = vec![
        (vec!["get-info"], "GET", "/getinfo".into(), vec![]),
        (vec!["get-balance"], "GET", "/getbalance".into(), vec![]),
        (vec!["list-channels"], "GET", "/listchannels".into(), vec![]),
        (
            vec![
                "create-invoice",
                "--description",
                "Order + & ⚡",
                "--amount-sat",
                "1000",
                "--expiry-seconds",
                "600",
                "--external-id",
                "order/1",
                "--webhook-url",
                "https://example.com/event?a=1&b=2",
            ],
            "POST",
            "/createinvoice".into(),
            vec![
                ("description", "Order + & ⚡"),
                ("amountSat", "1000"),
                ("expirySeconds", "600"),
                ("externalId", "order/1"),
                ("webhookUrl", "https://example.com/event?a=1&b=2"),
            ],
        ),
        (
            vec![
                "create-offer",
                "--description",
                "Tips",
                "--amount-sat",
                "100",
            ],
            "POST",
            "/createoffer".into(),
            vec![("description", "Tips"), ("amountSat", "100")],
        ),
        (
            vec!["get-ln-address"],
            "GET",
            "/getlnaddress".into(),
            vec![],
        ),
        (
            vec!["get-swap-in-address"],
            "GET",
            "/getswapinaddress".into(),
            vec![],
        ),
        (
            vec![
                "pay-invoice",
                "--invoice",
                "lnbc-placeholder",
                "--amount-sat",
                "21",
            ],
            "POST",
            "/payinvoice".into(),
            vec![("invoice", "lnbc-placeholder"), ("amountSat", "21")],
        ),
        (
            vec![
                "pay-offer",
                "--offer",
                "lno-placeholder",
                "--send-all",
                "--message",
                "thanks & hi",
            ],
            "POST",
            "/payoffer".into(),
            vec![
                ("offer", "lno-placeholder"),
                ("sendAll", "true"),
                ("message", "thanks & hi"),
            ],
        ),
        (
            vec![
                "pay-ln-address",
                "--address",
                "alice@example.com",
                "--amount-sat",
                "25",
            ],
            "POST",
            "/paylnaddress".into(),
            vec![("address", "alice@example.com"), ("amountSat", "25")],
        ),
        (
            vec![
                "send-to-address",
                "--amount-sat",
                "50",
                "--address",
                "bc1-example",
                "--feerate-sat-byte",
                "12",
            ],
            "POST",
            "/sendtoaddress".into(),
            vec![
                ("amountSat", "50"),
                ("address", "bc1-example"),
                ("feerateSatByte", "12"),
            ],
        ),
        (
            vec!["bump-fee", "--feerate-sat-byte", "11"],
            "POST",
            "/bumpfee".into(),
            vec![("feerateSatByte", "11")],
        ),
        (
            vec![
                "list-incoming-payments",
                "--from",
                "1",
                "--to",
                "2",
                "--limit",
                "3",
                "--offset",
                "4",
                "--all",
                "--external-id",
                "order + 1",
            ],
            "GET",
            "/payments/incoming".into(),
            vec![
                ("from", "1"),
                ("to", "2"),
                ("limit", "3"),
                ("offset", "4"),
                ("all", "true"),
                ("externalId", "order + 1"),
            ],
        ),
        (
            vec!["get-incoming-payment", "--payment-hash", &hash],
            "GET",
            format!("/payments/incoming/{hash}"),
            vec![],
        ),
        (
            vec![
                "list-outgoing-payments",
                "--limit",
                "5",
                "--offset",
                "6",
                "--all",
            ],
            "GET",
            "/payments/outgoing".into(),
            vec![("limit", "5"), ("offset", "6"), ("all", "true")],
        ),
        (
            vec![
                "get-outgoing-payment",
                "--payment-id",
                "961d413c-91ca-4f3d-9d91-760a08405901",
            ],
            "GET",
            "/payments/outgoing/961d413c-91ca-4f3d-9d91-760a08405901".into(),
            vec![],
        ),
        (
            vec!["get-outgoing-payment", "--payment-hash", &hash],
            "GET",
            format!("/payments/outgoingbyhash/{hash}"),
            vec![],
        ),
        (
            vec!["export-payments", "--from", "1", "--to", "2"],
            "POST",
            "/export".into(),
            vec![("from", "1"), ("to", "2")],
        ),
        (
            vec![
                "close-channel",
                "--channel-id",
                &hash,
                "--address",
                "bc1-example",
                "--feerate-sat-byte",
                "12",
            ],
            "POST",
            "/closechannel".into(),
            vec![
                ("channelId", &hash),
                ("address", "bc1-example"),
                ("feerateSatByte", "12"),
            ],
        ),
        (
            vec!["decode-invoice", "--invoice", "lnbc-example"],
            "POST",
            "/decodeinvoice".into(),
            vec![("invoice", "lnbc-example")],
        ),
        (
            vec!["decode-offer", "--offer", "lno-example"],
            "POST",
            "/decodeoffer".into(),
            vec![("offer", "lno-example")],
        ),
        (
            vec!["estimate-liquidity-fees", "--amount-sat", "1000000"],
            "GET",
            "/estimateliquidityfees".into(),
            vec![("amountSat", "1000000")],
        ),
        (
            vec![
                "lnurl-pay",
                "--lnurl",
                "lightning:LNURL1",
                "--message",
                "coffee",
                "--amount-sat",
                "1",
            ],
            "POST",
            "/lnurlpay".into(),
            vec![
                ("lnurl", "lightning:LNURL1"),
                ("message", "coffee"),
                ("amountSat", "1"),
            ],
        ),
        (
            vec!["lnurl-withdraw", "--lnurl", "lightning:LNURL1"],
            "POST",
            "/lnurlwithdraw".into(),
            vec![("lnurl", "lightning:LNURL1")],
        ),
        (
            vec!["lnurl-auth", "--lnurl", "LNURL1"],
            "POST",
            "/lnurlauth".into(),
            vec![("lnurl", "LNURL1")],
        ),
    ];
    for (args, method, path, expected) in cases {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = PhoenixdClient::new(
            &format!("http://{}/", listener.local_addr().unwrap()),
            Zeroizing::new("node-password".into()),
        )
        .unwrap();
        let prepared = commands::request(command(&args)).unwrap();
        let transaction = prepared.transaction;
        let server = async {
            let (mut stream, _) = listener.accept().await.unwrap();
            let (headers, body) = read_request(&mut stream).await;
            let request_line = headers.lines().next().unwrap();
            let target = request_line.split_whitespace().nth(1).unwrap();
            let url = reqwest::Url::parse(&format!("http://localhost{target}")).unwrap();
            assert_eq!(request_line.split_whitespace().next().unwrap(), method);
            assert_eq!(url.path(), path);
            assert!(headers.to_ascii_lowercase().contains(&format!(
                "authorization: basic {}",
                STANDARD.encode(":node-password").to_ascii_lowercase()
            )));
            assert!(!headers.contains("Bearer"));
            let pairs: std::collections::BTreeMap<String, String> =
                if method == "GET" || path == "/export" {
                    assert!(body.is_empty());
                    url.query_pairs().into_owned().collect()
                } else {
                    assert!(
                        headers
                            .to_ascii_lowercase()
                            .contains("content-type: application/x-www-form-urlencoded")
                    );
                    let mut form = reqwest::Url::parse("http://localhost").unwrap();
                    form.set_query(Some(&body));
                    form.query_pairs().into_owned().collect()
                };
            let expected: std::collections::BTreeMap<_, _> = expected
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            assert_eq!(pairs, expected, "{args:?}");
            let body = if transaction {
                "a".repeat(64)
            } else {
                json!({"ok":true}).to_string()
            };
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
        };
        timeout(Duration::from_secs(5), async {
            let ((), result) = tokio::join!(server, commands::run(&client, command(&args)));
            result.unwrap();
        })
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn failures_are_redacted_redirects_are_not_followed_and_lost_payments_are_not_replayed() {
    for status in [
        "401 Unauthorized",
        "302 Found",
        "429 Too Many Requests",
        "500 Internal Server Error",
        "disconnect",
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = PhoenixdClient::new(
            &format!("http://{}", listener.local_addr().unwrap()),
            Zeroizing::new("private-password".into()),
        )
        .unwrap();
        let server = async {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_request(&mut stream).await;
            if status != "disconnect" {
                let body = "private-password";
                stream.write_all(format!("HTTP/1.1 {status}\r\nLocation: /unexpected\r\nRetry-After: 2\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
            }
            drop(stream);
            assert!(
                timeout(Duration::from_millis(100), listener.accept())
                    .await
                    .is_err()
            );
        };
        let params = [("invoice".into(), "example".into())];
        let request = client.request(reqwest::Method::POST, &["payinvoice"], &params, false);
        let ((), result) = tokio::join!(server, request);
        let error = result.err().unwrap().to_string();
        assert!(!error.contains("private-password"));
        assert!(!error.contains("login"));
        if status.starts_with("429") {
            assert!(error.contains("2 seconds"));
        }
    }
}

#[tokio::test]
async fn payment_and_transaction_failure_bodies_return_an_error_even_with_http_200() {
    for (args, body) in [
        (
            vec!["pay-invoice", "--invoice", "example"],
            r#"{"type":"payment_failed","reason":"failed"}"#,
        ),
        (
            vec!["bump-fee", "--feerate-sat-byte", "2"],
            "no channel available",
        ),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = PhoenixdClient::new(
            &format!("http://{}", listener.local_addr().unwrap()),
            Zeroizing::new("password".into()),
        )
        .unwrap();
        let server = async {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_request(&mut stream).await;
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
        };
        let ((), result) = tokio::join!(server, commands::run(&client, command(&args)));
        assert!(result.is_err());
    }
}

#[test]
fn command_constraints_and_removed_commands_are_enforced() {
    let hash = "a".repeat(64);
    for args in [
        vec!["login"],
        vec!["logout"],
        vec!["status"],
        vec!["connect", "conn-test"],
        vec!["generate-key"],
        vec!["list-connections"],
        vec!["create-invoice"],
        vec![
            "create-invoice",
            "--description",
            "x",
            "--description-hash",
            &hash,
        ],
        vec![
            "pay-invoice",
            "--invoice",
            "x",
            "--amount-sat",
            "1",
            "--send-all",
        ],
        vec!["create-offer", "--amount-sat", "1"],
        vec!["get-outgoing-payment"],
        vec!["get-outgoing-payment", "--payment-id", "bad"],
        vec!["get-incoming-payment", "--payment-hash", "../secret"],
        vec!["estimate-liquidity-fees", "--amount-sat", "0"],
    ] {
        assert!(
            Cli::try_parse_from(std::iter::once("ndn").chain(args.iter().copied())).is_err(),
            "{args:?}"
        );
    }
    assert!(
        commands::request(command(&[
            "list-incoming-payments",
            "--from",
            "2",
            "--to",
            "1"
        ]))
        .is_err()
    );
    assert!(
        commands::request(command(&[
            "pay-ln-address",
            "--address",
            "alice@example.com"
        ]))
        .is_err()
    );
    assert!(
        Cli::try_parse_from(["ndn", "create-invoice", "--description", &"é".repeat(128)]).is_ok()
    );
    assert!(
        Cli::try_parse_from(["ndn", "create-invoice", "--description", &"é".repeat(129)]).is_err()
    );
    let req = commands::request(command(&["create-invoice", "--description-hash", &hash])).unwrap();
    assert_eq!(req.params, vec![("descriptionHash".into(), hash)]);
    let req = commands::request(command(&["lnurl-pay", "--lnurl", "example"])).unwrap();
    assert_eq!(req.params, vec![("lnurl".into(), "example".into())]);
}

#[test]
fn remote_passwords_require_https_and_urls_never_embed_credentials() {
    for url in [
        "http://example.com",
        "https://user:secret@example.com",
        "https://example.com/path",
        "https://example.com?password=secret",
        "https://example.com/#fragment",
        "not-secret-url",
    ] {
        let error = PhoenixdClient::new(url, Zeroizing::new("private-password".into()))
            .err()
            .unwrap()
            .to_string();
        assert!(!error.contains("secret"));
    }
    for url in [
        "https://conn-1234.nodana.app",
        "http://127.0.0.1:9740/",
        "http://[::1]:9740",
        "http://localhost:9740",
    ] {
        assert!(PhoenixdClient::new(url, Zeroizing::new("password".into())).is_ok());
    }
}

#[tokio::test]
async fn text_and_json_responses_are_preserved() {
    for body in ["lno1example", r#"{"balanceSat":123}"#] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = PhoenixdClient::new(
            &format!("http://{}", listener.local_addr().unwrap()),
            Zeroizing::new("password".into()),
        )
        .unwrap();
        let server = async {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_request(&mut stream).await;
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
        };
        let ((), reply) = tokio::join!(
            server,
            client.request(reqwest::Method::GET, &["getbalance"], &[], true)
        );
        match reply.unwrap() {
            Reply::Text(text) => assert_eq!(text, body),
            Reply::Json(value) => assert_eq!(value, json!({"balanceSat":123})),
        };
    }
}
