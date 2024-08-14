# Nodana CLI

This is the official CLI tool for [Nodana](https://nodana.io). With this package you can spin up Bitcoin and Lightning infrastructure in the cloud without needing to worry about setting up and maintaining your own servers.

To get started, install the package globally:

```sh
npm install -g @nodana/nodana-cli
```

Check that the installation was successful by running the following command in a terminal window:

```sh
nodana -v
```

This should display the installed version.

## Help

To view the available commands in the terminal, you can run:

```sh
nodana --help
```

## Commands

### Init

Get an API key.

```
nodana init
  -y (auto accept Nodana's terms and conditions)
```

After calling `nodana init`, the CLI will save your API key in a local file. The key will automatically be included in future requests.

### Create Invoice

Create a Lightning invoice to top up the credit on your API key. This will allow you to deploy services in the cloud. Each sat equals 2.5 minutes of run time. The minimum amount for an invoice is 1000 sats.

```
nodana create invoice
  -s <sats> (Required, amount in sats, min: 1k, max: 1m)
```

### Create A Service

Nodana currently supports the following services:

- Phoenixd (phoenixd)
- Alby Hub (alby-hub)
- Nostr Relay (nostr-relay) (coming soon)

You can create services with or without a service file. A service file lets you define extra settings for your service but if the default service is ok for your needs then you don't need to reference one.

Create a service (without a service file):

```
nodana service create <service-name>
  -y <auto confirm> (optional)
```

Create a service (with a service file)

```
nodana service create -f </path/to/service/file>
  -y <auto confirm> (optional)
```

Here's an example of a service file:

```toml
service = "<service-name>"

[settings]
name = "my-service"
webhook = "https://example.com/webhook" # (phoenixd only)
auto_liquidity = "5m"                   # (phoenixd only)
```

> Save the service file with a `.toml` extension

### Start Service

Start a service that has been stopped.

```
nodana service start <serviceId>
```

### Stop Service

Stop a service.

```
nodana service stop <serviceId>
```

### List Services

List services.

```
nodana service list
```

### Delete Service

Delete a service.

```
nodana service delete <serviceId>
  -y <auto confirm> (optional)
```

### Status

Check the credit balance for your API key.

```
nodana status
```

## Contributing

If you find any issues with this tool or would like to add more features then please fork the repo, make your changes and create a PR request. Changes will be reviewed as soon as possible.

## Support

Nostr or Twitter are great ways to get in touch. Just send a DM with your question or issue.
