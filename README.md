# Nodana CLI

Use `pxd` to control a Phoenixd node: create invoices and offers, send payments, inspect balances and channels, and use LNURL services. See the [CLI reference](https://nodana.io/cli-ref) for command options and examples.

## Install and upgrade

```bash
curl -fsSL https://nodana.io/cli | sh
```

Run the installer again to upgrade. It supports macOS on Apple silicon or Intel and Linux on x86-64. If `pxd` is not found, add the directory printed by the installer to your `PATH` and open a new terminal.

## Connect to your node

Set `NODANA_PHOENIXD_URL` to your node's complete HTTPS connection URL:

```bash
export NODANA_PHOENIXD_URL="https://node-a1b2.nodana.app"
```

Replace the example URL with your node's URL. For a node hosted by Nodana, copy the URL from its details in the dashboard and use the Phoenixd password you saved when creating the node.

Set `NODANA_PHOENIXD_PASSWORD` in your terminal or secret configuration. Use the limited Phoenixd password for reading node information and creating invoices. Use the full password for spending, channel closure, fee bumps and LNURL authentication. Keep both passwords private.

`--url` and `--password` override the environment for one command. Phoenixd commands require both a URL and password.

```bash
pxd get-info
pxd get-balance
pxd --url https://node-a1b2.nodana.app get-info
```

## Commands

Run `pxd --help` to see the Payments, Node, LNURL and Misc sections. Run `pxd create-invoice --help` for a command's options.

Commands print the Phoenixd response as JSON or text. Amount options ending in `-sat` use satoshis; history date filters use Unix milliseconds. Payment export returns a file path on the machine running Phoenixd.

### Receive a payment

Create an invoice for 1,000 satoshis:

```bash
pxd create-invoice --amount-sat 1000 --description "Order 1042" --external-id order-1042
```

`create-invoice` also shows a QR code for the payment request when run in an interactive terminal. Scan it with a Lightning wallet to pay. Redirected or piped output contains only the JSON response.

Use the payment hash returned with the invoice to check whether it has been paid:

```bash
pxd get-incoming-payment --payment-hash PAYMENT_HASH
```

Replace `PAYMENT_HASH` with the returned hash. Confirm the payment and amount before fulfilling an order.

### Send a payment

Set `NODANA_PHOENIXD_PASSWORD` to the full Phoenixd password, then pay a BOLT11 invoice:

```bash
pxd pay-invoice --invoice BOLT11_INVOICE
```

Replace `BOLT11_INVOICE` with the recipient's invoice. The amount comes from the invoice; for an amountless invoice, add `--amount-sat 1000` with the amount you want to send.

Save the returned payment ID and check the result:

```bash
pxd get-outgoing-payment --payment-id PAYMENT_ID
```

Replace `PAYMENT_ID` with the returned ID. You can also look up a payment with `--payment-hash PAYMENT_HASH`. After an interrupted payment, look it up before trying again.

### Inspect your node

```bash
pxd get-info
pxd get-balance
pxd list-channels
```

These commands work with either the limited or full Phoenixd password.

## Nodana-hosted nodes

To update a node hosted by Nodana, use the [Nodana dashboard](https://app.nodana.io).

### Verify the host

For a node on a protected Nodana host, supply its HTTPS URL and the release checksum you have independently approved. No Phoenixd password is required:

```bash
pxd attest --approve-release-sha256 APPROVED_RELEASE_SHA256
pxd attest --approve-release-sha256 APPROVED_RELEASE_SHA256 --json
```

Replace `APPROVED_RELEASE_SHA256` with the exact bundle's canonical manifest checksum you have independently approved: 64 lowercase hexadecimal characters.

Attestation verifies the host's evidence locally and exits successfully only when the proof, approved host image, running bundle checksum and TLS connection match. If verification fails, check the node URL, approved checksum and CLI version. Contact Nodana if it still fails; keep your password and recovery material private.

## Troubleshooting

- Missing connection settings: set `NODANA_PHOENIXD_URL` and `NODANA_PHOENIXD_PASSWORD`, or pass the connection flags.
- Password rejected: check the configured Phoenixd password.
- Access denied: use the full password if the command requires spending or other full access.
- Connection failure: check the URL and that the node is running.
