# select a builder

A builder is a machine running monitor periphery and docker. Any server connected to monitor can be chosen as the builder for a build.

Building on a machine running production software is usually not a great idea, as this process can use a lot of system resources. It is better to start up a temporary cloud machine dedicated for the build, then shut it down when the build is finished. Right now monitor supports AWS ec2 for this task.

### AWS builder

You can choose to build on AWS on the "builder" tab on the build's page. From here you can select preconfigured AMIs to use as a base to build the image. These must be configured in the monitor core configuration along with other information like defaults to use, AWS credentials, etc. This is explained on the [core setup page](https://github.com/mbecker20/monitor/blob/main/docs/setup.md). 