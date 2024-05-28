# Select a builder

A builder is a machine running monitor periphery and docker. Any server connected to monitor can be chosen as the builder for a build.

Building on a machine running production software is usually not a great idea, as this process can use a lot of system resources. It is better to start up a temporary cloud machine dedicated for the build, then shut it down when the build is finished. Right now monitor supports AWS ec2 for this task.

### AWS builder

Builders are now monitor resources, and are managed via the core API / can be updated using the UI.
To use this feature, you need an AWS Ec2 AMI with docker and monitor periphery configured to run on system start.
Once you create your builder and add the necessary configuration, it will be available to attach to builds.