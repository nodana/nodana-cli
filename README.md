# Nodana CLI

Use `ndn` to create invoices and offers, send payments, inspect your Phoenixd node, use LNURL services, and verify a protected host. See the [CLI reference](https://nodana.io/cli-ref) for every command and option.

## Install and upgrade

```bash
curl -fsSL https://nodana.io/cli | sh
```

Run the installer again to upgrade. It supports macOS on Apple silicon or Intel and Linux on x86-64. If `ndn` is not found, add the directory printed by the installer to your `PATH` and open a new terminal.

With the Rust toolchain installed, build from source with `cargo install --locked --path .`. Windows users can build from source.

## Connect to your node

Set `NODANA_PHOENIXD_URL` to your node's complete HTTPS connection URL:

```bash
export NODANA_PHOENIXD_URL="https://node-a1b2.nodana.app"
```

Replace the example URL with your node's URL. For a local daemon, use `http://127.0.0.1:9740`.

Set `NODANA_PHOENIXD_PASSWORD` in your terminal or secret configuration. Use the limited Phoenixd password for reading node information and creating invoices. Use the full password for spending, channel closure, fee bumps and LNURL authentication. Keep both passwords private.

`--url` and `--password` override the environment for one command. A missing URL or password stops the command before any request is made. Attestation needs only the HTTPS URL.

```bash
ndn get-info
ndn get-balance
ndn create-invoice --amount-sat 1000 --description "Order 1042" --external-id order-1042
ndn --url https://node-a1b2.nodana.app get-info
```

## Commands

Run `ndn --help` to see the Payments, Node, LNURL and Misc sections. Run `ndn create-invoice --help` for a command's options.

Commands print the Phoenixd response as JSON or text. Amount options ending in `-sat` use satoshis; history date filters use Unix milliseconds. Payment export returns a file path on the machine running Phoenixd.

Check payment results before reporting success. Confirm incoming payments and amounts before fulfilling an order. After an interrupted outgoing payment, use `get-outgoing-payment` with its payment ID or hash before trying again.

Verify a protected host with:

```bash
ndn attest
ndn attest --json
```

Attestation verifies the host's evidence locally and exits successfully only when the proof, approved image and TLS connection match. If verification fails, check the node URL and update the CLI. Contact Nodana if it still fails; keep your password and recovery material private.

## Troubleshooting

- Missing connection settings: set `NODANA_PHOENIXD_URL` and `NODANA_PHOENIXD_PASSWORD`, or pass the connection flags.
- Password rejected: check the configured Phoenixd password.
- Access denied: use the full password if the command requires spending or other full access.
- Connection failure: check the URL and that the node is running.
