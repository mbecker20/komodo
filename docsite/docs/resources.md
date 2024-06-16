# Resources

Entities like `Server`, `Deployment`, and `Build` are abstracted under a `Resource`. A server is a type of resource, a build is a type of resource, and so on.
All resources have common traits, such as a globally unique `name` and `id` amongst all other resources of the same resource type.
All resources can also be assigned `tags`, which can be used to group related resources.
For example, they can be grouped by environment (using `dev` or `prod` tags), or by function (`auth` or `ingress`), 
or really by any way that fits best for your infra (they are all user defined).

Here is a list of the resources and their description:
- `Server`: Represents a connected server. 
 	- Holds server config, like the address.
- `Deployment`: Represents a docker container on a server, whether it is actually deployed or not.
	- Holds deployment config, like the server it should deploy on, and the image / build to deploy.
- `Build`: Represents a docker image.
	- Holds build config, like the source repo, Dockerfile location, and version
- `Repo`: Represents a repo on a server, whether it is cloned or not.
	- Holds repo config, like the source repo, and the `on_clone` and `on_pull` commands, which run after the repo is cloned / pulled
- `Procedure`: Configure higher level actions by composing lower level actions.
	- Holds the actions to execute, like `RunBuild build_1` and `Deploy deployment_1`, and the order to execute them
- `Alerter`: Route the various alerts produced by monitor to alerting endpoints
	- Holds the alerting endpoint (Slack channel or Custom http POST), the alerting types to forward (eg. `ServerUnreachable` or `ContainerStateChange`).
- `Builder`: Represents a server used as the "builder" for builds. Can be connected server or ephemeral AWS server.
	- Holds builder config, like the AWS ami-id and security groups to allow for builder reachability.
- `ServerTemplate`: Configure cloud server templates (currently AWS and Hetzner) to easily launch more instances and auto connect them to Monitor
	- Holds the cloud server config
- `ResourceSync`: Declare Monitor resources in TOML files, push them to a Github Repo, and sync Monitor config from them.
	- Holds config for the source repo containing the files. Will display the computed diff and wait for user to execute.