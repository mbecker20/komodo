# Komodo Core Setup

To run Komodo Core, you will need Docker. See [the docker install docs](https://docs.docker.com/engine/install/).

### Deploy Komodo Core with Docker Compose

There is an example compose file here: [https://github.com/mbecker20/komodo/blob/main/config_example/core.compose.yaml](https://github.com/mbecker20/komodo/blob/main/config_example/core.compose.yaml).

Copy the contents to a `compose.yaml`, and deploy it with `docker compose up -d`.

:::info
Komodo Core itself can really only run remote builds.
You also have to [**install the Komodo Periphery agent**](/docs/connecting-servers) on your hosts and connect them as **Servers**
in order to alert / deploy etc.

If you **only need to connect on one server** (the one you are deploying Komodo Core on), you can do it all dockerized,
and use the [**all-in-one compose file**](https://github.com/mbecker20/komodo/blob/main/config_example/aio.compose.yaml).
This will deploy Komodo Core and Periphery, and automatically add the local periphery as a connected server. 

Deploying with the AIO compose file **will not** stop you from connecting more servers later, and is really just for setup convenience.

You can currently and always will be able to **connect as many servers an you like** using the Periphery agent. 
:::

### Configuration

You can configure Komodo with environment variables, or using a config file.

The example config file in the Komodo repo documents all the configuration options, along with the corresponding environment variables.
It can be found here: [https://github.com/mbecker20/komodo/blob/main/config_example/core.config.example.toml](https://github.com/mbecker20/komodo/blob/main/config_example/core.config.example.toml).

Note that configuration passed in environment variables will take precedent over what is given in the file.

:::note
To enable OAuth2 login, you must create a client on the respective OAuth provider,
for example [google](https://developers.google.com/identity/protocols/oauth2)
or [github](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps).
Komodo uses the `web application` login flow.
The redirect uri is `<base_url>/auth/google/callback` for google and `<base_url>/auth/github/callback` for github.
:::

### First login

Core should now be accessible on the specified port, so navigating to `http://<address>:<port>` will display the login page.

The first user to log in will be auto enabled and made an admin. Any additional users to create accounts will be disabled by default, and must be enabled by an admin.

### Tls

Core itself only supports http, so a reverse proxy like [caddy](https://caddyserver.com/) should be used for https.

## Deploy with Docker cli

### 1. Start Mongo

Mongo can be run locally using the docker cli:

```sh
docker run --name komodo-mongo \
	--network host \
	-v /local/storage/path:/data/db \
	-e MONGO_INITDB_ROOT_USERNAME="admin" \
	-e MONGO_INITDB_ROOT_PASSWORD="admin" \
	mongo:latest
```

You should replace the username and password with your own.
See [the image docs](https://hub.docker.com/_/mongo) for more details.

Note that this uses "host" networking, which will allow core to connect over localhost.
Many users will prefer the default "bridge" network, and to use port mapping with `-p 27017:27017`.

:::note
The disk space requirements of Komodo are dominated by the storage of system stats.
This depends on the number of connected servers (more system stats being produces / stored), stats collection frequency, and your stats pruning configuration.
If you need to save on space, you can configure these fields in your core config: - Stats poll frequency can be reduced using, for example, `monitoring_interval = "15-sec"` - Pruning can be tuned more aggresively using, for example, `keep_stats_for_days = 7`.
:::

### 2. Start Komodo core

Komodo core is distributed via Github Container Registry under the package [mbecker20/komodo](https://github.com/mbecker20/komodo/pkgs/container/komodo).

```sh
docker run -d --name komodo-core \
	--network host \
	-v $HOME/.config/komodo/core.config.toml:/config/config.toml \
	ghcr.io/mbecker20/komodo:latest
```

Note that this uses "host" networking, which will allow it to connect to a local periphery agent on localhost.
Many users will prefer the default "bridge" network, and to use port mapping with `-p 9120:9120`.
