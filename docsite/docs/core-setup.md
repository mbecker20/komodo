# core setup

setting up monitor core is fairly simple. there are some requirements to run monitor core:

 - a valid configuration file
 - an instance of MongoDB to which monitor core can connect
 - docker must be installed on the host

## 1. create the configuration file

create a configuration file on the system, for example at `~/.config/monitor/core.config.toml`, and copy the [example config](https://github.com/mbecker20/monitor/blob/main/config_example/core.config.example.toml). fill in all the necessary information before continuing.

:::note
to enable OAuth2 login, you must create a client on the respective OAuth provider, 
for example [google](https://developers.google.com/identity/protocols/oauth2) 
or [github](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps). 
monitor uses the `web application` login flow.
the redirect uri is `<base_url>/auth/google/callback` for google and `<base_url>/auth/github/callback` for github.
:::

:::note
all configuration can additionally be passed using environment variables, which override the value in the config file.
see ###CONFIG_RUST_DOCS###.
:::

## 2. start monitor core

monitor core is distributed via dockerhub under the public repo [mbecker2020/monitor_core](https://hub.docker.com/r/mbecker2020/monitor_core).

```sh
docker run -d --name monitor-core \
	-v $HOME/.monitor/core.config.toml:/config/config.toml \
	-p 9000:9000 \
	mbecker2020/monitor_core
```

## first login

monitor core should now be accessible on the specified port, so navigating to `http://<address>:<port>` will display the login page. 

the first user to log in will be auto enabled and made admin. any additional users to create accounts will be disabled by default.

## https

monitor core itself only supports http, so a reverse proxy like [caddy](https://caddyserver.com/) should be used for https