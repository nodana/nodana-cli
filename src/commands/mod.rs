use crate::Commands;
use reqwest::Method;

pub struct Request {
    pub method: Method,
    pub path: Vec<String>,
    pub params: Vec<(String, String)>,
}

impl Commands {
    pub fn request(self) -> Request {
        match self {
            Self::CreateInvoice {
                description,
                description_hash,
                amount_sat,
                expiry_seconds,
                external_id,
                webhook_url,
            } => {
                let path = vec!["createinvoice".into()];
                let mut params = Vec::new();
                if let Some(value) = description {
                    params.push(("description".into(), value.to_string()));
                }
                if let Some(value) = description_hash {
                    params.push(("descriptionHash".into(), value.to_string()));
                }
                if let Some(value) = amount_sat {
                    params.push(("amountSat".into(), value.to_string()));
                }
                if let Some(value) = expiry_seconds {
                    params.push(("expirySeconds".into(), value.to_string()));
                }
                if let Some(value) = external_id {
                    params.push(("externalId".into(), value.to_string()));
                }
                if let Some(value) = webhook_url {
                    params.push(("webhookUrl".into(), value.to_string()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::CreateOffer {
                description,
                amount_sat,
            } => {
                let path = vec!["createoffer".into()];
                let mut params = Vec::new();
                if let Some(value) = description {
                    params.push(("description".into(), value.to_string()));
                }
                if let Some(value) = amount_sat {
                    params.push(("amountSat".into(), value.to_string()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::GetLnAddress => {
                let path = vec!["getlnaddress".into()];
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::GetSwapInAddress => {
                let path = vec!["getswapinaddress".into()];
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::PayInvoice {
                invoice,
                amount_sat,
                send_all,
            } => {
                let path = vec!["payinvoice".into()];
                let mut params = vec![("invoice".into(), invoice.to_string())];
                if let Some(value) = amount_sat {
                    params.push(("amountSat".into(), value.to_string()));
                }
                if send_all {
                    params.push(("sendAll".into(), "true".into()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::PayOffer {
                offer,
                amount_sat,
                send_all,
                message,
            } => {
                let path = vec!["payoffer".into()];
                let mut params = vec![("offer".into(), offer.to_string())];
                if let Some(value) = amount_sat {
                    params.push(("amountSat".into(), value.to_string()));
                }
                if send_all {
                    params.push(("sendAll".into(), "true".into()));
                }
                if let Some(value) = message {
                    params.push(("message".into(), value.to_string()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::PayLnAddress {
                address,
                amount_sat,
                send_all,
                message,
            } => {
                let path = vec!["paylnaddress".into()];
                let mut params = vec![("address".into(), address.to_string())];
                if let Some(value) = amount_sat {
                    params.push(("amountSat".into(), value.to_string()));
                }
                if send_all {
                    params.push(("sendAll".into(), "true".into()));
                }
                if let Some(value) = message {
                    params.push(("message".into(), value.to_string()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::SendToAddress {
                amount_sat,
                address,
                feerate_sat_byte,
            } => {
                let path = vec!["sendtoaddress".into()];
                let params = vec![
                    ("amountSat".into(), amount_sat.to_string()),
                    ("address".into(), address.to_string()),
                    ("feerateSatByte".into(), feerate_sat_byte.to_string()),
                ];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::BumpFee { feerate_sat_byte } => {
                let path = vec!["bumpfee".into()];
                let params = vec![("feerateSatByte".into(), feerate_sat_byte.to_string())];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::ListIncomingPayments {
                from,
                to,
                limit,
                offset,
                all,
                external_id,
            } => {
                let path = vec!["payments".into(), "incoming".into()];
                let mut params = Vec::new();
                if let Some(value) = from {
                    params.push(("from".into(), value.to_string()));
                }
                if let Some(value) = to {
                    params.push(("to".into(), value.to_string()));
                }
                if let Some(value) = limit {
                    params.push(("limit".into(), value.to_string()));
                }
                if let Some(value) = offset {
                    params.push(("offset".into(), value.to_string()));
                }
                if all {
                    params.push(("all".into(), "true".into()));
                }
                if let Some(value) = external_id {
                    params.push(("externalId".into(), value.to_string()));
                }
                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::GetIncomingPayment { payment_hash } => {
                let path = vec!["payments".into(), "incoming".into(), payment_hash];
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::ListOutgoingPayments {
                from,
                to,
                limit,
                offset,
                all,
            } => {
                let path = vec!["payments".into(), "outgoing".into()];
                let mut params = Vec::new();
                if let Some(value) = from {
                    params.push(("from".into(), value.to_string()));
                }
                if let Some(value) = to {
                    params.push(("to".into(), value.to_string()));
                }
                if let Some(value) = limit {
                    params.push(("limit".into(), value.to_string()));
                }
                if let Some(value) = offset {
                    params.push(("offset".into(), value.to_string()));
                }
                if all {
                    params.push(("all".into(), "true".into()));
                }
                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::GetOutgoingPayment {
                payment_id,
                payment_hash,
            } => {
                let path = match (payment_id, payment_hash) {
                    (Some(id), None) => vec!["payments".into(), "outgoing".into(), id],
                    (None, Some(hash)) => vec!["payments".into(), "outgoingbyhash".into(), hash],
                    _ => unreachable!("clap validates the payment lookup"),
                };
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::ExportPayments { from, to } => {
                let path = vec!["export".into()];
                let mut params = Vec::new();
                if let Some(value) = from {
                    params.push(("from".into(), value.to_string()));
                }
                if let Some(value) = to {
                    params.push(("to".into(), value.to_string()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::GetInfo => {
                let path = vec!["getinfo".into()];
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::GetBalance => {
                let path = vec!["getbalance".into()];
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::ListChannels => {
                let path = vec!["listchannels".into()];
                let params = Vec::new();

                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::CloseChannel {
                channel_id,
                address,
                feerate_sat_byte,
            } => {
                let path = vec!["closechannel".into()];
                let params = vec![
                    ("channelId".into(), channel_id.to_string()),
                    ("address".into(), address.to_string()),
                    ("feerateSatByte".into(), feerate_sat_byte.to_string()),
                ];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::DecodeInvoice { invoice } => {
                let path = vec!["decodeinvoice".into()];
                let params = vec![("invoice".into(), invoice.to_string())];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::DecodeOffer { offer } => {
                let path = vec!["decodeoffer".into()];
                let params = vec![("offer".into(), offer.to_string())];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::EstimateLiquidityFees { amount_sat } => {
                let path = vec!["estimateliquidityfees".into()];
                let params = vec![("amountSat".into(), amount_sat.to_string())];
                Request {
                    method: Method::GET,
                    path,
                    params,
                }
            }
            Self::LnurlPay {
                lnurl,
                amount_sat,
                send_all,
                message,
            } => {
                let path = vec!["lnurlpay".into()];
                let mut params = vec![("lnurl".into(), lnurl.to_string())];
                if let Some(value) = amount_sat {
                    params.push(("amountSat".into(), value.to_string()));
                }
                if send_all {
                    params.push(("sendAll".into(), "true".into()));
                }
                if let Some(value) = message {
                    params.push(("message".into(), value.to_string()));
                }
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::LnurlWithdraw { lnurl } => {
                let path = vec!["lnurlwithdraw".into()];
                let params = vec![("lnurl".into(), lnurl.to_string())];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
            Self::LnurlAuth { lnurl } => {
                let path = vec!["lnurlauth".into()];
                let params = vec![("lnurl".into(), lnurl.to_string())];
                Request {
                    method: Method::POST,
                    path,
                    params,
                }
            }
        }
    }
}
