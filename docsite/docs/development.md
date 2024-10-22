# Development

## Dependencies

Running Komodo from [source](https://github.com/mbecker20/komodo) requires either [Docker](https://www.docker.com/), use of the included [devcontainer](https://code.visualstudio.com/docs/devcontainers/containers) or these dependencies installed:

* For backend (Komodo core server, periphery, API)
    * [Rust](https://www.rust-lang.org/) stable 1.81
    * [rustup](https://rustup.rs/)
    * [MongoDB](https://www.mongodb.com/) compatible database
* For frontend (Web UI)
    * [Node](https://nodejs.org/en) >= 18.18 + NPM
        * [Yarn](https://yarnpkg.com/)
    * [typeshare](https://github.com/1password/typeshare)
    * [Deno](https://deno.com/) >= 2.0.2

## Docker

After making changes to the project simply run `run test-compose-build` to rebuild Komodo and then `run test-compose-exposed` to start a Komodo container with the UI accessible at `localhost:9120`. Any changes made to source files will require re-running the build and exposed commands.

## Devcontainer

Use the included `.devcontainer.json` with VSCode or other compatible IDE to stand-up a full environment, including database, with one click.

[VSCode Tasks](https://code.visualstudio.com/Docs/editor/tasks) are provded for building and running Komodo. 

After opening the repository with the devcontainer run the task `Init` to build the frontend/backend. Then, the task `Run Komodo` can be used to run frontend/backend. Other tasks for rebuilding/running only parts of the application are also provided.

## Local

[runnables-cli](https://github.com/mbecker20/runnables-cli) can be used as a convience for running common project tasks (like a Makefile) found in `runfile.toml`. Otherwise, you can create your own project tasks by references the `cmd`s found in `runfile.toml`. All instructions below will use runnables-cli.

To run a full Komodo instance from a non-container environment run commands in this order:

* Ensure dependencies are up to date
    * `rustup update` -- ensure rust toolchain is up to date
* Build and Run backend
    * `run test-core` -- builds core binary
    * `run test-periphery` -- builds periphery binary
* Build Frontend
    * `run gen-client` -- generates TS client and adds to the frontend
    * Prepare API Client
        * `cd client/core/ts && yarn && yarn build && yarn link`
            * After running once client can be rebuilt with `run build-ts-client`
    * [Prepare Frontend](/frontend//README.md)
        * `cd frontend && yarn link komodo_client && yarn install`
            * After running once client can be built with `run build-frontend` or started in dev (watch) mode with `run start-frontend`

### Docs

Use `run docsite-start` to start the [Docusaurus](https://docusaurus.io/) Komodo docs site in development mode. Changes made to files in `/docsite` will be automatically reloaded by the server.