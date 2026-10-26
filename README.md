# Nodana CLI

Use `nd` to create and manage hosted nodes and webhook receivers. Use `pxd` to control a Phoenixd node: create invoices and offers, send payments, inspect balances and channels, and use LNURL services. See the [CLI reference](https://nodana.io/cli-ref) for command options and examples.

## Install and upgrade

```bash
curl -fsSL https://nodana.io/cli | sh
```

Run the installer again to upgrade. It supports macOS on Apple silicon or Intel and Linux on x86-64. If `pxd` is not found, add the directory printed by the installer to your `PATH` and open a new terminal.

The installer provides `nd` and its `pxd` alias. `pxd` is shorthand for `nd pxd`: `pxd get-info` and `nd pxd get-info` run the same command. Phoenixd examples below use `pxd`. Run `nd --help` for the main command's help.

## Manage hosted nodes and webhooks

Create an API key in the Nodana dashboard under **Developers → API Keys** and set `NODANA_API_KEY` in your terminal or secret configuration. Enable **Nodes → Read** to inspect nodes and webhooks, and **Nodes → Write** to make changes. The key selects the account to manage. Keep it private and replace it if it is revoked or exposed.

```bash
export NODANA_API_KEY="YOUR_API_KEY"
nd create-node --name "Payments" --auto-liquidity 2m
nd list-nodes
nd get-node NODE_ID
nd create-webhook NODE_ID --url https://payments.example.com/events
nd list-webhooks NODE_ID
```

Replace `NODE_ID` with your node's ID. Creation prints the node metadata and one-time credentials: the recovery seed, full Phoenixd password and restricted password. Save them securely immediately and keep the output out of shared logs and chats. The node continues provisioning; use `nd get-node NODE_ID` until its status is ready. After an interrupted creation request, check `nd list-nodes` before trying again.

Webhook creation prints its one-time `signingSecret`; save it in your receiver's secret settings. `nd list-webhooks NODE_ID` returns an array containing zero or one webhook.

Use `nd rename-node`, `nd start-node`, `nd stop-node` and `nd restart-node` to manage your node. `nd delete-node NODE_ID` permanently deletes it and asks you to type its ID; `--force` skips the prompt for automation. Handle remaining funds and save recovery information before deleting a node. Run `nd --help` or `nd create-node --help` for command options.

Commands print JSON to standard output. Status messages go to standard error. Authentication errors mean the API key needs checking; permission errors mean its account or scopes need checking. Add account credit in the dashboard if node creation is rejected for insufficient credit.

## Connect to your Phoenixd node

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

Run `pxd --help` to see the Payments, Node and LNURL sections. Run `pxd create-invoice --help` for a command's options.

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

Use `nd check-node-update NODE_ID` to inspect the available release, then apply its checksum:

```bash
nd update-node NODE_ID --release-sha256 APPROVED_RELEASE_SHA256
```

You can also update a hosted node in the [Nodana dashboard](https://app.nodana.io).

### Verify a host, node bundle, or both

Use `nd attest` with your node's HTTPS URL (or `NODANA_PHOENIXD_URL`) and at least one independently approved checksum. No Phoenixd password is required.

For the bundle, select the exact `bundle-v...` version in [nodana-phoenixd releases](https://github.com/nodana/nodana-phoenixd/releases), review the notes and download `release-sha256.txt`. Use its checksum for `--approve-release-sha256`; `SHA256SUMS` identifies different downloadable files.

For the host, open the [host approval release](https://github.com/nodana/nodana-host/releases/tag/host-policy-v1). Review the desired `host-release-<AMI_ID>.json` manifest and obtain its checksum from `host-release-<AMI_ID>-sha256.txt`. It pins that host's boot measurements and provenance, independently of later additions to Nodana's host approval list.

```bash
# Bundle only
nd attest --approve-release-sha256 APPROVED_BUNDLE_SHA256

# Host only
nd attest --approve-host-sha256 APPROVED_HOST_SHA256

# Both
nd attest --approve-host-sha256 APPROVED_HOST_SHA256 \
  --approve-release-sha256 APPROVED_BUNDLE_SHA256 --json
```

Every mode verifies the AWS proof, freshness, node/TLS binding and an approved host image. Bundle-only mode trusts Nodana's signed host approvals. Host-only mode pins the host but does not independently approve its reported bundle release. Supplying both checksums pins both releases. JSON results include `hostPinned`, `bundlePinned` and `hostReleaseSha256`.

The CLI downloads the signed host approvals automatically from public GitHub release assets. If verification fails, check the URL and intended checksums, then contact Nodana if needed. Do not send passwords or recovery material.

## Troubleshooting

- Missing connection settings: set `NODANA_PHOENIXD_URL` and `NODANA_PHOENIXD_PASSWORD`, or pass the connection flags.
- Password rejected: check the configured Phoenixd password.
- Access denied: use the full password if the command requires spending or other full access.
- Connection failure: check the URL and that the node is running.
