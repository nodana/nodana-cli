# Nodana CLI

This is the official CLI tool for [Nodana](https://nodana.io). With this package you can create [phoenixd](https://phoenix.acinq.co/server) Lightning nodes hosted on Nodana infrastructure. No personal details or credit cards required. Sats / min pricing.

To get started, install the package globally;

```sh
npm install @nodana/nodana-cli -g
```

Check that the installation was successful by running the following command in a terminal window:

```sh
nodana --help
```

This should display the commands available to you.

## Commands

### Init

Get an API key.

```
nodana init

  -y (auto accept Nodana's terms and conditions)
```

After calling `nodana init`, the CLI will save your API key in a local file. The key will automatically be included in future requests.

### Create Invoice

Create a Lightning invoice to top up the credit on your API key.

```
nodana create invoice
  -v <value> (Amount in sats, min: 1k, max: 1m)
```

### Create Node

Create and start a node.

```
nodana create node
  -n <name>
  -a <autoLiquidity>
  -w <webhook>
```

### Start

Start a node that has been stopped.

```
nodana start <nodeId>
```

### Stop

Stop a node. If you would like to delete a node then you must stop it first.

```
nodana stop <nodeId>
```

### List

List nodes.

```
nodana list
```

### Delete

Delete a node.

```
nodana delete <nodeId>
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
