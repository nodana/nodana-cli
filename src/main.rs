mod api;
mod commands;
mod output;

#[cfg(test)]
mod wire_tests;

use anyhow::{Context, Result, ensure};
use clap::{Args, Parser, Subcommand};
use serde::Serialize;
use zeroize::Zeroizing;

#[derive(Parser)]
#[command(name = "ndn", version, about = "Use your Phoenixd node directly")]
struct Cli {
    /// Override NODANA_PHOENIXD_URL (HTTPS for remote nodes)
    #[arg(long, global = true)]
    url: Option<String>,
    /// Override NODANA_PHOENIXD_PASSWORD with this node password
    #[arg(long, global = true)]
    password: Option<String>,
    /// Disable colours and invoice QR codes
    #[arg(long, global = true)]
    plain: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct Amount {
    /// Amount in satoshis
    #[arg(long, conflicts_with = "send_all", value_parser = positive)]
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_sat: Option<u64>,
    /// Send the wallet balance (requires a single channel)
    #[arg(long)]
    #[serde(skip_serializing_if = "is_false")]
    send_all: bool,
}

#[derive(Args, Serialize, Default)]
struct Dates {
    /// Start timestamp in Unix milliseconds
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<u64>,
    /// End timestamp in Unix milliseconds
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<u64>,
}

#[derive(Args, Serialize, Default)]
struct History {
    #[command(flatten)]
    #[serde(flatten)]
    dates: Dates,
    /// Number of payments to return (Phoenixd defaults to 20)
    #[arg(long, value_parser = positive)]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
    /// Pagination offset
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
    /// Include unpaid incoming invoices or failed outgoing payments
    #[arg(long)]
    #[serde(skip_serializing_if = "is_false")]
    all: bool,
}

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Subcommand)]
enum Commands {
    /// Read node information and channel summaries
    GetInfo,
    /// Read spendable balance and fee credit
    GetBalance,
    /// Read detailed channel state
    ListChannels,
    /// Create a BOLT11 invoice
    CreateInvoice {
        #[arg(long, required_unless_present = "description_hash", conflicts_with = "description_hash", value_parser = description)]
        description: Option<String>,
        #[arg(long, value_parser = hash)]
        description_hash: Option<String>,
        #[arg(long, value_parser = positive)]
        amount_sat: Option<u64>,
        #[arg(long, value_parser = positive)]
        expiry_seconds: Option<u64>,
        #[arg(long)]
        external_id: Option<String>,
        #[arg(long)]
        webhook_url: Option<String>,
    },
    /// Create a reusable BOLT12 offer
    CreateOffer {
        #[arg(long, value_parser = description)]
        description: Option<String>,
        #[arg(long, requires = "description", value_parser = positive)]
        amount_sat: Option<u64>,
    },
    /// Get the node's Lightning address
    GetLnAddress,
    /// Get an on-chain address for funding the wallet
    GetSwapInAddress,
    /// Pay a BOLT11 invoice using the full password
    PayInvoice {
        #[arg(long)]
        invoice: String,
        #[command(flatten)]
        amount: Amount,
    },
    /// Pay a BOLT12 offer using the full password
    PayOffer {
        #[arg(long)]
        offer: String,
        #[command(flatten)]
        amount: Amount,
        #[arg(long)]
        message: Option<String>,
    },
    /// Pay a Lightning address using the full password
    PayLnAddress {
        #[arg(long)]
        address: String,
        #[command(flatten)]
        amount: Amount,
        #[arg(long)]
        message: Option<String>,
    },
    /// Send funds to a Bitcoin address using the full password
    SendToAddress {
        #[arg(long, value_parser = positive)]
        amount_sat: u64,
        #[arg(long)]
        address: String,
        #[arg(long, value_parser = positive)]
        feerate_sat_byte: u64,
    },
    /// Increase the fee of unconfirmed transactions using the full password
    BumpFee {
        #[arg(long, value_parser = positive)]
        feerate_sat_byte: u64,
    },
    /// List incoming payments
    ListIncomingPayments {
        #[command(flatten)]
        history: History,
        #[arg(long)]
        external_id: Option<String>,
    },
    /// Look up an incoming payment by hash
    GetIncomingPayment {
        #[arg(long, value_parser = hash)]
        payment_hash: String,
    },
    /// List outgoing payments
    ListOutgoingPayments {
        #[command(flatten)]
        history: History,
    },
    /// Look up an outgoing payment by ID or hash
    GetOutgoingPayment {
        #[arg(long, required_unless_present = "payment_hash", conflicts_with = "payment_hash", value_parser = payment_id)]
        payment_id: Option<String>,
        #[arg(long, value_parser = hash)]
        payment_hash: Option<String>,
    },
    /// Export successful payments to a CSV file on the node
    ExportPayments {
        #[command(flatten)]
        dates: Dates,
    },
    /// Permanently close a channel using the full password
    CloseChannel {
        #[arg(long, value_parser = hash)]
        channel_id: String,
        #[arg(long)]
        address: String,
        #[arg(long, value_parser = positive)]
        feerate_sat_byte: u64,
    },
    /// Decode a BOLT11 invoice
    DecodeInvoice {
        #[arg(long)]
        invoice: String,
    },
    /// Decode a BOLT12 offer
    DecodeOffer {
        #[arg(long)]
        offer: String,
    },
    /// Estimate liquidity fees for an amount in satoshis
    EstimateLiquidityFees {
        #[arg(long, value_parser = positive)]
        amount_sat: u64,
    },
    /// Pay an LNURL resource using the full password
    LnurlPay {
        #[arg(long)]
        lnurl: String,
        #[command(flatten)]
        amount: Amount,
        #[arg(long)]
        message: Option<String>,
    },
    /// Withdraw the maximum amount offered by an LNURL service
    LnurlWithdraw {
        #[arg(long)]
        lnurl: String,
    },
    /// Authenticate to an LNURL service using the full password
    LnurlAuth {
        #[arg(long)]
        lnurl: String,
    },
}

fn positive(value: &str) -> std::result::Result<u64, String> {
    value
        .parse::<u64>()
        .ok()
        .filter(|n| *n > 0 && *n <= i64::MAX as u64)
        .ok_or_else(|| "Supply a positive integer no larger than 9223372036854775807".into())
}
fn description(value: &str) -> std::result::Result<String, String> {
    if value.encode_utf16().count() > 128 {
        return Err("Description must contain at most 128 characters".into());
    }
    Ok(value.to_owned())
}
fn hash(value: &str) -> std::result::Result<String, String> {
    if value.len() != 64 || !value.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("Supply a 64-character hexadecimal hash".into());
    }
    Ok(value.to_ascii_lowercase())
}
fn payment_id(value: &str) -> std::result::Result<String, String> {
    if value.len() != 36
        || !value.bytes().enumerate().all(|(i, c)| {
            if [8, 13, 18, 23].contains(&i) {
                c == b'-'
            } else {
                c.is_ascii_hexdigit()
            }
        })
    {
        return Err("Supply a payment UUID".into());
    }
    Ok(value.to_owned())
}

fn main() -> Result<()> {
    // Load dotenv before starting runtime threads. Never include file contents in errors.
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => anyhow::bail!("Could not load .env. Check its syntax and file permissions."),
    }
    main_async()
}

#[tokio::main]
async fn main_async() -> Result<()> {
    let cli = Cli::parse();
    output::configure(cli.plain);
    if let Err(error) = run(cli).await {
        output::error(&format!("{error:#}"));
        std::process::exit(1);
    }
    Ok(())
}

async fn run(cli: Cli) -> Result<()> {
    let password = Zeroizing::new(
        cli.password
            .or_else(|| std::env::var("NODANA_PHOENIXD_PASSWORD").ok())
            .context("Provide your node password using NODANA_PHOENIXD_PASSWORD or --password")?,
    );
    ensure!(
        !password.is_empty(),
        "The Phoenixd password must not be empty"
    );
    let url = cli
        .url
        .or_else(|| std::env::var("NODANA_PHOENIXD_URL").ok())
        .context("Provide your Phoenixd node URL using NODANA_PHOENIXD_URL or --url")?;
    let client = api::PhoenixdClient::new(&url, password)?;
    commands::run(&client, cli.command).await
}
