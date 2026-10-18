# Local development

The CLI sends requests directly to Phoenixd. Use an unfunded local test node for manual development. Set `NODANA_PHOENIXD_URL` to its URL and `NODANA_PHOENIXD_PASSWORD` to its password:

```bash
cargo run -- get-info
cargo run -- get-balance
```

For local configuration, copy `.env.example` to `.env` and supply the node URL and password there. Dotenv loads from the working directory or its nearest parent containing a `.env` file. Exported environment variables take precedence over the file; `--url` and `--password` override their respective environment variables. A malformed or unreadable `.env` stops the command.

The client uses HTTP Basic authentication with an empty username. HTTPS is required outside loopback addresses. Requests do not follow redirects, and mutations are not automatically replayed. The CLI does not persist node passwords or use the Nodana account login service.

```bash
cargo fmt --check
cargo check --offline
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
```

Tests use local mock servers with dummy passwords. They cover native routes and form/query encoding, CLI constraints, environment precedence, redacted authentication errors, redirects, interrupted payments, and failure bodies returned with HTTP 200. They do not move funds or require a live node.
