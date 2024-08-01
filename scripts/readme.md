# Periphery setup script

*Note*. This script can be run multiple times without issue, and it won't change existing config after the first run. Just run it again after a Monitor version release, and it will update the periphery version.

*Note*. The script can usually detect aarch64 system and use the periphery-aarch64 binary.

There's two ways to install periphery: `System` and `User`

## System (requires root)

Note. Run this after switching to root user (eg `sudo su -`).

```sh
curl -sSL https://raw.githubusercontent.com/mbecker20/monitor/main/scripts/setup-periphery.py | python3
```

Will install to paths:
- periphery (binary) -> `/usr/local/bin/periphery`
- periphery.service -> `/etc/systemd/system/periphery.service`
- periphery.config.toml -> `/etc/monitor/periphery.config.toml`

## User

*Note*. The user running periphery must be a member of the docker group, in order to use the docker cli without sudo.

*Note*. Ensure the user running periphery has write access to the configure [repo directory](https://github.com/mbecker20/monitor/blob/5f0a9ad65228a5c43bdcdfd5335c6e4bf2112591/config_example/periphery.config.example.toml#L5).
This allows periphery to clone repos and write compose files.

```sh
curl -sSL https://raw.githubusercontent.com/mbecker20/monitor/main/scripts/setup-periphery.py | python3 - --user
```

Will install to paths:
- periphery (binary) -> $HOME/.local/bin
- periphery.service -> $HOME/.config/systemd/user/periphery.service
- periphery.config.toml -> $HOME/.config/monitor/periphery.config.toml