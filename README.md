# Nodana CLI

`nod` is a CLI tool for managing projects and apps on
[Nodana](https://nodana.io).

## Install

```bash
curl -fsSL https://nodana.io/cli | sh
```

## Build From Source

Prerequisites:

- Rust toolchain
- Cargo

Build the binary:

```bash
cargo build --release
```

Run it locally during development:

```bash
cargo run -- project list
```

## Authentication

Create an API key from the tokens section on the Nodana website, then set it as an environment variable:

```bash
export NODANA_API_KEY="<your-key>"
```

If you want the env var to be persistent, add it to your shell profile.

```bash
echo "export NODANA_API_KEY=<your-key>" >> ~/.zshrc
```

`nod` reads `NODANA_API_KEY` for API commands. If it is missing, the CLI exits with an error.

## Command Shape

The CLI uses:

- positional arguments for primary identifiers like app IDs and project IDs
- named flags for configurable values like `--name` and `--auto-liquidity`

Examples:

```bash
nod project get prj-123
nod project create --name my-project
nod project update prj-123 --name new-name
nod app get app-123
nod app create phoenixd --project-id prj-123 --auto-liquidity 2m
```

## Usage

### Projects

```bash
nod project list
nod project get <id>
nod project create --name <name>
nod project update <id> --name <new-name>
nod project delete <id>
nod project apps <id>
```

### Apps

```bash
nod app create <template> [--project-id=<id>] [options]
nod app get <id>
nod app update <id>
nod app start <id>
nod app stop <id>
nod app restart <id>
nod app delete <id>
```

#### Templates

- `phoenixd`
- `alby-hub-phoenixd`
- `lnbits-phoenixd`
- `cdk-mintd`
- `nutshell`
- `boltzd`
- `fedimintd`
- `fedimintd-pro`
- `gatewayd`
- `lnbits`
- `vss`
- `alby-hub`

#### Examples

```bash
nod app create phoenixd --auto-liquidity 2m
nod app create phoenixd --auto-liquidity 5m --webhook https://example.com/hook
nod app create lnbits-phoenixd --auto-liquidity 10m --project-id prj-5fd3
nod app create cdk-mintd --mint-name "My Mint" --mint-description "A cashu mint"
nod app create nutshell --mint-name "My Mint" --mint-lnd-rest-url https://lnd:8080 --mint-lnd-macaroon <macaroon>
```

## Help

Use Clap help output for the full command reference:

```bash
nod --help
nod project --help
nod app --help
nod app create --help
```
