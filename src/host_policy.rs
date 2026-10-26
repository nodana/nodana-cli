use std::time::Duration;

use anyhow::{Context, Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use openssl::{pkey::PKey, sign::Verifier};
use serde::{Deserialize, Serialize};

pub const POLICY_URL: &str = "https://github.com/nodana/nodana-phoenixd/releases/download/host-policy-v1/nodana-host-policy.json";
pub const SIGNING_DOMAIN: &[u8] = b"nodana-host-policy-v1\0";
pub const MAX_POLICY_BYTES: usize = 96 * 1024;
pub const PUBLIC_KEY: &[u8] = include_bytes!("../policy/signing-public-key.pem");

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApprovedImage {
    pub image_id: String,
    pub pcr4: String,
    pub pcr7: String,
    pub pcr12: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Policy {
    pub version: u8,
    pub serial: u64,
    pub images: Vec<ApprovedImage>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedPolicy {
    pub payload: String,
    pub signature: String,
}

impl Policy {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.version == 1 && self.serial > 0,
            "Unsupported host approval policy"
        );
        ensure!(self.images.len() <= 32, "Invalid host approval policy");
        let mut ids = std::collections::BTreeSet::new();
        let mut measurements = std::collections::BTreeSet::new();
        for image in &self.images {
            let id = image.image_id.strip_prefix("ami-").unwrap_or_default();
            ensure!(id.len() == 17 && is_hex(id), "Invalid approved AMI ID");
            for pcr in [&image.pcr4, &image.pcr7, &image.pcr12] {
                ensure!(
                    pcr.len() == 96 && is_hex(pcr),
                    "Invalid approved boot measurement"
                );
            }
            ensure!(
                image.pcr4 != "00".repeat(48) && image.pcr7 != "00".repeat(48),
                "Empty approved boot measurement"
            );
            ensure!(
                ids.insert(&image.image_id)
                    && measurements.insert((&image.pcr4, &image.pcr7, &image.pcr12)),
                "Duplicate approved image"
            );
        }
        Ok(())
    }
}

fn is_hex(value: &str) -> bool {
    value
        .bytes()
        .all(|v| v.is_ascii_digit() || (b'a'..=b'f').contains(&v))
}

pub fn verify(bytes: &[u8], public_key: &[u8]) -> Result<Policy> {
    ensure!(
        bytes.len() <= MAX_POLICY_BYTES,
        "Host approval policy is too large"
    );
    let envelope: SignedPolicy =
        serde_json::from_slice(bytes).context("Invalid signed host approval policy")?;
    let payload = STANDARD
        .decode(&envelope.payload)
        .context("Invalid host policy encoding")?;
    let signature = STANDARD
        .decode(&envelope.signature)
        .context("Invalid host policy signature encoding")?;
    ensure!(
        signature.len() == 64
            && STANDARD.encode(&payload) == envelope.payload
            && STANDARD.encode(&signature) == envelope.signature,
        "Invalid host policy encoding"
    );
    let key = PKey::public_key_from_pem(public_key)?;
    ensure!(
        key.id() == openssl::pkey::Id::ED25519,
        "Invalid host policy signing key"
    );
    let mut message = SIGNING_DOMAIN.to_vec();
    message.extend(&payload);
    ensure!(
        Verifier::new_without_digest(&key)?.verify_oneshot(&signature, &message)?,
        "Host approval policy signature is invalid"
    );
    let policy: Policy =
        serde_json::from_slice(&payload).context("Invalid host approval policy")?;
    policy.validate()?;
    Ok(policy)
}

pub async fn load() -> Result<Policy> {
    // An explicit local policy still requires the same trusted signature.
    if let Some(path) = std::env::var_os("NODANA_HOST_POLICY_FILE") {
        let mut file =
            std::fs::File::open(path).context("Could not open signed host approval policy")?;
        let mut bytes = Vec::new();
        use std::io::Read;
        Read::by_ref(&mut file)
            .take(MAX_POLICY_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        return verify(&bytes, PUBLIC_KEY);
    }
    let client = policy_client()?;
    download(&client, POLICY_URL, PUBLIC_KEY).await
}

fn policy_client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        // GitHub release downloads redirect to its asset service. No credentials
        // are attached; every response still requires the pinned policy signature.
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() > 3 {
                attempt.error("Too many host policy redirects")
            } else if attempt.url().username().is_empty()
                && attempt.url().password().is_none()
                && attempt.url().scheme() == "https"
                && matches!(
                    attempt.url().host_str(),
                    Some(
                        "github.com"
                            | "release-assets.githubusercontent.com"
                            | "objects.githubusercontent.com"
                    )
                )
            {
                attempt.follow()
            } else {
                attempt.error("Unexpected host policy redirect")
            }
        }))
        .build()?)
}

async fn download(client: &reqwest::Client, url: &str, public_key: &[u8]) -> Result<Policy> {
    let mut response = client
        .get(url)
        .header(reqwest::header::CACHE_CONTROL, "no-cache")
        .send()
        .await
        .context("Could not download Nodana's signed host approval policy; try again later")?;
    ensure!(
        response.status().is_success(),
        "Nodana's host approval policy is unavailable; try again later"
    );
    ensure!(
        response
            .content_length()
            .is_none_or(|v| v <= MAX_POLICY_BYTES as u64),
        "Host approval policy is too large"
    );
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        ensure!(
            bytes.len() + chunk.len() <= MAX_POLICY_BYTES,
            "Host approval policy is too large"
        );
        bytes.extend_from_slice(&chunk);
    }
    verify(&bytes, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::sign::Signer;

    fn policy() -> Policy {
        Policy {
            version: 1,
            serial: 1,
            images: vec![ApprovedImage {
                image_id: "ami-03f3b551d17c25f49".into(),
                pcr4: "ab".repeat(48),
                pcr7: "cd".repeat(48),
                pcr12: "00".repeat(48),
                release: None,
            }],
        }
    }

    fn sign(policy: &Policy, key: &PKey<openssl::pkey::Private>, domain: &[u8]) -> Vec<u8> {
        let payload = serde_json::to_vec(policy).unwrap();
        let mut message = domain.to_vec();
        message.extend(&payload);
        serde_json::to_vec(&SignedPolicy {
            payload: STANDARD.encode(&payload),
            signature: STANDARD.encode(
                Signer::new_without_digest(key)
                    .unwrap()
                    .sign_oneshot_to_vec(&message)
                    .unwrap(),
            ),
        })
        .unwrap()
    }

    #[test]
    fn signed_policy_checks_signature_key_domain_and_exact_payload() {
        let key = PKey::generate_ed25519().unwrap();
        let public = key.public_key_to_pem().unwrap();
        let bytes = sign(&policy(), &key, SIGNING_DOMAIN);
        assert_eq!(
            verify(&bytes, &public).unwrap().images[0].image_id,
            "ami-03f3b551d17c25f49"
        );
        let other = PKey::generate_ed25519().unwrap();
        assert!(verify(&bytes, &other.public_key_to_pem().unwrap()).is_err());
        assert!(verify(&sign(&policy(), &key, b"nodana-node-bundle-v1\0"), &public,).is_err());
        let mut altered: SignedPolicy = serde_json::from_slice(&bytes).unwrap();
        altered.payload = STANDARD.encode(
            serde_json::to_vec(&Policy {
                serial: 2,
                ..policy()
            })
            .unwrap(),
        );
        assert!(verify(&serde_json::to_vec(&altered).unwrap(), &public).is_err());
        assert!(verify(&serde_json::to_vec(&policy()).unwrap(), &public).is_err());
        assert!(verify(&vec![b' '; MAX_POLICY_BYTES + 1], &public).is_err());
    }

    #[test]
    fn signed_policy_rejects_unsupported_versions_and_missing_serial() {
        let key = PKey::generate_ed25519().unwrap();
        let public = key.public_key_to_pem().unwrap();
        for invalid in [
            Policy {
                version: 2,
                ..policy()
            },
            Policy {
                serial: 0,
                ..policy()
            },
        ] {
            assert!(verify(&sign(&invalid, &key, SIGNING_DOMAIN), &public).is_err());
        }
    }

    #[test]
    fn policy_rejects_invalid_and_duplicate_measurements() {
        let mut value = policy();
        value.images[0].pcr4 = "00".repeat(48);
        assert!(value.validate().is_err());
        value = policy();
        value.images.push(value.images[0].clone());
        assert!(value.validate().is_err());
        value = policy();
        value.images[0].image_id = "not-an-ami".into();
        assert!(value.validate().is_err());
        value = policy();
        value.images[0].pcr7 = "ab".repeat(47);
        assert!(value.validate().is_err());
        value.images.clear();
        value.validate().unwrap(); // A signed empty policy withdraws every approval.
    }

    #[test]
    fn reviewed_production_measurements_are_valid_policy_input() {
        let input: serde_json::Value =
            serde_json::from_str(include_str!("../policy/host-images.json")).unwrap();
        let mut value = policy();
        value.images = serde_json::from_value(input["images"].clone()).unwrap();
        value.validate().unwrap();
        assert_eq!(value.images[0].image_id, "ami-03f3b551d17c25f49");
        assert_eq!(
            PKey::public_key_from_pem(PUBLIC_KEY).unwrap().id(),
            openssl::pkey::Id::ED25519
        );
    }

    #[tokio::test]
    async fn next_download_uses_replaced_policy_including_withdrawn_approvals() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let key = PKey::generate_ed25519().unwrap();
        let public = key.public_key_to_pem().unwrap();
        let original = sign(&policy(), &key, SIGNING_DOMAIN);
        let replacement = sign(
            &Policy {
                serial: 2,
                images: vec![],
                ..policy()
            },
            &key,
            SIGNING_DOMAIN,
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for body in [original, replacement] {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = vec![0; 4096];
                let length = socket.read(&mut request).await.unwrap();
                assert!(
                    String::from_utf8_lossy(&request[..length])
                        .to_ascii_lowercase()
                        .contains("cache-control: no-cache")
                );
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                socket.write_all(header.as_bytes()).await.unwrap();
                socket.write_all(&body).await.unwrap();
            }
        });
        let client = policy_client().unwrap();
        let url = format!("http://{address}/policy");
        assert_eq!(
            download(&client, &url, &public).await.unwrap().images.len(),
            1
        );
        let next = download(&client, &url, &public).await.unwrap();
        assert_eq!(next.serial, 2);
        assert!(next.images.is_empty());
        server.await.unwrap();
    }

    #[tokio::test]
    async fn download_requires_signed_policy_and_rejects_redirects_errors_and_oversized_bodies() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let key = PKey::generate_ed25519().unwrap();
        let public = key.public_key_to_pem().unwrap();
        let client = policy_client().unwrap();
        for (status, body, accepted) in [
            ("200 OK", sign(&policy(), &key, SIGNING_DOMAIN), true),
            ("200 OK", serde_json::to_vec(&policy()).unwrap(), false),
            ("200 OK", vec![b'x'; MAX_POLICY_BYTES + 1], false),
            ("302 Found", vec![], false),
            ("503 Service Unavailable", vec![], false),
        ] {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = vec![0; 4096];
                let length = socket.read(&mut request).await.unwrap();
                let request = String::from_utf8_lossy(&request[..length]).to_ascii_lowercase();
                assert!(!request.contains("authorization:"));
                let header = format!(
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\nLocation: http://127.0.0.1:1/\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                socket.write_all(header.as_bytes()).await.unwrap();
                let _ = socket.write_all(&body).await;
            });
            assert_eq!(
                download(&client, &format!("http://{address}/policy"), &public)
                    .await
                    .is_ok(),
                accepted
            );
            server.await.unwrap();
        }
    }
}
