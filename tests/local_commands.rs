use base64::{Engine, engine::general_purpose::STANDARD};
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    thread,
    time::{Duration, Instant},
};

fn mock_node(password: &str) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let expected = format!(
        "authorization: Basic {}",
        STANDARD.encode(format!(":{password}"))
    );
    let handle = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "CLI never contacted mock node");
                    thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("{error}"),
            }
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut bytes = Vec::new();
        loop {
            let mut buffer = [0; 4096];
            let count = stream.read(&mut buffer).unwrap();
            assert!(count > 0);
            bytes.extend_from_slice(&buffer[..count]);
            if bytes.windows(4).any(|v| v == b"\r\n\r\n") {
                break;
            }
        }
        let headers = String::from_utf8(bytes).unwrap();
        assert!(headers.starts_with("GET /getbalance HTTP/1.1"));
        assert!(headers.lines().any(|line| line == expected));
        let body = r#"{"balanceSat":123,"feeCreditSat":0}"#;
        stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                )
                .as_bytes(),
            )
            .unwrap();
    });
    (url, handle)
}

#[test]
fn dotenv_password_exported_environment_and_flag_precedence_work_without_saved_credentials() {
    let root = tempfile::tempdir().unwrap();
    let child = root.path().join("child");
    fs::create_dir(&child).unwrap();
    fs::write(
        root.path().join(".env"),
        "NODANA_PHOENIXD_PASSWORD=dotenv-password\n",
    )
    .unwrap();
    for (exported, flag, expected) in [
        (None, None, "dotenv-password"),
        (Some("environment-password"), None, "environment-password"),
        (
            Some("environment-password"),
            Some("flag-password"),
            "flag-password",
        ),
    ] {
        let (url, server) = mock_node(expected);
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_ndn"));
        cmd.current_dir(&child)
            .env_remove("NODANA_PHOENIXD_PASSWORD")
            .env("NODANA_CLI_DIR", root.path().join("unused"))
            .args(["--plain", "--url", &url]);
        if let Some(value) = exported {
            cmd.env("NODANA_PHOENIXD_PASSWORD", value);
        }
        if let Some(value) = flag {
            cmd.args(["--password", value]);
        }
        let result = cmd.arg("get-balance").output().unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap()["balanceSat"],
            123
        );
        assert!(!String::from_utf8_lossy(&result.stdout).contains(expected));
        assert!(!String::from_utf8_lossy(&result.stderr).contains(expected));
        assert!(!root.path().join("unused").exists());
        server.join().unwrap();
    }
}

#[test]
fn missing_credentials_bad_dotenv_and_removed_login_fail_without_exposing_secrets() {
    let temp = tempfile::tempdir().unwrap();
    for args in [
        vec!["--url", "http://127.0.0.1:9", "get-info"],
        vec!["login"],
        vec!["logout"],
        vec!["connect", "conn-test"],
        vec!["generate-key"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_ndn"))
            .current_dir(temp.path())
            .env_remove("NODANA_PHOENIXD_PASSWORD")
            .args(&args)
            .output()
            .unwrap();
        assert!(!output.status.success());
        if args[0] == "--url" {
            assert!(String::from_utf8_lossy(&output.stderr).contains("NODANA_PHOENIXD_PASSWORD"));
        }
    }
    fs::write(temp.path().join(".env"), "INVALID SECRET VALUE\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ndn"))
        .current_dir(temp.path())
        .arg("get-info")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Could not load .env"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("SECRET"));
}

#[test]
fn help_does_not_need_credentials_and_lists_only_native_commands() {
    let temp = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ndn"))
        .current_dir(temp.path())
        .env_remove("NODANA_PHOENIXD_PASSWORD")
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    for command in [
        "get-balance",
        "create-offer",
        "lnurl-withdraw",
        "export-payments",
    ] {
        assert!(text.contains(command));
    }
    for command in ["login", "logout", "generate-key", "list-connections"] {
        assert!(!text.contains(command));
    }
}

#[test]
fn url_uses_dotenv_then_exported_environment_then_explicit_flag() {
    let root = tempfile::tempdir().unwrap();
    let child = root.path().join("child");
    fs::create_dir(&child).unwrap();
    for source in ["dotenv", "environment", "flag"] {
        let (url, server) = mock_node("node-password");
        let dotenv_url = if source == "dotenv" {
            url.as_str()
        } else {
            "invalid-dotenv-url"
        };
        fs::write(
            root.path().join(".env"),
            format!("NODANA_PHOENIXD_URL={dotenv_url}\n"),
        )
        .unwrap();
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_ndn"));
        cmd.current_dir(&child)
            .env("NODANA_PHOENIXD_PASSWORD", "node-password")
            .env_remove("NODANA_PHOENIXD_URL");
        match source {
            "environment" => {
                cmd.env("NODANA_PHOENIXD_URL", &url);
            }
            "flag" => {
                cmd.env("NODANA_PHOENIXD_URL", "invalid-environment-url")
                    .args(["--url", &url]);
            }
            _ => {}
        }
        let output = cmd.arg("get-balance").output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        server.join().unwrap();
    }
}

#[test]
fn missing_or_empty_url_reports_setup_error_without_contacting_a_node() {
    let root = tempfile::tempdir().unwrap();
    for url in [None, Some("")] {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_ndn"));
        cmd.current_dir(root.path())
            .env("NODANA_PHOENIXD_PASSWORD", "node-password")
            .env_remove("NODANA_PHOENIXD_URL");
        if let Some(url) = url {
            cmd.env("NODANA_PHOENIXD_URL", url);
        }
        let output = cmd.arg("get-info").output().unwrap();
        assert!(!output.status.success());
        let error = String::from_utf8_lossy(&output.stderr);
        if url.is_none() {
            assert!(error.contains("NODANA_PHOENIXD_URL"));
        } else {
            assert!(error.contains("--url"));
        }
        assert!(!error.contains("node-password"));
    }
}
