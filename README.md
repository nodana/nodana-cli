# Nodana CLI

`ndn` manages Phoenixd nodes on Nodana. The [CLI reference](https://nodana.io/cli-ref) describes every command.

## Install

```bash
curl -fsSL https://nodana.io/cli | sh
```

## Build from source

For contributors with the Rust toolchain installed:

```bash
cargo install --locked --path .
```

## Authenticate

Connect through your browser:

```bash
ndn login
ndn status
```

The CLI opens the app's sign-in page. Confirm the matching terminal code and
authorize your chosen account. Existing browser sessions are reused, and MFA
is required when enabled. The app stays on a connection confirmation page.

CLI access lets you read and manage nodes in the selected account for 30 days.
Run `ndn login` again when you need to reconnect. Use `ndn status` to check
your connection.

The published installer provides macOS and Linux binaries; Windows support
requires building from source.

If the browser does not open automatically, open the link printed in your
terminal. Login requests expire after five minutes; keep the terminal open
while approving access. Run `ndn login` again to retry.

To disconnect:

```bash
ndn logout
```

## Commands

```text
ndn login
ndn status
ndn logout
ndn node list
ndn node create [--name NAME] [--auto-liquidity 2m|5m|10m] [--full]
ndn node get NODE_ID
ndn node start NODE_ID
ndn node stop NODE_ID
ndn node restart NODE_ID
ndn node update NODE_ID
ndn node delete NODE_ID [--force]
```

`create` prints the node's passwords and recovery seed once. Save them securely. `delete` asks you to type the node ID unless `--force` is set, then returns once deletion is accepted. Use `ndn node list` to check when the node has been removed.
Node creation stores the restricted password for dashboard access by default. Pass `--full` to store the full password and enable dashboard payments.

Run `ndn node --help` for command help.

Use `--plain` to disable colours, animation, and screen clearing.
