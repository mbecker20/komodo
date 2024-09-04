# Komodo CLI

Komodo CLI is a tool to execute actions on your Komodo instance from shell scripts.

## Install

```sh
cargo install komodo_cli
```

Note: On Ubuntu, also requires `apt install build-essential pkg-config libssl-dev`.

## Usage

### Credentials

Configure a file `~/.config/komodo/creds.toml` file with contents:
```toml
url = "https://your.komodo.address"
key = "YOUR-API-KEY"
secret = "YOUR-API-SECRET"
```

Note. You can specify a different creds file by using `--creds ./other/path.toml`.
You can also bypass using any file and pass the information using `--url`, `--key`, `--secret`:

```sh
komodo --url "https://your.komodo.address" --key "YOUR-API-KEY" --secret "YOUR-API-SECRET" ...
```

### Run Executions

```sh
# Triggers an example build
komodo execute run-build test_build
```

#### Manual
`komodo --help`
```md
Command line tool to execute Komodo actions

Usage: komodo [OPTIONS] <COMMAND>

Commands:
  execute  Runs an execution
  help     Print this message or the help of the given subcommand(s)

Options:
      --creds <CREDS>    The path to a creds file [default: /Users/max/.config/komodo/creds.toml]
      --url <URL>        Pass url in args instead of creds file
      --key <KEY>        Pass api key in args instead of creds file
      --secret <SECRET>  Pass api secret in args instead of creds file
  -y, --yes              Always continue on user confirmation prompts
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

`komodo execute --help`
```md
Runs an execution

Usage: komodo execute <COMMAND>

Commands:
  none                    The "null" execution. Does nothing
  run-procedure           Runs the target procedure. Response: [Update]
  run-build               Runs the target build. Response: [Update]
  cancel-build            Cancels the target build. Only does anything if the build is `building` when called. Response: [Update]
  deploy                  Deploys the container for the target deployment. Response: [Update]
  start-deployment        Starts the container for the target deployment. Response: [Update]
  restart-deployment      Restarts the container for the target deployment. Response: [Update]
  pause-deployment        Pauses the container for the target deployment. Response: [Update]
  unpause-deployment      Unpauses the container for the target deployment. Response: [Update]
  stop-deployment         Stops the container for the target deployment. Response: [Update]
  destroy-deployment      Stops and destroys the container for the target deployment. Reponse: [Update]
  clone-repo              Clones the target repo. Response: [Update]
  pull-repo               Pulls the target repo. Response: [Update]
  build-repo              Builds the target repo, using the attached builder. Response: [Update]
  cancel-repo-build       Cancels the target repo build. Only does anything if the repo build is `building` when called. Response: [Update]
  start-container         Starts the container on the target server. Response: [Update]
  restart-container       Restarts the container on the target server. Response: [Update]
  pause-container         Pauses the container on the target server. Response: [Update]
  unpause-container       Unpauses the container on the target server. Response: [Update]
  stop-container          Stops the container on the target server. Response: [Update]
  destroy-container       Stops and destroys the container on the target server. Reponse: [Update]
  start-all-containers    Starts all containers on the target server. Response: [Update]
  restart-all-containers  Restarts all containers on the target server. Response: [Update]
  pause-all-containers    Pauses all containers on the target server. Response: [Update]
  unpause-all-containers  Unpauses all containers on the target server. Response: [Update]
  stop-all-containers     Stops all containers on the target server. Response: [Update]
  prune-containers        Prunes the docker containers on the target server. Response: [Update]
  delete-network          Delete a docker network. Response: [Update]
  prune-networks          Prunes the docker networks on the target server. Response: [Update]
  delete-image            Delete a docker image. Response: [Update]
  prune-images            Prunes the docker images on the target server. Response: [Update]
  delete-volume           Delete a docker volume. Response: [Update]
  prune-volumes           Prunes the docker volumes on the target server. Response: [Update]
  prune-system            Prunes the docker system on the target server, including volumes. Response: [Update]
  run-sync                Runs the target resource sync. Response: [Update]
  deploy-stack            Deploys the target stack. `docker compose up`. Response: [Update]
  start-stack             Starts the target stack. `docker compose start`. Response: [Update]
  restart-stack           Restarts the target stack. `docker compose restart`. Response: [Update]
  pause-stack             Pauses the target stack. `docker compose pause`. Response: [Update]
  unpause-stack           Unpauses the target stack. `docker compose unpause`. Response: [Update]
  stop-stack              Starts the target stack. `docker compose stop`. Response: [Update]
  destroy-stack           Destoys the target stack. `docker compose down`. Response: [Update]
  sleep                   
  help                    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### --yes

You can use `--yes` to avoid any human prompt to continue, for use in automated environments.

