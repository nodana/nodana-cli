mod api;
mod attestation;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ndn",
    version,
    about = "Work directly with your Phoenixd node",
    disable_help_subcommand = true,
    help_template = "{about-with-newline}\n{usage-heading} {usage}\n\nPayments:\n  create-invoice            Create BOLT11 invoice\n  create-offer              Create BOLT12 offer\n  get-ln-address            Get Lightning address\n  get-swap-in-address       Get swap-in address\n  pay-invoice               Pay BOLT11 invoice\n  pay-offer                 Pay BOLT12 offer\n  pay-ln-address            Pay Lightning address\n  send-to-address           Pay on-chain\n  bump-fee                  Bump fee\n  list-incoming-payments    List incoming payments\n  get-incoming-payment      Get incoming payment\n  list-outgoing-payments    List outgoing payments\n  get-outgoing-payment      Get outgoing payment\n  export-payments           CSV export\n\nNode:\n  get-info                  Get node info\n  get-balance               Get balance\n  list-channels             List channels\n  close-channel             Close channel\n  decode-invoice            Decode invoice\n  decode-offer              Decode offer\n  estimate-liquidity-fees   Estimate liquidity fees\n\nLNURL:\n  lnurl-pay                 LNURL Pay\n  lnurl-withdraw            LNURL Withdraw\n  lnurl-auth                LNURL Auth\n\nMisc:\n  attest                    Verify the protected host and its connection\n\nOptions:\n{options}"
)]
pub struct Cli {
    /// Node connection URL (NODANA_PHOENIXD_URL)
    #[arg(long, global = true)]
    url: Option<String>,
    /// Phoenixd password (NODANA_PHOENIXD_PASSWORD); prefer the environment variable
    #[arg(long, global = true)]
    password: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create BOLT11 invoice
    CreateInvoice {
        /// Invoice description, up to 128 characters. Supply either --description or --description-hash.
        #[arg(
            long,
            required_unless_present = "description_hash",
            conflicts_with = "description_hash"
        )]
        description: Option<String>,
        /// SHA-256 hash of the description, as an alternative to description.
        #[arg(
            long,
            required_unless_present = "description",
            conflicts_with = "description"
        )]
        description_hash: Option<String>,
        /// Amount requested in satoshis. Omit to allow the payer to choose the amount.
        #[arg(long)]
        amount_sat: Option<u64>,
        /// Invoice lifetime in seconds. Defaults to 3600 (one hour).
        #[arg(long)]
        expiry_seconds: Option<u64>,
        /// Your order or payment reference. Use it when looking up incoming invoices.
        #[arg(long)]
        external_id: Option<String>,
        /// Additional callback URL for this invoice, called directly by Phoenixd. Configure [Webhooks](/api-ref/webhooks) in the dashboard for notifications delivered by Nodana.
        #[arg(long)]
        webhook_url: Option<String>,
    },
    /// Create BOLT12 offer
    CreateOffer {
        /// Offer description, up to 128 characters. Required when --amount-sat is provided.
        #[arg(long)]
        description: Option<String>,
        /// Amount requested in satoshis. Omit to allow the payer to choose the amount.
        #[arg(long, requires = "description")]
        amount_sat: Option<u64>,
    },
    /// Get Lightning address
    GetLnAddress,
    /// Get swap-in address
    GetSwapInAddress,
    /// Pay BOLT11 invoice
    PayInvoice {
        /// BOLT11 invoice to pay.
        #[arg(long)]
        invoice: String,
        /// Amount in satoshis. If omitted, uses the amount in the payment request. Supply --amount-sat or --send-all for an amountless request.
        #[arg(long)]
        amount_sat: Option<u64>,
        /// Send the wallet balance when there is a single channel. Mutually exclusive with --amount-sat.
        #[arg(long, conflicts_with = "amount_sat")]
        send_all: bool,
    },
    /// Pay BOLT12 offer
    PayOffer {
        /// BOLT12 offer to pay.
        #[arg(long)]
        offer: String,
        /// Amount in satoshis. If omitted, uses the amount in the payment request. Supply --amount-sat or --send-all for an amountless request.
        #[arg(long)]
        amount_sat: Option<u64>,
        /// Send the wallet balance when there is a single channel. Mutually exclusive with --amount-sat.
        #[arg(long, conflicts_with = "amount_sat")]
        send_all: bool,
        /// Optional message for the recipient; the receiving service may restrict its length.
        #[arg(long)]
        message: Option<String>,
    },
    /// Pay Lightning address
    PayLnAddress {
        /// Lightning address using BIP-353 or LNURL.
        #[arg(long)]
        address: String,
        /// Amount in satoshis. Supply --amount-sat or --send-all; Lightning addresses do not supply a default amount.
        #[arg(long, required_unless_present = "send_all")]
        amount_sat: Option<u64>,
        /// Send the wallet balance when there is a single channel. Mutually exclusive with --amount-sat.
        #[arg(long, conflicts_with = "amount_sat")]
        send_all: bool,
        /// Optional message for the recipient; the receiving service may restrict its length.
        #[arg(long)]
        message: Option<String>,
    },
    /// Pay on-chain
    SendToAddress {
        /// Amount to send in satoshis.
        #[arg(long)]
        amount_sat: u64,
        /// Destination Bitcoin address.
        #[arg(long)]
        address: String,
        /// Fee rate in satoshis per virtual byte.
        #[arg(long)]
        feerate_sat_byte: u64,
    },
    /// Bump fee
    BumpFee {
        /// Fee rate in satoshis per virtual byte.
        #[arg(long)]
        feerate_sat_byte: u64,
    },
    /// List incoming payments
    ListIncomingPayments {
        /// Start time in Unix milliseconds. Defaults to 0.
        #[arg(long)]
        from: Option<u64>,
        /// End time in Unix milliseconds. Defaults to now.
        #[arg(long)]
        to: Option<u64>,
        /// Number of payments to return. Defaults to 20.
        #[arg(long)]
        limit: Option<u64>,
        /// Pagination offset. Defaults to 0.
        #[arg(long)]
        offset: Option<u64>,
        /// Include unpaid invoices.
        #[arg(long)]
        all: bool,
        /// Only return payments with this external ID.
        #[arg(long)]
        external_id: Option<String>,
    },
    /// Get incoming payment
    GetIncomingPayment {
        /// Payment hash as a hexadecimal string.
        #[arg(long)]
        payment_hash: String,
    },
    /// List outgoing payments
    ListOutgoingPayments {
        /// Start time in Unix milliseconds. Defaults to 0.
        #[arg(long)]
        from: Option<u64>,
        /// End time in Unix milliseconds. Defaults to now.
        #[arg(long)]
        to: Option<u64>,
        /// Number of payments to return. Defaults to 20.
        #[arg(long)]
        limit: Option<u64>,
        /// Pagination offset. Defaults to 0.
        #[arg(long)]
        offset: Option<u64>,
        /// Include failed payments.
        #[arg(long)]
        all: bool,
    },
    /// Get outgoing payment
    GetOutgoingPayment {
        /// Internal payment UUID.
        #[arg(
            long,
            required_unless_present = "payment_hash",
            conflicts_with = "payment_hash"
        )]
        payment_id: Option<String>,
        /// Payment hash to look up instead of the internal payment ID.
        #[arg(
            long,
            required_unless_present = "payment_id",
            conflicts_with = "payment_id"
        )]
        payment_hash: Option<String>,
    },
    /// CSV export
    ExportPayments {
        /// Start time in Unix milliseconds. Defaults to 0.
        #[arg(long)]
        from: Option<u64>,
        /// End time in Unix milliseconds. Defaults to now.
        #[arg(long)]
        to: Option<u64>,
    },
    /// Get node info
    GetInfo,
    /// Get balance
    GetBalance,
    /// List channels
    ListChannels,
    /// Close channel
    CloseChannel {
        /// ID of the channel to close.
        #[arg(long)]
        channel_id: String,
        /// Bitcoin address to receive the channel balance.
        #[arg(long)]
        address: String,
        /// Fee rate in satoshis per virtual byte.
        #[arg(long)]
        feerate_sat_byte: u64,
    },
    /// Decode invoice
    DecodeInvoice {
        /// BOLT11 invoice to decode.
        #[arg(long)]
        invoice: String,
    },
    /// Decode offer
    DecodeOffer {
        /// BOLT12 offer to decode.
        #[arg(long)]
        offer: String,
    },
    /// Estimate liquidity fees
    EstimateLiquidityFees {
        /// Liquidity amount in satoshis.
        #[arg(long)]
        amount_sat: u64,
    },
    /// LNURL Pay
    LnurlPay {
        /// LNURL-pay resource.
        #[arg(long)]
        lnurl: String,
        /// Amount to pay in satoshis. If omitted with --send-all unset, uses the minimum accepted by the service.
        #[arg(long)]
        amount_sat: Option<u64>,
        /// Send the wallet balance when there is a single channel. Mutually exclusive with --amount-sat.
        #[arg(long, conflicts_with = "amount_sat")]
        send_all: bool,
        /// Optional message for the recipient; the receiving service may restrict its length.
        #[arg(long)]
        message: Option<String>,
    },
    /// LNURL Withdraw
    LnurlWithdraw {
        /// LNURL-withdraw resource.
        #[arg(long)]
        lnurl: String,
    },
    /// LNURL Auth
    LnurlAuth {
        /// LNURL-auth resource.
        #[arg(long)]
        lnurl: String,
    },
    /// Verify the protected host and its connection
    Attest {
        /// Return the verification result as JSON
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() {
    if let Err(error) = run(Cli::parse()).await {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<()> {
    let url = cli
        .url
        .or_else(|| std::env::var("NODANA_PHOENIXD_URL").ok());
    let url = api::node_url(url.as_deref())?;
    if let Commands::Attest { json } = cli.command {
        return attestation::run(url, json).await;
    }
    let password = cli
        .password
        .or_else(|| std::env::var("NODANA_PHOENIXD_PASSWORD").ok())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("Set NODANA_PHOENIXD_PASSWORD or pass --password"))?;
    let result = api::PhoenixClient::new(url, password)?
        .execute(cli.command.request())
        .await?;
    println!("{result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn reference_commands_produce_the_documented_api_requests() {
        // Captured from www2's CLI usage and linked API endpoint definitions.
        let fixtures: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/commands.json")).unwrap();
        let mut commands = BTreeSet::new();
        for fixture in fixtures.as_array().unwrap() {
            let argv: Vec<_> = fixture["argv"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            commands.insert(argv[1].to_owned());
            let request = Cli::try_parse_from(&argv).unwrap().command.request();
            assert_eq!(
                request.method.as_str(),
                fixture["method"].as_str().unwrap(),
                "{argv:?}"
            );
            assert_eq!(serde_json::json!(request.path), fixture["path"], "{argv:?}");
            let params: BTreeMap<_, _> = request.params.into_iter().collect();
            assert_eq!(serde_json::json!(params), fixture["params"], "{argv:?}");
        }
        commands.insert("attest".into());
        assert_eq!(
            commands,
            Cli::command()
                .get_subcommands()
                .map(|c| c.get_name().to_owned())
                .collect()
        );
    }

    #[test]
    fn validates_required_and_exclusive_options() {
        for args in [
            vec!["create-invoice"],
            vec![
                "create-invoice",
                "--description",
                "x",
                "--description-hash",
                "abc",
            ],
            vec!["create-offer", "--amount-sat", "1"],
            vec!["pay-invoice"],
            vec![
                "pay-invoice",
                "--invoice",
                "lnbc1",
                "--amount-sat",
                "1",
                "--send-all",
            ],
            vec![
                "pay-offer",
                "--offer",
                "lno1",
                "--amount-sat",
                "1",
                "--send-all",
            ],
            vec!["pay-ln-address", "--address", "alice@example.com"],
            vec![
                "lnurl-pay",
                "--lnurl",
                "lnurl1",
                "--amount-sat",
                "1",
                "--send-all",
            ],
            vec!["get-outgoing-payment"],
            vec![
                "get-outgoing-payment",
                "--payment-id",
                "id",
                "--payment-hash",
                "hash",
            ],
            vec!["send-to-address", "--amount-sat", "1", "--address", "bc1"],
            vec!["estimate-liquidity-fees", "--amount-sat", "-1"],
        ] {
            assert!(
                Cli::try_parse_from(std::iter::once("ndn").chain(args.clone())).is_err(),
                "{args:?}"
            );
        }
        for args in [
            vec!["create-invoice", "--description-hash", "abc"],
            vec!["create-offer"],
            vec![
                "pay-ln-address",
                "--address",
                "alice@example.com",
                "--send-all",
            ],
            vec!["lnurl-pay", "--lnurl", "lnurl1"],
            vec!["attest", "--json"],
        ] {
            assert!(Cli::try_parse_from(std::iter::once("ndn").chain(args)).is_ok());
        }
        let request =
            Cli::try_parse_from(["ndn", "get-outgoing-payment", "--payment-hash", "hash"])
                .unwrap()
                .command
                .request();
        assert_eq!(request.path, ["payments", "outgoingbyhash", "hash"]);
    }

    #[test]
    fn groups_help_and_removes_account_commands() {
        Cli::command().debug_assert();
        let help = Cli::command().render_help().to_string();
        for section in ["Payments:", "Node:", "LNURL:", "Misc:"] {
            assert!(help.contains(section));
        }
        for command in ["login", "logout", "status", "node"] {
            assert!(Cli::try_parse_from(["ndn", command]).is_err());
        }
        let cli = Cli::try_parse_from([
            "ndn",
            "--url",
            "https://example.com",
            "--password",
            "secret",
            "get-info",
        ])
        .unwrap();
        assert_eq!(cli.password.as_deref(), Some("secret"));
    }
}
