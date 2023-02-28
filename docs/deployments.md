# deploying applications

Monitor can deploy any docker images that it can access with the configured docker accounts. It works by parsing the deployment configuration into a ```docker run``` command. The configuration is stored on MongoDB, and records of all actions (update config, deploy, stop, etc.) are stored as well.

## configuring the image

There are two options to configure the deployed image. 

### attaching a monitor build
If the software you want to deploy if built by monitor, you can attach the build directly to the deployment.

By default, monitor will deploy the latest available version of the build, or you can specify a specific version using the version dropdown.

Also by default, monitor will use the same docker account that is attached to the build in order to pull the image on the periphery server. If that account is not available on the server, you can specify another available account to use instead, this account just needs to have read access to the docker repository.

### using a custom image
You can also manually specify an image name, like ```mongo``` or ```mbecker2020/random_image:0.1.1```.

If the image repository is private, you can select an available docker account to use to pull the image.

## configuring the network

One feature of docker is that it allows for the creation of [virtual networks between containers](https://docs.docker.com/network/). Monitor allows you to specify a docker virtual network to connect the container to, or to use the host system networking to bypass the docker virtual network.

The default selection is ```host```, which bypasses the docker virtual network layer.

## configuring restart behavior

Docker, like systemd, has a couple options for handling when a container exits. See [docker restart policies](https://docs.docker.com/config/containers/start-containers-automatically/). Monitor allows you to select the appropriate restart behavior from these options.

## configuring environment variables

Monitor enables you to easily manage environment variables passed to the container. In the GUI, click the 'edit' button on the 'environment' card, this will bring up the environment menu.

You pass environment variables just as you would with a ```.env``` file:

```
ENV_VAR_1=some_value
ENV_VAR_2=some_other_value
```

##

[next: permissions](https://github.com/mbecker20/monitor/blob/main/docs/permissions.md)

[back to table of contents](https://github.com/mbecker20/monitor/blob/main/readme.md)