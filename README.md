# Nodana CLI

`ndn` manages Phoenixd nodes on Nodana. The [CLI reference](https://nodana.io/docs/cli) describes every command.

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

The CLI and browser receive connection status through Server-Sent Events (SSE),
without periodically polling the API. Reconnecting streams receive the current
request status. After approval, the CLI collects its token once in a separate
request; tokens and request secrets are never sent in event payloads or URLs.

CLI session tokens grant node read/write access and expire after 30 days.
They are managed automatically, stored in the OS credential store, and are
not displayed or made available through CLI commands or the Developers UI.
`ndn status` checks your connection; `ndn logout` revokes this session and
removes its local credential. If revocation cannot reach the API, logout keeps
the credential so you can retry. Signing in again replaces the local session
and attempts to revoke the previous one.

On macOS, storage uses Keychain; Linux requires an unlocked Secret Service
(such as GNOME Keyring or KWallet), and Windows uses Credential Manager.
The published installer provides macOS and Linux binaries; Windows support
requires building from source.
The CLI does not read `NODANA_API_KEY` and offers no token argument or file
fallback. User-created API keys remain available for direct API integrations.

If the browser does not open automatically, open the link printed in your
terminal. Login requests expire after five minutes; keep the terminal open
while approving access. A failed or declined login keeps an existing local
credential. Run `ndn login` again to retry.

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

`create` prints the node's passwords and recovery seed once. Save them securely. `delete` asks you to type the node ID unless `--force` is set.
Node creation stores the restricted password for dashboard access by default. Pass `--full` to store the full password and enable dashboard payments.

Run `ndn node --help` for command help.

Interactive terminals use headings, tables, status colours, and transient
spinners. Login clears its screen before showing the final connection summary.
Node commands retain their results and one-time credentials in the terminal.
Use `--plain` to disable colours, animation, and screen clearing. Piped output
has no screen control sequences or animation; node lists use tab-separated rows.
The CLI does not provide a JSON output mode.
Expired local sessions start browser authorization again in an interactive
terminal; plain or piped commands ask you to run `ndn login`. Invalid or revoked
server credentials also ask you to reconnect, without replaying node mutations.
