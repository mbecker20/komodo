# Development

## Dependencies

Running Komodo from [source](https://github.com/mbecker20/komodo) requires either [Docker](https://www.docker.com/) or these dependencies installed:

* For backend (Komodo core server, periphery)
    * [Rust](https://www.rust-lang.org/) stable 1.81(?)
* For frontend (Web UI)
    * [Node](https://nodejs.org/en) > 18 + NPM
        * [Yarn](https://yarnpkg.com/)

Optionally, [runnables-cli](https://github.com/mbecker20/runnables-cli) can be used as a convience for running common project tasks (like a Makefile) found in `runfile.toml`. Otherwise, you can create your own project tasks by references the `cmd`s found in `runfile.toml`. All instructions below will use runnables-cli.

## Docker

(TBD adding a port to `test.compose.yaml`)

After making changes to the project simply run `run test-compose-build` to rebuild Komodo and then `run test-compose` to start a Komodo container with the UI accessible at `localhost:9120`.

## Local and Docs

### Komodo

To run a full Komodo instance run commands in this order:

* `cargo build` -- builds core and periphery
* `run test-core`
* `run test-periphery`
* `run gen-ts-types`
* `run build-ts-client`
* `run build-frontend`
* `run start-fronted`

(TBD specifying running only parts of Komodo or rebuild/restart after code iteration)

### Docs

Use `run docsite-start` to start the [Docusaurus](https://docusaurus.io/) Komodo docs site in development mode. Changes made to files in `/docsite` will be automatically reloaded by the server.