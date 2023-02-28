# deploying applications

Monitor can deploy any docker images that it can access with the configured docker accounts. It works by parsing the deployment configuration into a ```docker run``` command. The configuration is stored on MongoDB, and records of all actions (update config, deploy, stop, etc.) are stored as well.

## configuring the image

There are two options to configure the deployed image. 

### attaching a monitor build
If the software you want to deploy is built by monitor, you can attach the build directly to the deployment.

By default, monitor will deploy the latest available version of the build, or you can specify a specific version using the version dropdown.

Also by default, monitor will use the same docker account that is attached to the build in order to pull the image on the periphery server. If that account is not available on the server, you can specify another available account to use instead, this account just needs to have read access to the docker repository.

### using a custom image
You can also manually specify an image name, like ```mongo``` or ```mbecker2020/random_image:0.1.1```.

If the image repository is private, you can select an available docker account to use to pull the image.

## configuring the network

One feature of docker is that it allows for the creation of [virtual networks between containers](https://docs.docker.com/network/). Monitor allows you to specify a docker virtual network to connect the container to, or to use the host system networking to bypass the docker virtual network.

The default selection is ```host```, which bypasses the docker virtual network layer.

If you do select select a network other than host, you can specify port bindings with the GUI. For example, if you are running mongo (which defaults to port 27017), you could use the mapping:

```
27018 : 27017
```

In this case, you would access mongo from outside of the container on port ```27018```.

Note that this is not the only affect of using a network other than ```host```. For example, containers running on different networks can not communicate, and ones on the same network can not reach other containers on ```localhost``` even when they are running on the same system. This behavior can be a bit confusing if you are not familiar with it, and it can be bypassed entirely by just using ```host``` network.

## configuring restart behavior

Docker, like systemd, has a couple options for handling when a container exits. See [docker restart policies](https://docs.docker.com/config/containers/start-containers-automatically/). Monitor allows you to select the appropriate restart behavior from these options.

## configuring environment variables

Monitor enables you to easily manage environment variables passed to the container. In the GUI, click the 'edit' button on the 'environment' card, this will bring up the environment menu.

You pass environment variables just as you would with a ```.env``` file:

```
ENV_VAR_1=some_value
ENV_VAR_2=some_other_value
```

## configuring volumes

A docker container's filesystem is segregated from that of the host. However, it is still possible for a container to access system files and directories, this is accomplished by using [bind mounts](https://docs.docker.com/storage/bind-mounts/).

Say your container needs to read a config file located on the system at ```/home/ubuntu/config.toml```. You can specify the bind mount to be:

```
/home/ubuntu/config.toml : /config/config.toml
```

The first path is the one on the system, the second is the path in the container. Your application would then read the file at ```/config/config.toml``` in order to load its contents.

These can be configured easily with the GUI in the 'volumes' card. You can configure as many bind mounts as you need.

## extra args

Not all features of docker are mapped directly by monitor, only the most common. You can still specify any custom flags for monitor to include in the ```docker run``` command by utilizing 'extra args'. For example, you can enable log rotation using these two extra args:

```
--log-opt max-size=10M
```
```
--log-opt max-file=3
```

## post image

Sometimes you need to specify some flags to be passed directly to the application. What is put here is inserted into the docker run command after the image. For example, to pass the ```--quiet``` flag to MongoDB, the docker run command would be:

```
docker run -d --name mongo-db mongo:6.0.3 --quiet
```

In order to achieve this with monitor, just pass ```--quiet``` to 'post image'.

## container lifetime management

The lifetime of a docker container is more like a virtual machine. They can be created, started, stopped, and destroyed. The lifetime management actions monitor presents to the user is relative to the containers state. For example, when the container is ```running```, you can either stop it, destroy it, or redeploy it.

### stopping a container

Sometimes you want to stop a running application but preserve its logs and configuration, either to be restarted later or to view the logs at a later time. It is more like *pausing* the application with its current config, as no configuration (like environment variable, volume mounts, etc.) will be changed when the container is started again. In order to restart an application with updated configuration, it must be *redeployed*.

### container redeploy

redeploying is the action of destroying a container and recreating it. If you update deployment config, these changes will not take effect until the container is redeployed. Just note this will destroy the previous containers logs along with the container itself.

[next: permissions](https://github.com/mbecker20/monitor/blob/main/docs/permissions.md)

[back to table of contents](https://github.com/mbecker20/monitor/blob/main/readme.md)