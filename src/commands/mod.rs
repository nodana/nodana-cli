use anyhow::{Result, ensure};
use reqwest::Method;
use serde_json::{Value, json};

use crate::{
    Commands,
    api::{PhoenixdClient, Reply},
    output,
};

type Parameters = Vec<(String, String)>;

pub(crate) struct Request {
    pub method: Method,
    pub parts: Vec<String>,
    pub params: Parameters,
    pub query: bool,
    pub transaction: bool,
    pub invoice: bool,
}

impl Request {
    fn new(method: Method, route: &str, params: Value) -> Result<Self> {
        let query = method == Method::GET || route == "export";
        let params = parameters(params)?;
        let transaction = matches!(route, "sendtoaddress" | "bumpfee" | "closechannel");
        Ok(Self {
            method,
            parts: route.split('/').map(str::to_owned).collect(),
            params,
            query,
            transaction,
            invoice: route == "createinvoice",
        })
    }
}

pub(crate) fn request(command: Commands) -> Result<Request> {
    use Commands::*;
    let empty = json!({});
    let (method, route, params) = match command {
        GetInfo => (Method::GET, "getinfo".into(), empty),
        GetBalance => (Method::GET, "getbalance".into(), empty),
        ListChannels => (Method::GET, "listchannels".into(), empty),
        GetLnAddress => (Method::GET, "getlnaddress".into(), empty),
        GetSwapInAddress => (Method::GET, "getswapinaddress".into(), empty),
        CreateInvoice {
            description,
            description_hash,
            amount_sat,
            expiry_seconds,
            external_id,
            webhook_url,
        } => (
            Method::POST,
            "createinvoice".into(),
            json!({"description":description,"descriptionHash":description_hash,"amountSat":amount_sat,"expirySeconds":expiry_seconds,"externalId":external_id,"webhookUrl":webhook_url}),
        ),
        CreateOffer {
            description,
            amount_sat,
        } => (
            Method::POST,
            "createoffer".into(),
            json!({"description":description,"amountSat":amount_sat}),
        ),
        PayInvoice { invoice, amount } => (
            Method::POST,
            "payinvoice".into(),
            merge(json!({"invoice":invoice}), serde_json::to_value(amount)?),
        ),
        PayOffer {
            offer,
            amount,
            message,
        } => (
            Method::POST,
            "payoffer".into(),
            merge(
                json!({"offer":offer,"message":message}),
                serde_json::to_value(amount)?,
            ),
        ),
        PayLnAddress {
            address,
            amount,
            message,
        } => {
            ensure!(
                amount.amount_sat.is_some() || amount.send_all,
                "pay-ln-address requires --amount-sat or --send-all"
            );
            (
                Method::POST,
                "paylnaddress".into(),
                merge(
                    json!({"address":address,"message":message}),
                    serde_json::to_value(amount)?,
                ),
            )
        }
        SendToAddress {
            amount_sat,
            address,
            feerate_sat_byte,
        } => (
            Method::POST,
            "sendtoaddress".into(),
            json!({"amountSat":amount_sat,"address":address,"feerateSatByte":feerate_sat_byte}),
        ),
        BumpFee { feerate_sat_byte } => (
            Method::POST,
            "bumpfee".into(),
            json!({"feerateSatByte":feerate_sat_byte}),
        ),
        ListIncomingPayments {
            history,
            external_id,
        } => {
            validate_dates(&history.dates)?;
            (
                Method::GET,
                "payments/incoming".into(),
                merge(
                    serde_json::to_value(history)?,
                    json!({"externalId":external_id}),
                ),
            )
        }
        GetIncomingPayment { payment_hash } => (
            Method::GET,
            format!("payments/incoming/{payment_hash}"),
            empty,
        ),
        ListOutgoingPayments { history } => {
            validate_dates(&history.dates)?;
            (
                Method::GET,
                "payments/outgoing".into(),
                serde_json::to_value(history)?,
            )
        }
        GetOutgoingPayment {
            payment_id,
            payment_hash,
        } => {
            let path = if let Some(id) = payment_id {
                format!("payments/outgoing/{id}")
            } else if let Some(hash) = payment_hash {
                format!("payments/outgoingbyhash/{hash}")
            } else {
                anyhow::bail!("Supply --payment-id or --payment-hash");
            };
            (Method::GET, path, empty)
        }
        ExportPayments { dates } => {
            validate_dates(&dates)?;
            (Method::POST, "export".into(), serde_json::to_value(dates)?)
        }
        CloseChannel {
            channel_id,
            address,
            feerate_sat_byte,
        } => (
            Method::POST,
            "closechannel".into(),
            json!({"channelId":channel_id,"address":address,"feerateSatByte":feerate_sat_byte}),
        ),
        DecodeInvoice { invoice } => (
            Method::POST,
            "decodeinvoice".into(),
            json!({"invoice":invoice}),
        ),
        DecodeOffer { offer } => (Method::POST, "decodeoffer".into(), json!({"offer":offer})),
        EstimateLiquidityFees { amount_sat } => (
            Method::GET,
            "estimateliquidityfees".into(),
            json!({"amountSat":amount_sat}),
        ),
        LnurlPay {
            lnurl,
            amount,
            message,
        } => (
            Method::POST,
            "lnurlpay".into(),
            merge(
                json!({"lnurl":lnurl,"message":message}),
                serde_json::to_value(amount)?,
            ),
        ),
        LnurlWithdraw { lnurl } => (Method::POST, "lnurlwithdraw".into(), json!({"lnurl":lnurl})),
        LnurlAuth { lnurl } => (Method::POST, "lnurlauth".into(), json!({"lnurl":lnurl})),
    };
    Request::new(method, &route, params)
}

fn validate_dates(dates: &crate::Dates) -> Result<()> {
    if let (Some(from), Some(to)) = (dates.from, dates.to) {
        ensure!(from <= to, "--from must not be later than --to");
    }
    for date in [dates.from, dates.to].into_iter().flatten() {
        ensure!(
            date <= i64::MAX as u64,
            "Timestamp exceeds Phoenixd's supported range"
        );
    }
    Ok(())
}

fn merge(mut value: Value, other: Value) -> Value {
    if let (Some(target), Some(source)) = (value.as_object_mut(), other.as_object()) {
        target.extend(source.clone());
    }
    value
}

fn parameters(value: Value) -> Result<Parameters> {
    let object = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Invalid command parameters"))?;
    Ok(object
        .iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, v)| {
            (
                k.clone(),
                v.as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| v.to_string()),
            )
        })
        .collect())
}

pub async fn run(client: &PhoenixdClient, command: Commands) -> Result<()> {
    let request = request(command)?;
    let parts: Vec<_> = request.parts.iter().map(String::as_str).collect();
    match client
        .request(request.method, &parts, &request.params, request.query)
        .await?
    {
        Reply::Json(value) => {
            println!("{}", serde_json::to_string_pretty(&value)?);
            ensure!(
                !request.transaction,
                "Phoenixd did not return a transaction ID; check the result before retrying"
            );
            ensure!(
                value.get("type").and_then(Value::as_str) != Some("payment_failed"),
                "Phoenixd reported that the payment failed"
            );
            if request.invoice
                && let Some(invoice) = value.get("serialized").and_then(Value::as_str)
            {
                output::invoice_qr(invoice);
            }
        }
        Reply::Text(text) => {
            println!("{}", output::clean(&text));
            if request.transaction {
                ensure!(
                    crate::hash(text.trim()).is_ok(),
                    "Phoenixd did not return a transaction ID; check the result before retrying"
                );
            }
        }
    }
    Ok(())
}
