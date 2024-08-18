# Connecting Servers

Connecting a server to monitor has 2 steps:

1.  Install the Periphery agent on the server
2.  Adding the server to monitor via the core API

Once step 1. is complete, you can just connect the server to Monitor Core from the UI.

## Install

You can install Periphery as a systemd managed process, run it as a [docker container](https://github.com/mbecker20/monitor/pkgs/container/periphery), or do whatever you want with the binary.

Some Periphery actions interact with your hosts file system, like cloning repos, or accessing local compose files.
For this reason, runnning periphery in a container can be a bit more complicated.
Additionally, Periphery in a container tends to overreport the disks by default, but this can be fixed via some configuration.

### Install the Periphery agent - systemd

As root user:
```sh
curl -sSL https://raw.githubusercontent.com/mbecker20/monitor/main/scripts/setup-periphery.py | python3
```

Periphery can also be installed to run as the calling user, just note this comes with some additional configuration.

```sh
curl -sSL https://raw.githubusercontent.com/mbecker20/monitor/main/scripts/setup-periphery.py | python3 - --user
```

You can find more information (and view the script) in the [readme](https://github.com/mbecker20/monitor/tree/main/scripts).

:::info
This script can be run multiple times without issue, and it won't change existing config after the first run. Just run it again after a Monitor version release, and it will update the periphery version.
:::

### Install the Periphery agent - container

You can use a docker compose file like this:
```yaml
services:
  monitor-periphery:
    image: ghcr.io/mbecker20/periphery:latest # use ghcr.io/mbecker20/periphery:latest-aarch64 for arm support
    logging:
      driver: local
		ports:
			- 8120:8120
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - monitor-repos:/etc/monitor/repos # manage repos in a docker volume, or change it to an accessible host directory.
    # environment:
    #   # If the disk size is overreporting, can use one of these to 
    #   # whitelist / blacklist the disks to filter them, whichever is easier. 
    #   PERIPHERY_INCLUDE_DISK_MOUNTS: /etc/monitor/repos 
    #   PERIPHERY_EXCLUDE_DISK_MOUNTS: /snap
```

### Manual install steps - binaries

1.  Download the periphery binary from the latest [release](https://github.com/mbecker20/monitor/releases).

2.  Create and edit your config files, following the [config example](https://github.com/mbecker20/monitor/blob/main/config_example/periphery.config.example.toml).

:::note
See the [periphery config docs](https://docs.rs/monitor_client/latest/monitor_client/entities/config/periphery/index.html)
for more information on configuring periphery.
:::

3.  Ensure that inbound connectivity is allowed on the port specified in periphery.config.toml (default 8120).

4.  Install docker. See the [docker install docs](https://docs.docker.com/engine/install/).

:::note
Ensure that the user which periphery is run as has access to the docker group without sudo.
:::

5.  Start the periphery binary with your preferred process manager, like systemd.

### Example periphery start command

```
periphery \
	--config-path /path/to/periphery.config.base.toml \
	--config-path /other_path/to/overide-periphery-config-directory \
	--config-keyword periphery \
	--config-keyword config \
	--merge-nested-config true
```

### Passing config files

Either file paths or directory paths can be passed to `--config-path`.

When using directories, the file entries can be filtered by name with the `--config-keyword` argument, which can be passed multiple times to add more keywords. If passed, then only config files with file names that contain all keywords will be merged.

When passing multiple config files, later --config-path given in the command will always overide previous ones. Directory config files are merged in alphabetical order by name, so `config_b.toml` will overide `config_a.toml`.

There are two ways to merge config files. The default behavior is to completely replace any base fields with whatever fields are present in the overide config. So if you pass `allowed_ips = []` in your overide config, the final allowed_ips will be an empty list as well.

`--merge-nested-config true` will merge config fields recursively and extend config array fields.

For example, with `--merge-nested-config true` you can specify an allowed ip in the base config, and another in the overide config, they will both be present in the final config.

Similarly, you can specify a base docker / github account pair, and extend them with additional accounts in the overide config.