# Changelog

## <ins>Komodo v1.13 (Sep 2024)</ins>
- Renamed the project to **Komodo**.
- Manage docker networks, volumes, and images.
- Manage Containers at the server level, without creating any Deployment.
- Add bulk Start / Restart / Pause actions for all containers on a server.
- Add **Secret** mode to Variables to hide the value in updates / logs
	- Secret mode also prevents any non-admin users from retrieving the value from the API. Non admin users will still see the variable name.
- Interpolate Variables / Secrets into everything I could think of 
	- Deployment / Stack / Repo / Build **extra args**.
	- Deployment **command**.
	- Build **pre build**.
	- Repo **on_clone / on_pull**.
- Added **Hetzner Singapore** datacenter for Hetzner ServerTemplates
- **Removed Google Font** - now just use system local font to avoid any third party calls.

## <ins>Monitor v1.13 - Komodo (Aug 2024)</ins>
- This is the first named release, as I think it is really big. The Komodo Dragon is the largest species of Monitor lizard.
- **Deploy docker compose** with the new **Stack** resource.
	- Can define the compose file in the UI, or direct Monitor to clone a git repo containing compose files.
	- Use webhooks to redeploy the stack on push to the repo
	- Manage the environment variables passed to the compose command.
- **Builds** can now be configured with an alternate repository name to push the image under.
	-An optional tag can also be configured to be postfixed onto the version, like image:1.13-aarch64. 
	This helps for pushing alternate build configurations under the same image repo, just under different tags.
- **Repos** can now be "built" using builders. The idea is, you spawn an AWS instance, clone a repo, execute a shell command
(like running a script in the repo), and terminating the instance. The script can build a binary, and push it to some binary repository.
Users will have to manage their own versioning though.
- **High level UI Updates** courtesy of @karamvirsingh98

## <ins>v1.12 (July 2024)</ins>
- Break free of Github dependance. Use other git providers, including self hosted ones.
- Same for Docker registry. You can also now use any docker registry for your images.

## <ins>v1 (Spring 2024)</ins>

- **New resource types**:
	- **Repo**: Clone / pull configured repositories on desired Server. Run shell commands in the repo on every clone / pull to acheive automation. Listen for pushes to a particular branch to automatically pull the repo and run the command.
	- **Procedure**: Combine multiple *executions* (Deploy, RunBuild) and run them in sequence or in parallel. *RunProcedure* is an execution type, meaning procedures can run *other procedures*.
	- **Builder**: Ephemeral builder configuration has moved to being an API / UI managed entity for greater observability and ease of management.
	- **Alerter**: Define multiple alerting endpoints and manage them via the API / UI.
		- Slack support continues with the *Slack* Alerter variant.
		- Send JSON serialized alert data to any HTTP endpoint with the *Custom* Alerter variant.
	- **Template**: Define a template for your cloud provider's VM configuration
		- Launch VMs based on the template and automatically add them as Monitor servers.
		- Supports AWS EC2 and Hetzner Cloud.
	- **Sync**: Sync resources declared in toml files in Github repos.
		- Manage resources declaratively, with git history for configuration rollbacks.
		- See the actions which will be performed in the UI, and execute them upon manual confirmation.
		- Use a Git webhook to automatically execute syncs on git push.

- **Resource Tagging**
	- Attach multiple *tags* to resources, which can be used to group related resources together. These can be used to filter resources in the UI.
	- For example, resources can be given tags for *environment*, like `Prod`, `Uat`, or `Dev`. This can be combined with tags for the larger system the resource is a part of, such as `Authentication`, `Logging`, or `Messaging`.
	- Proper tagging will make it easy to find resources and visualize system components, even as the number of resources grows large.

- **Variables**
	- Manage global, non-secret key-value pairs using the API / UI.
	- These values can be interpolated into deployment `environments` and build `build_args`

- **Core Accounts and Secrets**
	- Docker / Github accounts and Secrets can now be configured in the Core configuration file.
	- They can still be added to the Periphery configuration as before. Accounts / Secrets defined in the Core configuration will be preferentially used over like ones defined in Periphery configuration.

- **User Groups**
	- Admins can now create User Groups and assign permissions to them as if they were a user. 
	- Multiple users can then be added to the group, and a user can be added to multiple groups.
	- Users in the group inherit the group's permissions. 

- **Builds**
	- Build log displays the **latest commit hash and message**.
	- In-progress builds are able to be **cancelled before completion**.
	- Specify a specific commit hash to build from.

- **Deployments**
	- Filter log lines using multiple search terms.

- **Alerting**
	- The alerting system has been redesigned with a stateful model.
	- Alerts can be in an Open or a Resolved state, and alerts are only sent on state changes.
	- For example, say a server has just crossed 80% memory usage, the configured memory threshold. An alert will be created in the Open state and the alert data will be sent out. Later, it has dropped down to 70%. The alert will be changed to the Resolved state and the alert data will again be sent.
	- In addition to server usage alerts, Monitor now supports deployment state change alerts. These are sent when a deployment's state changes without being caused by a Monitor action. For example, if a deployment goes from the Running state to the Exited state unexpectedly, say from a crash, an alert will be sent.
	- Current and past alerts can be retrieved using the API and viewed on the UI.

- **New UI**:
	- The Monitor UI has been revamped to support the new features and improve the user experience.

## <ins>v0 (Winter 2022)</ins>

- Move Core and Periphery implementation to Rust.
- Add AWS Ec2 ephemeral build instance support.
	- Configuration added to core configuration file.
- Automatic build versioning system, supporting image rollback.
- Realtime and historical system stats - CPU, memory, disk.
- Simple stats alerting based on out-of-bounds values for system stats.
	- Support sending alerts to Slack.

## <ins>Pre-versioned releases</ins>

- Defined main resource types:
	- Server
	- Deployment
	- Build
- Basics of Monitor:
	- Build docker images from Github repos.
	- Manage image deployment on connected servers, see container status, get container logs.
	- Add account credentials in Periphery configuration.
- Core and Periphery implemented in Typescript.