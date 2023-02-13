# connecting servers

Integrating a device into the monitor system has 2 steps:

 1. Setup and start the periphery agent on the server
 2. Adding the server to monitor via the core API

## setup monitor periphery

The easiest way to do this is to follow the [monitor guide](https://github.com/mbecker20/monitor-guide). This is a repo containing directions and scripts enabling command line installation via ssh or remotely.

### manual install steps

 1. Download the periphery binary from the latest [release](https://github.com/mbecker20/monitor/releases) or install it using [cargo](https://crates.io/crates/monitor_periphery). If the monitor cli.
 2. Create and edit ~/.monitor/periphery.config.toml, following the [config example](https://github.com/mbecker20/monitor/blob/main/config_example/periphery.config.example.toml). The file can be anywhere, it can be passed to periphery via the --config-path flag or with the CONFIG_PATH environment variable. The monitor cli can also be used: ```monitor periphery gen-config```
 3. Ensure that inbound connectivity is allowed on the port specified in periphery.config.toml (default 8000).
 4. Install docker. Make sure whatever user periphery is run as has access to the docker group without sudo.
 5. Start the periphery binary with your preferred process manager, like systemd. The config read from the file is printed on startup, ensure that it is as expected.

## adding the server to monitor

The easiest way to add the server is with the GUI. On the home page, click the + button to the right of the server search bar, configure the name and address of the server. The address is the full http/s url to the periphery server, eg http://12.34.56.78:8000.

Once it is added, you can use access the GUI to modify some config, like the alerting thresholds for cpu, memory and disk usage. A server can also be temporarily disabled, this will prevent alerting if it goes offline.

Since no state is stored on the periphery servers, you can easily redirect all builds / deployments to be hosted on a different server. Just update the address to point to the new server.

[next: building](https://github.com/mbecker20/monitor/blob/main/docs/builds.md)

[back to table of contents](https://github.com/mbecker20/monitor/blob/main/readme.md)