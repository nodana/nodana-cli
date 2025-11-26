# Nodana CLI

`nod` manages Phoenixd nodes on Nodana. The [CLI reference](https://nodana.io/docs/cli) describes every command.

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

Create an API key in the Nodana dashboard under **Developers → API Keys**, with the scopes needed for your commands. Set it in your environment:

```bash
export NODANA_API_KEY="<your-api-key>"
```

The CLI sends this key in the `Authorization: Bearer` header. Keep it private. For local development, `NODANA_API_ORIGIN` can override the default `https://api.nodana.io` origin.

## Commands

```text
nod node list
nod node create [--name NAME] [--auto-liquidity 2m|5m|10m] [--full]
nod node get NODE_ID
nod node start NODE_ID
nod node stop NODE_ID
nod node restart NODE_ID
nod node update NODE_ID
nod node delete NODE_ID [--force]
```

`create` prints the node's passwords and recovery seed once. Save them securely. `delete` asks you to type the node ID unless `--force` is set.
Node creation stores the restricted password for dashboard access by default. Pass `--full` to store the full password and enable dashboard payments.

Run `nod node --help` for command help.

## Releases

GitHub Actions publishes a release when a tag starting with `v` is pushed.
Before tagging, update the version in `Cargo.toml` and `Cargo.lock`, ensure the
checks pass, and push the release commit to `main`.

For example, to release version `0.2.0`:

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

The release workflow runs tests and builds `nod-linux-x86_64`,
`nod-darwin-x86_64`, and `nod-darwin-arm64`. Once all builds succeed, it creates
a GitHub Release with those binaries attached. The installer at
`https://nodana.io/cli` downloads binaries from the latest release in
`nodana/nodana-cli`, so keep these asset names and repository name unchanged.
