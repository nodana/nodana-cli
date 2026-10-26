//! Local verification of AWS NitroTPM evidence and its public TLS binding.
use std::{
    collections::BTreeSet,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use ciborium::value::Value;
use nd::host_policy::{self, ApprovedImage};
use openssl::{
    bn::BigNum,
    ecdsa::EcdsaSig,
    hash::MessageDigest,
    sha::sha256,
    stack::Stack,
    x509::{X509, X509StoreContext, store::X509StoreBuilder},
};
use reqwest::{Url, tls::TlsInfo};
use serde::{Deserialize, Serialize};

fn validate_checksum(value: &str) -> Result<()> {
    ensure!(
        value.len() == 64
            && value
                .bytes()
                .all(|v| v.is_ascii_digit() || (b'a'..=b'f').contains(&v)),
        "Supply the independently approved 64-character lowercase manifest checksum"
    );
    Ok(())
}

const AWS_ROOT: &str = "641a0321a3e244efe456463195d606317ed7cdcc3c1756e09893f3c68f79bb5b";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Evidence {
    certificate: String,
    attestation_document: String,
    identity: Identity,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Identity {
    version: u8,
    hostname: String,
    node_id: String,
    node_public_key: String,
    owner_public_key: String,
    certificate_sha256: String,
    release: ReleaseIdentity,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReleaseIdentity {
    release_sha256: String,
    version: String,
    phoenixd_version: String,
    state_format: String,
}

#[derive(Clone, Copy)]
struct Approvals<'a> {
    release: Option<&'a str>,
    host: Option<&'a str>,
}

impl Approvals<'_> {
    fn validate(self) -> Result<()> {
        ensure!(
            self.release.is_some() || self.host.is_some(),
            "Approve at least one host or node bundle checksum"
        );
        if let Some(value) = self.release {
            validate_checksum(value).context("Invalid node bundle checksum")?;
        }
        if let Some(value) = self.host {
            validate_checksum(value).context("Invalid host release checksum")?;
        }
        Ok(())
    }

    fn images(self, images: &[ApprovedImage]) -> Result<Vec<ApprovedImage>> {
        let mut approved = Vec::new();
        for image in images {
            if let Some(sha) = self.host
                && image.host_release_sha256()? != sha
            {
                continue;
            }
            approved.push(image.clone());
        }
        ensure!(
            self.host.is_none() || !approved.is_empty(),
            "The approved host release checksum does not match any image in Nodana's signed policy"
        );
        Ok(approved)
    }
}

pub async fn run(url: Url, json: bool, release: Option<&str>, host: Option<&str>) -> Result<()> {
    let approvals = Approvals { release, host };
    approvals.validate()?;
    ensure!(
        url.scheme() == "https" && url.port_or_known_default() == Some(443) && url.path() == "/",
        "Attestation requires your node's HTTPS URL on port 443"
    );
    let policy = host_policy::load().await?;
    let mut nonce = [0; 32];
    openssl::rand::rand_bytes(&mut nonce)?;
    let response = crate::api::http_client()?
        .post(url.join("attestation")?)
        .json(&serde_json::json!({"nonce": STANDARD.encode(nonce)}))
        .send()
        .await
        .context("Could not request host attestation")?;
    crate::api::check_status(&response)?;
    let peer = response
        .extensions()
        .get::<TlsInfo>()
        .and_then(TlsInfo::peer_certificate)
        .context("TLS peer certificate is unavailable")?
        .to_vec();
    let body = crate::api::read_limited(response, 128 * 1024).await?;
    let evidence: Evidence =
        serde_json::from_slice(&body).context("Invalid attestation response")?;
    let result = verify_evidence(
        &evidence,
        &url,
        &peer,
        &nonce,
        now()?,
        &policy.images,
        approvals,
    )?;
    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "Verified attestation for {}",
            result["hostname"].as_str().unwrap_or_default()
        );
        if approvals.host.is_some() {
            println!(
                "Host release: approved checksum matched ({})",
                result["hostReleaseSha256"].as_str().unwrap_or_default()
            );
        } else {
            println!("Host: matches Nodana's signed approvals");
        }
        if approvals.release.is_some() {
            println!(
                "Node bundle: approved checksum matched ({})",
                result["release"]["releaseSha256"]
                    .as_str()
                    .unwrap_or_default()
            );
        } else {
            println!(
                "Node bundle: reported by the verified host; no bundle checksum was requested"
            );
        }
    }
    Ok(())
}

fn now() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .try_into()?)
}

fn verify_evidence(
    evidence: &Evidence,
    url: &Url,
    peer: &[u8],
    nonce: &[u8],
    now: u64,
    images: &[ApprovedImage],
    approvals: Approvals<'_>,
) -> Result<serde_json::Value> {
    let identity = &evidence.identity;
    ensure!(
        identity.version == 2
            && Some(identity.hostname.as_str()) == url.host_str()
            && !identity.node_id.is_empty()
            && !identity.node_public_key.is_empty()
            && !identity.owner_public_key.is_empty(),
        "Unexpected node identity"
    );
    approvals.validate()?;
    if let Some(approval) = approvals.release {
        ensure!(
            identity.release.release_sha256 == approval,
            "The running node release has not been approved"
        );
    }
    ensure!(
        hex::encode(sha256(evidence.certificate.as_bytes())) == identity.certificate_sha256,
        "Certificate digest does not match the proof"
    );
    let certs = X509::stack_from_pem(evidence.certificate.as_bytes())?;
    ensure!(
        !certs.is_empty() && certs[0].to_der()? == peer,
        "Attestation does not cover this TLS connection"
    );
    // serde_json's default map is sorted; convert the struct to a Value before encoding.
    let canonical = serde_json::to_vec(&serde_json::to_value(identity)?)?;
    let mut binding = b"nodana-public-tls-v1\0".to_vec();
    binding.extend(canonical);
    let document = decode_base64(&evidence.attestation_document, 65536)?;
    let images = approvals.images(images)?;
    let (module, image) = verify_document(&document, nonce, &sha256(&binding), now, &images)?;
    let host_sha256 = images
        .iter()
        .find(|entry| entry.image_id == image)
        .context("Missing verified host image")?
        .host_release_sha256()?;
    Ok(
        serde_json::json!({"verified": true, "hostname": identity.hostname, "nodeId": identity.node_id, "moduleId": module, "imageId": image,"hostReleaseSha256":host_sha256,"hostPinned":approvals.host.is_some(),"bundlePinned":approvals.release.is_some(),"release":identity.release}),
    )
}

fn decode_base64(text: &str, maximum: usize) -> Result<Vec<u8>> {
    ensure!(
        text.len() <= maximum.div_ceil(3) * 4,
        "Attestation document is too large"
    );
    let bytes = STANDARD
        .decode(text)
        .context("Invalid attestation encoding")?;
    ensure!(
        bytes.len() <= maximum && STANDARD.encode(&bytes) == text,
        "Invalid attestation encoding"
    );
    Ok(bytes)
}

fn decode(bytes: &[u8]) -> Result<Value> {
    let mut cursor = std::io::Cursor::new(bytes);
    let value: Value = ciborium::from_reader(&mut cursor).context("Invalid attestation CBOR")?;
    ensure!(
        cursor.position() == bytes.len() as u64,
        "Trailing attestation data"
    );
    validate_maps(&value, 0)?;
    Ok(value)
}

fn validate_maps(value: &Value, depth: usize) -> Result<()> {
    ensure!(depth < 32, "Attestation nesting is too deep");
    match value {
        Value::Map(entries) => {
            let mut seen = BTreeSet::new();
            for (key, value) in entries {
                let mut encoded = Vec::new();
                ciborium::into_writer(key, &mut encoded)?;
                ensure!(seen.insert(encoded), "Duplicate attestation map key");
                validate_maps(key, depth + 1)?;
                validate_maps(value, depth + 1)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                validate_maps(value, depth + 1)?;
            }
        }
        Value::Tag(_, value) => validate_maps(value, depth + 1)?,
        _ => {}
    }
    Ok(())
}

fn field<'a>(value: &'a Value, name: &str) -> Result<&'a Value> {
    value
        .as_map()
        .and_then(|entries| {
            entries
                .iter()
                .find_map(|(key, value)| (key.as_text() == Some(name)).then_some(value))
        })
        .with_context(|| format!("Missing attestation field {name}"))
}
fn bytes(value: &Value, min: usize, max: usize) -> Result<&[u8]> {
    let value = value.as_bytes().context("Invalid attestation bytes")?;
    ensure!(
        (min..=max).contains(&value.len()),
        "Invalid attestation byte length"
    );
    Ok(value)
}
fn integer(value: &Value) -> Result<i128> {
    Ok(value
        .as_integer()
        .context("Invalid attestation integer")?
        .into())
}

fn verify_document(
    document: &[u8],
    nonce: &[u8],
    binding: &[u8],
    now: u64,
    images: &[ApprovedImage],
) -> Result<(String, String)> {
    ensure!(
        document.len() <= 65536 && nonce.len() == 32 && binding.len() == 32 && !images.is_empty(),
        "Host verification is not configured"
    );
    let mut cose = decode(document)?;
    if let Value::Tag(tag, inner) = cose {
        ensure!(tag == 18, "Invalid COSE tag");
        cose = *inner;
    }
    let cose = cose.as_array().context("Invalid COSE document")?;
    ensure!(cose.len() == 4, "Invalid COSE document");
    let protected = bytes(&cose[0], 1, 128)?;
    let header = decode(protected)?;
    let header = header.as_map().context("Invalid COSE header")?;
    ensure!(
        header.len() == 1
            && integer(&header[0].0)? == 1
            && integer(&header[0].1)? == -35
            && cose[1].as_map().is_some_and(|v| v.is_empty()),
        "Unsupported attestation signature"
    );
    let payload_bytes = bytes(&cose[2], 1, 65536)?;
    let payload = decode(payload_bytes)?;
    let signature = bytes(&cose[3], 96, 96)?;
    let module = field(&payload, "module_id")?
        .as_text()
        .context("Invalid host identity")?;
    let timestamp = integer(field(&payload, "timestamp")?)?;
    ensure!(
        !module.is_empty()
            && module.len() <= 256
            && field(&payload, "digest")?.as_text() == Some("SHA384")
            && timestamp >= i128::from(now) - 300_000
            && timestamp <= i128::from(now) + 30_000,
        "Host attestation has expired or is invalid"
    );
    ensure!(
        bytes(field(&payload, "nonce")?, 32, 32)? == nonce
            && bytes(field(&payload, "user_data")?, 32, 32)? == binding,
        "Host response does not match this request"
    );
    let pcrs = field(&payload, "nitrotpm_pcrs")?
        .as_map()
        .context("Invalid boot measurements")?;
    let pcr = |index: i128| -> Result<String> {
        let value = pcrs
            .iter()
            .find_map(|(key, value)| (integer(key).ok() == Some(index)).then_some(value))
            .context("Missing boot measurement")?;
        Ok(hex::encode(bytes(value, 48, 48)?))
    };
    let (pcr4, pcr7, pcr12) = (pcr(4)?, pcr(7)?, pcr(12)?);
    let image = images
        .iter()
        .find(|image| image.pcr4 == pcr4 && image.pcr7 == pcr7 && image.pcr12 == pcr12)
        .context("This host image is not in Nodana's current signed approval policy. Check the node URL or contact Nodana")?;
    let bundle = field(&payload, "cabundle")?
        .as_array()
        .context("Invalid attestation chain")?;
    ensure!((1..=8).contains(&bundle.len()), "Invalid attestation chain");
    let root_der = bytes(&bundle[0], 1, 2048)?;
    ensure!(
        hex::encode(sha256(root_der)) == AWS_ROOT,
        "Untrusted attestation authority"
    );
    let root = X509::from_der(root_der)?;
    let leaf = X509::from_der(bytes(field(&payload, "certificate")?, 1, 2048)?)?;
    let mut store = X509StoreBuilder::new()?;
    store.add_cert(root)?;
    let mut params = openssl::x509::verify::X509VerifyParam::new()?;
    params.set_time((now / 1000).try_into()?);
    store.set_param(&params)?;
    let mut chain = Stack::new()?;
    for item in &bundle[1..] {
        chain.push(X509::from_der(bytes(item, 1, 2048)?)?)?;
    }
    let mut context = X509StoreContext::new()?;
    ensure!(
        context.init(&store.build(), &leaf, &chain, |ctx| ctx.verify_cert())?,
        "Invalid attestation certificate chain"
    );
    let key = leaf.public_key()?;
    ensure!(
        key.ec_key()?.group().curve_name() == Some(openssl::nid::Nid::SECP384R1),
        "Unsupported attestation signing key"
    );
    let der_signature = EcdsaSig::from_private_components(
        BigNum::from_slice(&signature[..48])?,
        BigNum::from_slice(&signature[48..])?,
    )?
    .to_der()?;
    let signed = Value::Array(vec![
        Value::Text("Signature1".into()),
        Value::Bytes(protected.into()),
        Value::Bytes(Vec::new()),
        Value::Bytes(payload_bytes.into()),
    ]);
    let mut signed_bytes = Vec::new();
    ciborium::into_writer(&signed, &mut signed_bytes)?;
    let mut verifier = openssl::sign::Verifier::new(MessageDigest::sha384(), &key)?;
    verifier.update(&signed_bytes)?;
    if !verifier.verify(&der_signature)? {
        bail!("Invalid host attestation signature");
    }
    Ok((module.into(), image.image_id.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_public_evidence_supports_host_bundle_or_both_at_capture_time() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/public-attestation.json"))
                .unwrap();
        let mut evidence: Evidence = serde_json::from_value(fixture["evidence"].clone()).unwrap();
        let image: ApprovedImage = serde_json::from_value(fixture["image"].clone()).unwrap();
        let host = image.host_release_sha256().unwrap();
        let bundle = evidence.identity.release.release_sha256.clone();
        let url = Url::parse(&format!("https://{}/", evidence.identity.hostname)).unwrap();
        let peer = X509::stack_from_pem(evidence.certificate.as_bytes()).unwrap()[0]
            .to_der()
            .unwrap();
        let nonce: [u8; 32] = decode_base64(fixture["nonce"].as_str().unwrap(), 32)
            .unwrap()
            .try_into()
            .unwrap();
        let doc = decode_base64(&evidence.attestation_document, 65536).unwrap();
        let cose = decode(&doc).unwrap();
        let cose = match &cose {
            Value::Tag(_, value) => value.as_ref(),
            value => value,
        };
        let payload = decode(cose.as_array().unwrap()[2].as_bytes().unwrap()).unwrap();
        let capture_time: u64 = integer(field(&payload, "timestamp").unwrap())
            .unwrap()
            .try_into()
            .unwrap();
        for approvals in [
            Approvals {
                release: Some(&bundle),
                host: None,
            },
            Approvals {
                release: None,
                host: Some(&host),
            },
            Approvals {
                release: Some(&bundle),
                host: Some(&host),
            },
        ] {
            let result = verify_evidence(
                &evidence,
                &url,
                &peer,
                &nonce,
                capture_time,
                std::slice::from_ref(&image),
                approvals,
            )
            .unwrap();
            assert_eq!(result["hostPinned"], approvals.host.is_some());
            assert_eq!(result["bundlePinned"], approvals.release.is_some());
            assert_eq!(result["hostReleaseSha256"], host);
        }
        for approvals in [
            Approvals {
                release: None,
                host: None,
            },
            Approvals {
                release: Some(&"f".repeat(64)),
                host: Some(&host),
            },
            Approvals {
                release: Some(&bundle),
                host: Some(&"f".repeat(64)),
            },
        ] {
            assert!(
                verify_evidence(
                    &evidence,
                    &url,
                    &peer,
                    &nonce,
                    capture_time,
                    std::slice::from_ref(&image),
                    approvals
                )
                .is_err()
            );
        }
        // Even without a bundle pin, changing the reported bundle breaks the
        // AWS-signed binding to this host, node, certificate and challenge.
        evidence.identity.release.release_sha256 = "e".repeat(64);
        assert!(
            verify_evidence(
                &evidence,
                &url,
                &peer,
                &nonce,
                capture_time,
                &[image],
                Approvals {
                    release: None,
                    host: Some(&host)
                }
            )
            .is_err()
        );
    }

    #[test]
    fn approval_is_explicit_and_exact() {
        assert!(validate_checksum(&"a".repeat(64)).is_ok());
        for value in ["latest".into(), "A".repeat(64), "a".repeat(63)] {
            assert!(validate_checksum(&value).is_err());
        }
    }

    fn fixture() -> (serde_json::Value, Vec<ApprovedImage>) {
        // Public AWS-signed evidence, shared with the SDK verifier tests.
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/nitrotpm.json")).unwrap();
        let image = serde_json::from_value(fixture["approvedImage"].clone()).unwrap();
        (fixture, vec![image])
    }

    #[test]
    fn verifies_real_aws_evidence_at_capture_time() {
        let (fixture, images) = fixture();
        let result = verify_document(
            &decode_base64(fixture["document"].as_str().unwrap(), 65536).unwrap(),
            &decode_base64(fixture["nonce"].as_str().unwrap(), 32).unwrap(),
            &decode_base64(fixture["binding"].as_str().unwrap(), 32).unwrap(),
            fixture["now"].as_u64().unwrap(),
            &images,
        )
        .unwrap();
        assert_eq!(result.1, images[0].image_id);
    }

    #[test]
    fn rejects_replay_unapproved_images_and_tampering() {
        let (fixture, mut images) = fixture();
        let document = decode_base64(fixture["document"].as_str().unwrap(), 65536).unwrap();
        let nonce = decode_base64(fixture["nonce"].as_str().unwrap(), 32).unwrap();
        let binding = decode_base64(fixture["binding"].as_str().unwrap(), 32).unwrap();
        let now = fixture["now"].as_u64().unwrap();
        assert!(verify_document(&document, &[0; 32], &binding, now, &images).is_err());
        assert!(verify_document(&document, &nonce, &[0; 32], now, &images).is_err());
        assert!(verify_document(&document, &nonce, &binding, now + 400_000, &images).is_err());
        assert!(verify_document(&document, &nonce, &binding, now - 60_000, &images).is_err());
        let mut tampered = document.clone();
        *tampered.last_mut().unwrap() ^= 1;
        assert!(verify_document(&tampered, &nonce, &binding, now, &images).is_err());
        images[0].pcr4 = "ff".repeat(48);
        assert!(verify_document(&document, &nonce, &binding, now, &images).is_err());
    }

    #[test]
    fn rejects_duplicate_keys_and_trailing_data() {
        assert!(decode(&[0xa2, 1, 2, 1, 3]).is_err());
        assert!(decode(&[0xa0, 0]).is_err());
    }
}
