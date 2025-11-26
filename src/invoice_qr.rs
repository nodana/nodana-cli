use anyhow::Result;
use qrcode::{QrCode, render::unicode::Dense1x2};

/// Render the BOLT11 payment request returned by Phoenixd, with its quiet zone.
pub fn render(response: &str) -> Result<Option<String>> {
    let response: serde_json::Value = serde_json::from_str(response)?;
    let Some(invoice) = response.get("serialized").and_then(|value| value.as_str()) else {
        return Ok(None);
    };
    if invoice.is_empty() {
        return Ok(None);
    }
    // BOLT11 is case insensitive. Uppercase enables compact alphanumeric QR
    // encoding, which makes long payment requests easier to scan in a terminal.
    let code = QrCode::new(invoice.to_ascii_uppercase())?;
    let image = code.render::<Dense1x2>().quiet_zone(true).build();
    // Fix both colours so the code scans on light and dark terminal themes.
    Ok(Some(
        image
            .lines()
            .map(|line| format!("\x1b[30;47m{line}\x1b[0m"))
            .collect::<Vec<_>>()
            .join("\n"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_payment_request_with_a_white_quiet_zone() {
        let qr = render(r#"{"paymentHash":"different-value","serialized":"lnbc1example"}"#)
            .unwrap()
            .unwrap();
        let rows: Vec<_> = qr.lines().collect();
        assert!(rows.len() > 10);
        assert!(
            rows.iter()
                .all(|row| row.starts_with("\x1b[30;47m") && row.ends_with("\x1b[0m"))
        );
        assert!(
            rows[0]
                .trim_start_matches("\x1b[30;47m")
                .trim_end_matches("\x1b[0m")
                .chars()
                .all(|c| c == ' ')
        );
        assert_eq!(
            qr,
            render(r#"{"serialized":"LNBC1EXAMPLE"}"#).unwrap().unwrap()
        );
        assert_ne!(
            qr,
            render(r#"{"serialized":"lnbc1another"}"#).unwrap().unwrap()
        );
    }

    #[test]
    fn skips_missing_payment_requests_and_handles_oversized_invoices() {
        for response in ["{}", r#"{"serialized":null}"#, r#"{"serialized":""}"#] {
            assert!(render(response).unwrap().is_none());
        }
        let response = serde_json::json!({"serialized": "lnbc".repeat(3000)}).to_string();
        assert!(render(&response).is_err());
    }
}
