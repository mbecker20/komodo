---
slug: /intro
---

# What is Monitor?

Monitor is a web app to provide structure for managing your servers, builds, deployments, and automated procedures.

With Monitor you can:

 1. Build application source into auto-versioned images. 
 2. Create, start, stop, and restart Docker containers, and view their status and logs.
 3. Manage repositories on remote servers.
 4. Keep a record of all the actions that are performed and by whom.
 5. View realtime and historical system resource usage, and alert for out of bounds values.

## Docker

Monitor is opinionated by design, and uses [docker](https://docs.docker.com/) as the container engine for building and deploying.

## Architecture and Components

Monitor is composed of a single core and any amount of connected servers running the periphery application. 

### Core
Monitor Core is a web server hosting the Core API and browser UI. All user interaction with the connected servers flow through the Core. It is the stateful part of the system, with the application state stored on an instance of MongoDB.

### Periphery
Monitor Periphery is a small stateless web server that runs on all connected servers. It exposes an API called by Monitor Core to perform actions on the server, get system usage, and container status / logs. It is only intended to be reached from the core, and has an address whitelist to limit the IPs allowed to call this API.

## Core API

Monitor exposes powerful functionality over the Core's REST and Websocket API, enabling infrastructure engineers to manage their infrastructure programmatically. There is a [rust crate](https://crates.io/crates/monitor_client) to simplify programmatic interaction with the API, but in general this can be accomplished using any programming language that can make REST requests. 

## Permissioning

Monitor is a system designed to be used by many users, whether they are developers, operations personnel, or administrators. The ability to affect an applications state is very powerful, so monitor has a granular permissioning system to only provide this functionality to the intended users. The permissioning system is explained in detail in the [permissioning](/docs/permissioning) section. 

User sign-on is possible using username / password, or with Oauth (Github and Google). See [Core Setup](/docs/core-setup).