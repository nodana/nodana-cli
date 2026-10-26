//! Operator tool: sign and check reviewed host approvals.
use std::{fs, io::Write, path::PathBuf};

use anyhow::{Context, Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::{ArgGroup, Parser};
use nd::host_policy::{PUBLIC_KEY, Policy, SIGNING_DOMAIN, SignedPolicy, verify};
use openssl::{pkey::PKey, sign::Signer};

#[derive(Parser)]
#[command(group(ArgGroup::new("source").required(true).args(["input", "check"])))]
struct Args {
    #[arg(long)]
    input: Option<PathBuf>,
    /// Verify a signed policy without using a private key
    #[arg(long)]
    check: Option<PathBuf>,
    #[arg(long, required_unless_present = "check")]
    output: Option<PathBuf>,
    #[arg(long, required_unless_present = "check")]
    private_key_env: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(path) = args.check {
        let policy = verify(&fs::read(path)?, PUBLIC_KEY)?;
        println!(
            "Verified host policy {} ({} images)",
            policy.serial,
            policy.images.len()
        );
        return Ok(());
    }
    let policy: Policy =
        serde_json::from_slice(&fs::read(args.input.context("Policy input required")?)?)?;
    policy.validate()?;
    let private = std::env::var(
        args.private_key_env
            .context("Private-key environment variable required")?,
    )
    .context("Set the private-key environment variable")?;
    let key = PKey::private_key_from_pem(private.as_bytes()).context("Invalid signing key")?;
    let public = PKey::public_key_from_pem(PUBLIC_KEY)?;
    ensure!(
        key.id() == openssl::pkey::Id::ED25519
            && key.raw_public_key()? == public.raw_public_key()?,
        "Signing key does not match the CLI's trusted public key"
    );
    let payload = serde_json::to_vec(&policy)?;
    let mut message = SIGNING_DOMAIN.to_vec();
    message.extend(&payload);
    let bytes = serde_json::to_vec_pretty(&SignedPolicy {
        payload: STANDARD.encode(&payload),
        signature: STANDARD
            .encode(Signer::new_without_digest(&key)?.sign_oneshot_to_vec(&message)?),
    })?;
    verify(&bytes, PUBLIC_KEY)?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(args.output.context("Output path required")?)?
        .write_all(&bytes)?;
    println!(
        "Signed host policy {} ({} images)",
        policy.serial,
        policy.images.len()
    );
    Ok(())
}
