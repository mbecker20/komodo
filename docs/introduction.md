

# introduction

If you have many servers running many applications, it can be a challenge to keep things organized and easily accessible. Without structure, things can become messy quickly, which means operational issues are more likely to arise and they can take longer to resolve. Ultimately these issues hinder productivity and waste valuable time. Monitor is a web app to provide this structure for how applications are built, deployed, and managed across many servers.

## docker

Monitor is opinionated by design, and [docker](https://docs.docker.com/) is the tool of choice. Docker provides the ability to package applications and their runtime dependencies into a standalone bundle, called an *image*. This makes them easy to "ship" to any server and run without the hassle of setting up the runtime environment. Docker uses the image as a sort of template to create *containers*. Containers are kind of like virtual machines but with different performance characteristics, namely that processes contained still run natively on the system kernel. The file system is seperate though, and like virtual machines, they can be created, started, stopped, and destroyed.

## monitor

Monitor is a solution for handling for the following:

 1. Build application source into auto-versioned images. 
 2. Create, start, stop, and restart Docker containers, and view their status and logs.
 3. Keep a record of all the actions that are performed and by whom.
 4. View realtime and historical system resource usage.
 5. Alerting for server health, like high cpu, memory, disk, etc.

## architecture and components

Monitor is composed of a single core and any amount of connected servers running the periphery application. 

### monitor core
The core is a web server that hosts the core API and serves the frontend to be accessed in a web browser. All user interaction with the connected servers flow through the core. It is the stateful part of the system, with the application state stored on an instance of MongoDB.

### monitor periphery
The periphery is a stateless web server that exposes API called by the core. The core calls this API to get system usage and container status / logs, clone git repos, and perform docker actions. It is only intended to be reached from the core, and has an address whitelist to limit the IPs allowed to call this API.

### monitor cli
This is a simple standalone cli that helps perform some actions required to setup monitor core and periphery, like generating config files. 

## core API

Monitor exposes powerful functionality over the core's REST API, enabling infrastructure engineers to manage deployments programmatically in addition to with the GUI. There is a [rust crate](https://crates.io/crates/monitor_client) to simplify programmatic interaction with the API, but in general this can be accomplished using any programming language that can make REST requests. 

## permissioning

Monitor is a system designed to be used by many users, whether they are developers, operations personnel, or administrators. The ability to affect an applications state is very powerful, so monitor has a granular permissioning system to only provide this functionality to the intended users. The permissioning system is explained in detail in the [permissioning](https://github.com/mbecker20/monitor/blob/main/docs/permissions.md) section. 

User sign-on is possible using username / password, or with Oauth (Github and Google). Allowed login methods can be configured from the [core config](https://github.com/mbecker20/monitor/blob/main/config_example/core.config.example.toml).

[next: connecting servers](https://github.com/mbecker20/monitor/blob/main/docs/servers.md)

[back to table of contents](https://github.com/mbecker20/monitor/blob/main/readme.md)