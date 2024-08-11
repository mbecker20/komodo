# Monitor CLI

Monitor CLI is a tool to sync monitor resources and execute operations.

## Install

```sh
cargo install monitor_cli
```

Note: On Ubuntu, also requires `apt install build-essential pkg-config libssl-dev`.

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
  cancel-build         Cancels the target build. Only does anything if the build is `building` when called. Response: [Update]
  deploy               Deploys the container for the target deployment. Response: [Update]
  start-container      Starts the container for the target deployment. Response: [Update]
  restart-container    Restarts the container for the target deployment. Response: [Update]
  pause-container      Pauses the container for the target deployment. Response: [Update]
  unpause-container    Unpauses the container for the target deployment. Response: [Update]
  stop-container       Stops the container for the target deployment. Response: [Update]
  remove-container     Stops and removes the container for the target deployment. Reponse: [Update]
  clone-repo           Clones the target repo. Response: [Update]
  pull-repo            Pulls the target repo. Response: [Update]
  build-repo           Builds the target repo, using the attached builder. Response: [Update]
  cancel-repo-build    Cancels the target repo build. Only does anything if the repo build is `building` when called. Response: [Update]
  stop-all-containers  Stops all containers on the target server. Response: [Update]
  prune-networks       Prunes the docker networks on the target server. Response: [Update]
  prune-images         Prunes the docker images on the target server. Response: [Update]
  prune-containers     Prunes the docker containers on the target server. Response: [Update]
  run-sync             Runs the target resource sync. Response: [Update]
  deploy-stack         Deploys the target stack. `docker compose up`. Response: [Update]
  start-stack          Starts the target stack. `docker compose start`. Response: [Update]
  restart-stack        Restarts the target stack. `docker compose restart`. Response: [Update]
  pause-stack          Pauses the target stack. `docker compose pause`. Response: [Update]
  unpause-stack        Unpauses the target stack. `docker compose unpause`. Response: [Update]
  stop-stack           Starts the target stack. `docker compose stop`. Response: [Update]
  destroy-stack        Destoys the target stack. `docker compose down`. Response: [Update]
  sleep                
  help                 Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### --yes

You can use `--yes` to avoid any human prompt to continue, for use in automated environments.

