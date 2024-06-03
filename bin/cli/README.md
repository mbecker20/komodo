# Monitor CLI

Monitor CLI is a tool to sync monitor resources and execute operations.

## Install

```sh
cargo install monitor_cli
```

## Usage

### Credentials

Configure a file `~/.config/monitor/creds.toml` file with contents:
```toml
url = "https://your.monitor.address"
key = "YOUR-API-KEY"
secret = "YOUR-API-SECRET"
```

Note. You can specify a different creds file by using `--creds ./other/path.toml`.
You can also bypass using any file and pass the information using `--url`, `--key`, `--secret`:

```sh
monitor --url "https://your.monitor.address" --key "YOUR-API-KEY" --secret "YOUR-API-SECRET" ...
```

### Run Syncs

```sh
## Sync resources in a single file
monitor sync ./resources/deployments.toml

## Sync resources gathered across multiple files in a directory
monitor sync ./resources

## Path defaults to './resources', in this case you can just use:
monitor sync
```

#### Manual
```md
Runs syncs on resource files

Usage: monitor sync [OPTIONS] [PATH]

Arguments:
  [PATH]  The path of the resource folder / file Folder paths will recursively incorporate all the resources it finds under the folder [default: ./resources]

Options:
      --delete  Will delete any resources that aren't included in the resource files
  -h, --help    Print help
```

### Run Executions

```sh
# Triggers an example build
monitor execute run-build test_build
```

#### Manual
```md
Runs an execution

Usage: monitor execute <COMMAND>

Commands:
  none                 The "null" execution. Does nothing
  run-procedure        Runs the target procedure. Response: [Update]
  run-build            Runs the target build. Response: [Update]
  deploy               Deploys the container for the target deployment. Response: [Update]
  start-container      Starts the container for the target deployment. Response: [Update]
  stop-container       Stops the container for the target deployment. Response: [Update]
  stop-all-containers  Stops all deployments on the target server. Response: [Update]
  remove-container     Stops and removes the container for the target deployment. Reponse: [Update]
  clone-repo           Clones the target repo. Response: [Update]
  pull-repo            Pulls the target repo. Response: [Update]
  prune-networks       Prunes the docker networks on the target server. Response: [Update]
  prune-images         Prunes the docker images on the target server. Response: [Update]
  prune-containers     Prunes the docker containers on the target server. Response: [Update]
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

