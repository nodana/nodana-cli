# CLI development

The root `nd attest` command verifies a protected host without account or Phoenixd credentials. The `nd` executable dispatches account commands to `src/management.rs` and Phoenixd commands to the existing Phoenixd modules. Invoking the same executable through a link named `pxd` selects Phoenixd directly. Releases contain one executable per supported platform; the installer creates the alias.

Run `cargo fmt --check`, `cargo clippy --locked -- -D warnings` and `cargo test --locked`. The HTTP regression tests bind loopback listeners and never create hosted resources. Account requests can target a local stand-in through `--api-url` or `NODANA_API_ORIGIN`; the default is `https://api.nodana.io`. Overrides must be HTTPS origins, or HTTP on localhost, without paths or credentials.

Node creation generates a fresh random 128-bit request ID and sends one request. The current API returns credentials once and rejects repeated accepted request IDs. Do not add automatic creation retries or derive Phoenixd credentials from an account API key. Preserve credentials in successful JSON output and never include API error response bodies or bearer tokens in diagnostic messages.
