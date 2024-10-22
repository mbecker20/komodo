# Development

## Dependencies

Running Komodo from [source](https://github.com/mbecker20/komodo) requires either [Docker](https://www.docker.com/) or these dependencies installed:

* For backend (Komodo core server, periphery, API)
    * [Rust](https://www.rust-lang.org/) stable 1.81
    * [rustup](https://rustup.rs/)
    * [MongoDB](https://www.mongodb.com/) compatible database
* For frontend (Web UI)
    * [Node](https://nodejs.org/en) >= 18.18 + NPM
        * [Yarn](https://yarnpkg.com/)
    * [typeshare](https://github.com/1password/typeshare)

Optionally, [runnables-cli](https://github.com/mbecker20/runnables-cli) can be used as a convience for running common project tasks (like a Makefile) found in `runfile.toml`. Otherwise, you can create your own project tasks by references the `cmd`s found in `runfile.toml`. All instructions below will use runnables-cli.

## Docker

After making changes to the project simply run `run test-compose-build` to rebuild Komodo and then `run test-compose-exposed` to start a Komodo container with the UI accessible at `localhost:9120`.

## Local and Docs

### Komodo

To run a full Komodo instance run commands in this order:

* Ensure dependencies are up to date
    * `rustup update` -- ensure rust toolchain is up to date
* Build backend
    * `cargo build` -- builds core and periphery
    * `run test-core` -- builds core binary
    * `run test-periphery` -- builds periphery binary
* Build Frontend
    * `run gen-ts-types` -- generates types for use with building typescript client
    * Prepare API Client
        * `cd client/core/ts && yarn && yarn build && yarn link`
            * After running once client can be rebuilt with `run build-ts-client`
    * [Prepare Frontend](/frontend//README.md)
        * `cd frontend && yarn link komodo_client && yarn install`
            * After running once client can be built with `run build-frontend` or started in dev (watch) mode with `run start-frontend`

### Docs

Use `run docsite-start` to start the [Docusaurus](https://docusaurus.io/) Komodo docs site in development mode. Changes made to files in `/docsite` will be automatically reloaded by the server.