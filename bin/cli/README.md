# monitor CLI

Monitor CLI is a command line tool to sync monitor resources and execute file defined procedures.

## Examples

```sh
## Sync resources in a single file
monitor sync ./resources/deployments.toml

## Sync resources gathered across multiple files in a directory
monitor sync ./resources

## Path defaults to './resources', in this case you can just use:
monitor sync
```

```sh
## Execute a TOML defined procedure
monitor exec ./execution/execution.toml
```