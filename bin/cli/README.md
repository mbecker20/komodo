# Monitor CLI

Monitor CLI is a tool to sync monitor resources and execute file defined procedures.

## Usage

Configure a file `~/.config/monitor/creds.toml` file with contents:
```toml
url = "https://your.monitor.address"
key = "YOUR-API-KEY"
secret = "YOUR-API-SECRET"
```

Note. You can specify a different creds file by using `--creds ./other/path.toml`.

With your creds in place, you can run syncs:

```sh
## Sync resources in a single file
monitor sync ./resources/deployments.toml

## Sync resources gathered across multiple files in a directory
monitor sync ./resources

## Path defaults to './resources', in this case you can just use:
monitor sync
```

And executions:

```sh
## Execute a TOML defined procedure
monitor exec ./execution/execution.toml
```