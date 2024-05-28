# nodana-cli

This is the official CLI tool for [Nodana](https://nodana.io). With this package you can create instances of [phoenixd](https://phoenix.acinq.co/server) on Nodana infrastructure. No personal details or credit cards required. Sats / min pricing.

To get started, install the package globally;

```sh
npm install @nodana-io/nodana-cli -g
```

Check that the installation was successful by running the following command in a terminal window:

```sh
nodana --help
```

This should display the commands available to you.

## Commands

### Init

Get an API key. Keys are currently limited due to beta testing.

```
nodana init
```

Options

```
-y (auto accept Nodana's terms and conditions)
```

After calling `nodana init`, the CLI will save your API key in a file called `.nodana.conf`. The key will automatically be included in all requests listed below. You can override this by providing the API key manually using the `-k <key>` option.

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

### Status (coming soon)

Check how many sats you have remaining.

```
nodana status
```
