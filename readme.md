### There are seven main tasks of monitor:
## 1. Manage Github repos
- Configure the repo URL and branch
- Define and run scripts in the root of the repo
- Clone the repo either locally or remotely. Generally, this will be local to the server hosting monitor to be used for building, but some repos need to cloned remotely alongside the actual running containers.
## 2. Build docker images off of the repos
- Define the subdirectory of the repo to build off of
- Manually build images during development and stream the output
- Auto build images during deployment following an auto pull
## 3. Host a docker image registry
- The locally built images are then sent stored in the image registry, which is itself a server
- This is to enable us to remotely pull the latest built images (alone) on our deployment servers
## 4. Start and manage docker containers, either locally or remotely
- Configure the ports, volumes, network, restart options of the run command
- Sometimes we will deploy our own images, sometimes prebuilt ones. We optionally specify an image name if it is prebuilt, otherwise the app looks to see if you have a build on the stack and uses that image. If neither exist on the stack, there is no option to deploy.
- Once containers are running, we need to track their status, and be able to stop, remove, and restart containers. 
## 5. Configure overarching actions involving all three steps
- Once we configure these steps, it will be a pain to manually log on and run them one by one. The solution is to configure actions involving all three steps that can be triggered through a git push. 
## 6. Log all actions performed
- Every stack / server creation, config update, auto pull, manual action, etc is logged into the database
- These logs are sortable by multiple fields, ie by timestamp, relevant stack, server, and type (build, pull, deploy etc)
## 7. Manage available servers
- As our AWS instances change, we need to be able to update the servers we have to work with.
- Duplicate entire server configurations to other instances. If monitor sets everything up on one server, it can copy its config and deploy all the same containers on the new server. (may be complicated when it comes to nodes / syncing though). Should be able to handle EBS volume formatting and mounting steps.

In order to conceptually simplify these steps into cohesive development streams, the app wraps ups the configuration of these into one object: the stack.

    type Stack = {
      name: string
      serverID: string

      /* repo related */
      repoURL?: string
      repoName?: string
      pullName?: string
      branch?: string
      repoScripts?: Script[]
      remoteRepo?: boolean

      /* build related */
      buildPath?: string
      dockerfilePath?: String // relative to buildPath
      buildScripts?: Script[]

      /* deployment related */
      image?: string // only defined if no repoURL associated with stack
      latest?: boolean // if custom image, use this to add :latest
      ports?: Conversion[]
      volumes?: Conversion[]
      network?: string
      restart?: string
      postImage?: string // this is additional commands to be run with the docker run command that come after the image (clearer with example)
      logTail?: number

      operationIDs: string[]

      owner: string
    }

Most of the properties of the stack are optional. This is to make stacks dynamic enough to be able to handle their various use cases. The frontend is able to look at what has been defined and only present actions that are possible given the current config, and the config being updated will reveal additional actions.

## Server Client

This is an app running in a container on our deployment servers that opens an API that allows it to be reached from the main Monitor server. Like the monitor app itself, the native running docker daemon (at /var/run/docker.sock on the base of the deployment servers file system) is passed through to the container, which allows it to see the containers running, start / stop them, have access to logs, overall full access to docker commands. It also opens a route to run commands on the server and return the result.

## Mongo

The database of choice is Mongo, using "mongoose" as a dependency to simplify interactions with the database. You will find a folder of "schema" in backend/src. These are defined similarly to typescript types, like so:

    import { model, Schema } from 'mongoose'
    import { SYSROOT } from '../const'
    import { ScriptSchema } from './misc'

    const ServersSchema = new Schema({
      name: { type: String, unique: true, index: true },
      address: { type: String, unique: true },
      password: String,
      port: { type: String, default: '6060' },
      websocketPort: { type: String, default: '7070' },
      rootDirectory: { type: String, default: SYSROOT },
      stackIDs: [String],
      scripts: [ScriptSchema]
    }, { timestamps: true })

    const ServersManager = model('Servers', ServersSchema)

    export default ServersManager

The export of these schema definitions (in this case ServersManager) are handles through which we can make queries / updates to the database.

    const server1 = await ServersManager.findOne({ name: 'server1' })
    await ServersManager.create({ name: 'server2', ... })


## Setup

The final Monitor deployment includes, as a base, three individual containers running side by side, and four stacks (three of which are associated with the above containers). The stacks / containers are: MongoDB, Image Registry, and Monitor. The one additional stack (Monitor Frontend Repo) is configured as a local repo alone, and includes the built frontend to be served by the monitor backend. This repo folder is mounted into the Monitor container to be served, and since it is external, it can be updated independantly without touching the running Monitor backend.

When Monitor first starts up, it is started directly after docker is installed with no other containers running. The monitor container is configured to launch the "setup.ts" script on launch, which checks whether it has been setup up fully before (whether it's a restart or a full setup). If it is a full setup, it will then start up a mongo container, connect to it, then start up the image registry container, clone the frontend repo, and finally add the initial servers, stacks, and logs of these events to the now running mongo instance database, before starting up the main monitor app (which could not itself start without these other containers / frontend being set up). 

## Websockets

Currently, all commands must finish running before the result is returned in an HTTP response to the frontend. For commands that take a while to run, this is kind of a painful disconnected experience of running commands. Also, container logs will not update by themselves, they have to be re-fetched for every update. I would like to setup up a way for command line output / container log output to be streamed via websockets for a more direct user experience interacting with Monitor. This allows all the data to be streamed over in real time, and the technology will continue to be essential in order to implement live updating ether eye data streams.


