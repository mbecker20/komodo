# Docker Compose

Monitor supports docker compose through the `Stack` resource.

## Define the compose file/s

Monitor supports 3 ways of defining the compose files:
	1. **Write them in the UI**, and Monitor will write them to your host at deploy-time.
	2. **Store them in a git repo**, and have Monitor clone it on the host to deploy.
	3. **Store the files anywhere on the host**, and Monitor will just run the compose commands on the existing files.

The recommended way to deploy Stacks is using compose files located in a git repo.

If you manage your compose files in git repos:

- All your files, across all servers, are available locally to edit in your favorite text editor.
- All of your changes are tracked, and can be reverted.
- You can use the git webhooks to do other automations when you change the compose file contents. Redeploying will be as easy as just `git push`.

:::info
Many Monitor resources need access to git repos. There is an in-built token management system (managed in UI or in config file) to give resources access to credentials.
All resources which depend on git repos are able to use these credentials to access private repos.
:::

## Importing Existing Compose projects

First create the Stack in Monitor, and ensure it has access to the compose files using one
of the three methods above. Make sure to attach the server you wish to deploy on.

In order for Monitor to pick up a running project, it has to know the compose "project name".
You can find the project name by running `docker compose ls` on the host.

By default, Monitor will assume the Stack name is the compose project name.
If this is different than the project name on the host, you can configure a custom "Project Name" in the config.

## Pass Environment Variables

Monitor is able to pass custom environment variables to the docker compose process.
This works by:

1. Write the variables to a ".env" file on the host at deploy-time.
2. Pass the file to docker compose using the `--env-file` flag.

:::info
Just like all other resources with Environments (Deployments, Repos, Builds),
Stack Environments support **Variable and Secret interpolation**. Define global variables
in the UI and share the values across environments.
:::