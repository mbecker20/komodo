# Docker Compose

Monitor supports docker compose through the `Stack` resource. Just create a new Stack, with any name, to get started.

## Define the file/s
While Monitor supports pasting in / managing the compose file in UI, the best way to deploy Stacks is using compose files located in a git repo.

If you manage your compose files in git repos:

- All your files, across all servers, are available locally to edit in your favorite text editor.
- All of your changes are tracked, and can be reverted.
- You can layer multiple compose files for greater composability, just like using `docker compose -f service_1.yaml -f service_2.yaml ...`
- You can use the git webhooks to do other automations when you change the compose file contents. Redeploying will be as easy as just `git push`.

:::info
Many Monitor resources need access to git repos. There is an in-built token management system (managed in UI or in config file) to give resources access to credentials.
All resources which depend on git repos are able to use these credentials to access private repos.
:::

## Define the Environment


## Deploy Stacks
