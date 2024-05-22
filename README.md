# nodana-cli

This is the CLI tool from [Nodana](https://nodana.io). With this package you can run an instace of [phoenixd](https://phoenix.acinq.co/server) on Nodana infrastructure. No personal details or credit cards required.

To get started, install the package globally;

```sh
npm install nodana-cli -g
```

Check that the installation was successful by running the following command in a terminal window:

```sh
nodana --help
```

This should display the commands available to you.

## API Key

You need an API key to use the CLI. As we are currently beta testing, you will need a token which you can exchange for an API key using the `exchange` command.

> Tokens are available over Nostr. Check the Nodana website for the link.

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
