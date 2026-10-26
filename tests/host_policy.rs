use std::{fs, process::Command};

use openssl::pkey::PKey;

#[test]
fn operator_signer_rejects_untrusted_keys_without_writing_a_policy_or_leaking_the_key() {
    let root = std::env::temp_dir().join(format!("nd-policy-signer-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let output_path = root.join("signed.json");
    let key = PKey::generate_ed25519().unwrap();
    let pem = String::from_utf8(key.private_key_to_pem_pkcs8().unwrap()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_nd-host-policy"))
        .args([
            "--input",
            "policy/host-images.json",
            "--private-key-env",
            "TEST_POLICY_KEY",
            "--output",
        ])
        .arg(&output_path)
        .env("TEST_POLICY_KEY", &pem)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!output_path.exists());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("Signing key does not match"));
    assert!(!text.contains("BEGIN PRIVATE KEY"));
    assert!(!text.contains(pem.lines().nth(1).unwrap()));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn local_policy_override_cannot_supply_unsigned_approvals() {
    let root = std::env::temp_dir().join(format!("nd-policy-unsigned-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    let path = root.join("unsigned.json");
    fs::copy("policy/host-images.json", &path).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_nd"))
        .args([
            "attest",
            "--url",
            "https://127.0.0.1/",
            "--approve-release-sha256",
            &"a".repeat(64),
        ])
        .env("NODANA_HOST_POLICY_FILE", &path)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Invalid signed host approval policy")
    );
    fs::remove_dir_all(root).unwrap();
}
