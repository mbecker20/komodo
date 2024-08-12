# Resources

Monitor is extendible through the **Resource** abstraction. Entities like `Server`, `Deployment`, and `Stack` are all **Monitor Resources**.

All resources have common traits, such as a unique `name` and `id` amongst all other resources of the same resource type.
All resources can be assigned `tags`, which can be used to group related resources.

:::note
Many resources need access to git repos / docker registries. There is an in-built token management system (managed in UI or in config file) to give resources access to credentials.
All resources which depend on git repos / docker registries are able to use these credentials to access private repos.
:::

## Server

-- Configure the connection to periphery agents.<br></br>
-- Set alerting thresholds.<br></br>
-- Can be attached to **Deployments**, **Stacks**, **Repos**, and **Builders**.

## Deployment

-- Deploy a docker container on the attached Server.<br></br>
-- Manage services at the container level, perform orchestration using **Procedures** and **ResourceSyncs**.

## Stack

-- Deploy with docker compose.<br></br>
-- Provide the compose file in UI, or move the files to a git repo and use a webhook for auto redeploy on push.<br></br>
-- Supports composing multiple compose files using `docker compose -f ... -f ...`.<br></br>
-- Pass environment variables usable within the compose file. Interpolate in app-wide variables / secrets.

## Repo

-- Put scripts in git repos, and run them on a Server, or using a Builder.<br></br>
-- Can build binaries, perform automation, really whatever you can think of.

## Build

-- Build application source into docker images, and push them to the configured registry.<br></br>
-- The source can be any git repo containing a Dockerfile.

## Builder

-- Either points to a connected server, or holds configuration to launch a single-use AWS instance to build the image.<br></br>
-- Can be attached to **Builds** and **Repos**.

## Procedure

-- Compose many actions on other resource type, like `RunBuild` or `DeployStack`, and run it on button push (or with a webhook).<br></br>
-- Can run one or more actions in parallel "stages", and compose a series of parallel stages to run sequentially.

## ResourceSync

-- Orchestrate all your configuration declaratively by defining it in `toml` files, which are checked into a git repo.<br></br>
-- Can deploy **Deployments** and **Stacks** if changes are suggested.<br></br>
-- Specify deploy ordering with `after` array. (like docker compose `depends_on` but can span across servers.).

## Alerter

-- Route alerts to various endpoints.<br></br>
-- Can configure rules on each Alerter, such as resource whitelist, blacklist, or alert type filter.

## ServerTemplate

-- Easily expand your cloud network by storing cloud server lauch templates on various providers.<br></br>
-- Auto connect the server to monitor on launch, using `User Data` launch scripts.
