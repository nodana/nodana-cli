# Nodana CLI

Use `ndn` to work directly with a Phoenixd node. Supply its URL and Phoenixd password to create and track invoices, send payments, inspect channels and use LNURL services. See the [CLI reference](https://nodana.io/cli-ref) for the complete command list.

## Install

```bash
curl -fsSL https://nodana.io/cli | sh
```

The installer provides macOS and Linux binaries. If `ndn` is not found, add the installation directory printed by the installer to your `PATH` and open a new terminal. Run the installer again to upgrade.

## Connect to your node

Set `NODANA_PHOENIXD_URL` to your node’s complete HTTPS URL, such as `https://conn-1234.nodana.app:9740`. Set `NODANA_PHOENIXD_PASSWORD` to the node password in your terminal or backend environment. The CLI reads both variables automatically. Use `--url` or `--password` to override either value for a single command. Use the limited Phoenixd password for reads and receiving payments; spending and channel closure require the full password.

```bash
ndn get-info
ndn get-balance
```

For a daemon on the same machine, the default Phoenixd URL is `http://127.0.0.1:9740`.

## Receive a payment

```bash
ndn create-invoice --amount-sat 1000 --description "Order 1042" --external-id order-1042

ndn get-incoming-payment --payment-hash "$PAYMENT_HASH"
```

Save the returned `paymentHash` and share `serialized`, the BOLT11 invoice, with the payer. Check `isPaid` and `receivedSat` before fulfilling the order. `externalId` associates the invoice with an order; it does not deduplicate invoice creation.

## Send and track a payment

```bash
ndn pay-invoice --invoice "$BOLT11_INVOICE"

ndn get-outgoing-payment --payment-hash "$PAYMENT_HASH"
```

Use the full Phoenixd password to send funds. Check the payment result and reconcile an interrupted attempt before submitting another payment.

## More commands

- [Payments](https://nodana.io/cli-ref/create-invoice): invoices, offers, addresses, history and exports.
- [Node](https://nodana.io/cli-ref/get-info): information, balance, channels, decoding and liquidity fees.
- [LNURL](https://nodana.io/cli-ref/lnurl-pay): pay, withdraw and authenticate.

Commands return Phoenixd JSON or text responses. Amount options ending in `-sat` use satoshis; history date filters use Unix milliseconds. Run `ndn --help` or `ndn create-invoice --help` for usage.

## Build from source

With the Rust toolchain installed:

```bash
cargo install --locked --path .
```

Windows users can build from source.
