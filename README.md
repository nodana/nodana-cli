# nodana-cli

This is the official CLI tool for [Nodana](https://nodana.io). With this package you can create instances of [phoenixd](https://phoenix.acinq.co/server) on Nodana infrastructure. No personal details or credit cards required. Sats / min pricing.

To get started, install the package globally;

```sh
npm install @nodana/nodana-cli -g
```

Check that the installation was successful by running the following command in a terminal window:

```sh
nodana --help
```

This should display the commands available to you.

## API Key

You need an API key to use the CLI. As we are currently beta testing, you will need a token which you can exchange for an API key using the `exchange` command. Tokens are available via Nostr. Send us a DM requesting a token and if there's a place left in our beta testing program then we will send you one.

## Commands

### Exchange

Exchange a token for an API key.

```
nodana exchange
```

Options

```
-t <token>
```

### Authorisation

Once you have successfully exchanged a token for an API key, the CLI will create a `.nodana.conf` file in the root of the package which will contain your API key. The key will automatically be included in all requests listed below. You can provide the API key manually by passing `-k <key>` if you prefer.

### Create

Create and start a container.

```
nodana create
```

Options

```
-p <password>
-s <seed>
-a <autoLiquidity>
-w <webhook>
-x <webhookSecret>
```

### Start

Start a container that has been stopped.

```
nodana start <containerId>
```

### Stop

Stop a container. If you would like to delete a container then you must stop it first.

```
nodana stop <containerId>
```

### List

List containers.

```
nodana list
```

### Delete

Delete a container.

```
nodana delete <containerId>
```
