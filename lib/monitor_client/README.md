# monitor client

## *interact with the monitor system programatically*

with this crate you can leverage all the functionality of monitor through rust code. for example, you can...

 - create and manage complex deployment cycles
 - execute builds on specific schedules
 - monitor server stats programatically
 - program response actions to monitor updates sent over websocket

## initialize the client

you can initialize the client by directly passing args to the various initializers:

```rust
use monitor_client::MonitorClient;

let MONITOR_URL: &str = "https://monitor.mogh.tech";

let monitor = MonitorClient::new_with_token(MONITOR_URL, jwt_token).await?; // pass a valid jwt
let monitor = MonitorClient::new_with_password(MONITOR_URL, username, password).await?; // pass local user credentials
let monitor = MonitorClient::new_with_secret(MONITOR_URL, username, secret).await?; // pass api secret
```

or from the application environment / dotenv:

```sh
MONITOR_URL=https://monitor.mogh.tech # required
MONITOR_TOKEN=<jwt>                   # optional. pass the jwt directly.
MONITOR_USERNAME=<username>           # required for password / secret login
MONITOR_PASSWORD=<password>           # the users password
MONITOR_SECRET=<secret>               # the api secret
```

to log in, you must pass either
 1. MONITOR_TOKEN
 2. MONITOR_USERNAME and MONITOR_PASSWORD
 3. MONITOR_USERNAME and MONITOR_SECRET

you can then initialize the client using this method:
```rust
let monitor = MonitorClient::new_from_env().await?;
```

## use the client

the following will select a server, build monitor core on it, and deploy it.

```rust
let server = monitor
	.list_servers(None)
	.await?
	.pop()
	.ok_or(anyhow!("no servers"))?;

let build = BuildBuilder::default()
	.name("monitor_core".into())
	.server_id(server.server.id.clone().into())
	.repo("mbecker20/monitor".to_string().into())
	.branch("main".to_string().into())
	.docker_build_args(
		DockerBuildArgs {
			build_path: ".".into(),
			dockerfile_path: "core/Dockerfile".to_string().into(),
			..Default::default()
		}
		.into(),
	)
	.pre_build(
		Command {
			path: "frontend".into(),
			command: "yarn && yarn build".into(),
		}
		.into(),
	)
	.build()?;

let build = monitor.create_full_build(&build).await?;

println!("{build:#?}");

let build_update = monitor.build(&build.id).await?;

println!("{build_update:#?}");

let deployment = DeploymentBuilder::default()
	.name("monitor_core_1".into())
	.server_id(server.server.id.clone())
	.build_id(build.id.clone().into())
	.docker_run_args(
		DockerRunArgsBuilder::default()
			.volumes(vec![Conversion {
				local: "/home/max/.monitor/core.config.toml".into(),
				container: "/config/config.toml".into(),
			}])
			.build()?,
	)
	.build()?;

let deployment = monitor.create_full_deployment(&deployment).await?;

println!("{deployment:#?}");

let deploy_update = monitor.deploy_container(&deployment.id).await?;

println!("{deploy_update:#?}");
```

note. this crate re-exports the [monitor types](https://crates.io/crates/monitor_types) crate under the module "types"