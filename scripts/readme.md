# Periphery setup script

These scripts will set up Komodo Periphery on your hosts, managed by systemd.

*Note*. This script can be run multiple times without issue, and it won't change existing config after the first run. Just run it again after a Komodo version release, and it will update the periphery version.

*Note*. The script can usually detect aarch64 system and use the periphery-aarch64 binary.

There's two ways to install periphery: `System` and `User`

## System (requires root)

Note. Run this after switching to root user (eg `sudo su -`).

```sh
curl -sSL https://raw.githubusercontent.com/mbecker20/komodo/main/scripts/setup-periphery.py | python3
```

Will install to paths:
- periphery (binary) -> `/usr/local/bin/periphery`
- periphery.service -> `/etc/systemd/system/periphery.service`
- periphery.config.toml -> `/etc/komodo/periphery.config.toml`

## User

*Note*. The user running periphery must be a member of the docker group, in order to use the docker cli without sudo.

*Note*. Ensure the user running periphery has write access to the configure [repo directory](https://github.com/mbecker20/komodo/blob/main/config/periphery.config.toml).
This allows periphery to clone repos and write compose files.

```sh
curl -sSL https://raw.githubusercontent.com/mbecker20/komodo/main/scripts/setup-periphery.py | python3 - --user
```

Will install to paths:
- periphery (binary) -> `$HOME/.local/bin`
- periphery.service -> `$HOME/.config/systemd/user/periphery.service`
- periphery.config.toml -> `$HOME/.config/komodo/periphery.config.toml`