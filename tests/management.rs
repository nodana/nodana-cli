//! End-to-end requests against a local API stand-in; no hosted resources are created.
use std::{
    io::{Read, Write},
    net::TcpListener,
    process::{Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde_json::{Value, json};

fn invoke(args: &[&str], origin: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_nd"));
    command
        .env("NODANA_API_KEY", "synthetic-api-key")
        .env_remove("NODANA_API_ORIGIN")
        .env_remove("NODANA_PHOENIXD_URL")
        .env_remove("NODANA_PHOENIXD_PASSWORD")
        .stdin(Stdio::null());
    if let Some(origin) = origin {
        command.args(["--api-url", origin]);
    }
    command.args(args).output().unwrap()
}

fn exchange(args: &[&str], status: &str, body: &str) -> (Output, String) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    listener.set_nonblocking(true).unwrap();
    let status = status.to_owned();
    let body = body.to_owned();
    let server = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut socket = loop {
            if let Ok((socket, _)) = listener.accept() {
                break socket;
            }
            assert!(
                Instant::now() < deadline,
                "CLI did not send the expected request"
            );
            thread::sleep(Duration::from_millis(10));
        };
        socket.set_nonblocking(false).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut bytes = Vec::new();
        let mut buffer = [0u8; 4096];
        loop {
            let read = socket.read(&mut buffer).unwrap();
            assert!(read > 0, "Incomplete HTTP request");
            bytes.extend_from_slice(&buffer[..read]);
            if let Some(end) = bytes.windows(4).position(|value| value == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&bytes[..end]);
                let length = headers
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                if bytes.len() >= end + 4 + length {
                    break;
                }
            }
        }
        write!(socket, "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        String::from_utf8(bytes).unwrap()
    });
    let output = invoke(args, Some(&origin));
    (output, server.join().unwrap())
}

fn assert_request(request: &str, method: &str, path: &str, body: Option<Value>) {
    assert!(
        request.starts_with(&format!("{method} /v1/{path} HTTP/1.1\r\n")),
        "{request}"
    );
    assert!(
        request
            .to_lowercase()
            .contains("authorization: bearer synthetic-api-key\r\n")
    );
    assert!(!request.to_lowercase().contains("authorization: basic"));
    let (_, actual_body) = request.split_once("\r\n\r\n").unwrap();
    match body {
        Some(body) => assert_eq!(serde_json::from_str::<Value>(actual_body).unwrap(), body),
        None => assert!(actual_body.is_empty()),
    }
}

#[test]
fn node_creation_prints_one_time_credentials_and_sends_fresh_request_ids() {
    let result = json!({
        "node":{"id":"node-abcd","status":"provisioning"},
        "credentials":{"seed":"synthetic recovery seed","password":"synthetic-full-password","restrictedPassword":"synthetic-limited-password"}
    });
    let mut ids = Vec::new();
    for _ in 0..2 {
        let (output, request) = exchange(
            &[
                "create-node",
                "--name",
                "Payments",
                "--auto-liquidity",
                "5m",
            ],
            "202 Accepted",
            &result.to_string(),
        );
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
            result
        );
        let body: Value = serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
        let id = body["requestId"].as_str().unwrap();
        assert_eq!(id.len(), 32);
        assert!(
            id.bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        );
        assert_request(
            &request,
            "POST",
            "nodes",
            Some(json!({"name":"Payments","autoLiquidity":"5m","requestId":id})),
        );
        ids.push(id.to_owned());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("only once"));
        assert!(!stderr.contains("synthetic-full-password"));
        assert!(!stderr.contains("synthetic recovery seed"));
    }
    assert_ne!(ids[0], ids[1]);
}

#[test]
fn node_lifecycle_and_update_commands_use_documented_methods_and_json() {
    let checksum = "a".repeat(64);
    let cases = vec![
        (vec!["list-nodes"], "GET", "nodes", None),
        (
            vec!["get-node", "node-abcd"],
            "GET",
            "nodes/node-abcd",
            None,
        ),
        (
            vec!["rename-node", "node-abcd", "--name", " Payments "],
            "PATCH",
            "nodes/node-abcd",
            Some(json!({"name":"Payments"})),
        ),
        (
            vec!["start-node", "node-abcd"],
            "POST",
            "nodes/node-abcd/start",
            None,
        ),
        (
            vec!["stop-node", "node-abcd"],
            "POST",
            "nodes/node-abcd/stop",
            None,
        ),
        (
            vec!["restart-node", "node-abcd"],
            "POST",
            "nodes/node-abcd/restart",
            None,
        ),
        (
            vec!["check-node-update", "node-abcd"],
            "GET",
            "nodes/node-abcd/update",
            None,
        ),
        (
            vec!["update-node", "node-abcd", "--release-sha256", &checksum],
            "POST",
            "nodes/node-abcd/update",
            Some(json!({"releaseSha256":checksum})),
        ),
        (
            vec!["delete-node", "node-abcd", "--force"],
            "DELETE",
            "nodes/node-abcd",
            None,
        ),
    ];
    for (args, method, path, body) in cases {
        let (output, request) = exchange(&args, "202 Accepted", "");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_request(&request, method, path, body);
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
            json!({"accepted":true})
        );
    }
}

#[test]
fn webhook_commands_preserve_false_empty_description_and_creation_secret() {
    let cases = vec![
        (
            vec!["get-webhook", "node-abcd"],
            "GET",
            "nodes/node-abcd/webhook-endpoint",
            None,
        ),
        (
            vec![
                "create-webhook",
                "node-abcd",
                "--url",
                "https://receiver.example/events",
            ],
            "POST",
            "nodes/node-abcd/webhook-endpoint",
            Some(json!({"url":"https://receiver.example/events"})),
        ),
        (
            vec![
                "update-webhook",
                "node-abcd",
                "--enabled",
                "false",
                "--description",
                "",
            ],
            "PATCH",
            "nodes/node-abcd/webhook-endpoint",
            Some(json!({"enabled":false,"description":""})),
        ),
        (
            vec!["delete-webhook", "node-abcd"],
            "DELETE",
            "nodes/node-abcd/webhook-endpoint",
            None,
        ),
        (
            vec!["list-webhook-deliveries", "node-abcd"],
            "GET",
            "nodes/node-abcd/webhook-deliveries",
            None,
        ),
    ];
    for (args, method, path, body) in cases {
        let response = if args[0] == "create-webhook" {
            json!({"endpoint":{"id":"webhook-test"},"signingSecret":"synthetic-signing-secret"})
        } else {
            json!({"result":"preserved"})
        };
        let (output, request) = exchange(&args, "200 OK", &response.to_string());
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_request(&request, method, path, body);
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
            response
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("synthetic-signing-secret"));
    }
    for endpoint in [Value::Null, json!({"id":"webhook-test","enabled":false})] {
        let (output, request) = exchange(
            &["list-webhooks", "node-abcd"],
            "200 OK",
            &json!({"endpoint":endpoint}).to_string(),
        );
        assert!(output.status.success());
        assert_request(&request, "GET", "nodes/node-abcd/webhook-endpoint", None);
        let expected = if endpoint.is_null() {
            json!([])
        } else {
            json!([endpoint])
        };
        assert_eq!(
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
            expected
        );
    }
}

#[test]
fn failures_do_not_print_server_secrets_or_claim_success() {
    for status in [
        "401 Unauthorized",
        "403 Forbidden",
        "409 Conflict",
        "429 Too Many Requests",
        "503 Service Unavailable",
    ] {
        let (output, _) = exchange(
            &["create-node"],
            status,
            r#"{"error":"synthetic-api-key synthetic-secret"}"#,
        );
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).unwrap();
        assert!(!error.contains("synthetic-api-key"));
        assert!(!error.contains("synthetic-secret"));
    }
    for args in [
        vec!["create-node"],
        vec![
            "create-webhook",
            "node-abcd",
            "--url",
            "https://receiver.example/events",
        ],
    ] {
        let (output, _) = exchange(&args, "202 Accepted", "");
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("no creation details"));
    }
}

#[test]
fn root_attestation_does_not_require_account_or_phoenixd_credentials() {
    let output = Command::new(env!("CARGO_BIN_EXE_nd"))
        .env_remove("NODANA_API_KEY")
        .env_remove("NODANA_PHOENIXD_PASSWORD")
        .env_remove("NODANA_PHOENIXD_URL")
        .args([
            "attest",
            "--url",
            "http://127.0.0.1:1",
            "--approve-release-sha256",
            &"a".repeat(64),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("Attestation requires your node's HTTPS URL"));
    assert!(!error.contains("NODANA_API_KEY"));
    assert!(!error.contains("NODANA_PHOENIXD_PASSWORD"));
}

#[test]
fn invalid_input_and_unconfirmed_deletion_make_no_api_request() {
    for args in [
        vec!["delete-node", "node-abcd"],
        vec!["update-webhook", "node-abcd"],
        vec!["get-node", "../secret"],
        vec!["create-node", "--auto-liquidity", "off"],
        vec!["create-node", "--name", "This name is too long"],
        vec![
            "create-webhook",
            "node-abcd",
            "--url",
            "http://receiver.example",
        ],
        vec![
            "create-webhook",
            "node-abcd",
            "--url",
            "https://secret@receiver.example",
        ],
        vec!["update-node", "node-abcd", "--release-sha256", "latest"],
    ] {
        let output = invoke(&args, Some("http://127.0.0.1:1"));
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("Could not reach"));
    }
}
