# Core Setup

To run Monitor Core, you will need:

 - A valid configuration file.
 - An instance of MongoDB to which Core can connect.
 - Docker must be installed on the host.

## 1. Create the configuration file

Create a configuration file on the system, for example at `~/.config/monitor/core.config.toml`, and copy the [example config](https://github.com/mbecker20/monitor/blob/main/config_example/core.config.example.toml). Fill in all the necessary information before continuing.

:::note
To enable OAuth2 login, you must create a client on the respective OAuth provider, 
for example [google](https://developers.google.com/identity/protocols/oauth2) 
or [github](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps). 
Monitor uses the `web application` login flow.
The redirect uri is `<base_url>/auth/google/callback` for google and `<base_url>/auth/github/callback` for github.
:::

:::note
Most configuration can additionally be passed using environment variables, which override the value in the config file.
See [config docs](https://docs.rs/monitor_client/latest/monitor_client/entities/config/core/index.html).
:::

## 2. Start monitor core

Monitor core is distributed via Github Container Registry under the package [mbecker20/monitor_core](https://github.com/mbecker20/monitor/pkgs/container/monitor_core).

```sh
docker run -d --name monitor-core \
	-v $HOME/.monitor/core.config.toml:/config/config.toml \
	-p 9000:9000 \
	ghcr.io/mbecker20/monitor_core
```

## First login

Core should now be accessible on the specified port, so navigating to `http://<address>:<port>` will display the login page. 

The first user to log in will be auto enabled and made admin. any additional users to create accounts will be disabled by default.

## Tls

Core itself only supports http, so a reverse proxy like [caddy](https://caddyserver.com/) should be used for https.