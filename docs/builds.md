# building images

Monitor builds docker images by cloning the source repository from Github, running ```docker build```, and pushing the resulting image to docker hub. Any repo containing a 'Dockerfile' is buildable using this method.

## repo configuration
To specify the github repo to build, just give it the name of the repo and the branch under *repo config*. The name is given like ```mbecker20/monitor```, it includes the username / organization that owns the repo.

Many repos are private, in this case a Github access token is required in the periphery.config.toml of the building server. these are specified in the config like ```username = "access_token"```. An account which has access to the repo and is available on the periphery server can be selected to use via the *github account* dropdown menu.

## docker build configuration

In order to docker build, monitor just needs to know the build directory and the path of the Dockerfile, you can configure these in the *build config* section.

If the build directory is the root of the repository, you pass the build path as ```.```. If the build directory is some folder of the repo, just pass the name of the the folder. Do not pass the preceding "/". for example ```build/directory```

The dockerfile's path is given relative to the build directory. So if your build directory is ```build/directory``` and the dockerfile is in ```build/directory/Dockerfile.example```, you give the dockerfile path simply as ```Dockerfile.example```.

Just as with private repos, you will need to select a docker account to use with ```docker push```. 

## running a pre build command

Sometimes a command needs to be run before running ```docker build```, you can configure this in the *pre build* section. 

There are two fields to pass for *pre build*. the first is *path*, which changes to working directory. To run the command in the root of the repo, just pass ```.```. The second field is *command*, this is the shell command to be executed after the repo is cloned.

For example, say your repo had a folder in it called "scripts" with a shell script "on-clone.sh". You would give *path* as "scripts" and command as "sh on-clone.sh". Or you could make *path* just "." and then command would be "sh scripts/on-clone.sh". Either way works fine.

## adding build args

The Dockerfile may make use of [build args](https://docs.docker.com/engine/reference/builder/#arg). Build args can be passed using the gui by pressing the ```edit``` button. They are passed in the menu just like in a .env file:

```
BUILD_ARG1=some_value
BUILD_ARG2=some_other_value
```

## builder configuration

A builder is a machine running monitor periphery and docker. Any server connected to monitor can be chosen as the builder for a build.

Building on a machine running production software is usually not a great idea, as this process can use a lot of the system resources. It is better to start up a temporary cloud machine dedicated for the build, then shut it down when the build is finished. Right now monitor supports AWS ec2 for this task.

### AWS builder

You can choose to build on AWS on the "builder" tab on the build's page. From here you can configure the AMI to use as a base to build the image. These must be configured in the monitor core configuration along with other information like defaults to use, AWS credentials, etc. This is explained on the [core setup page](https://github.com/mbecker20/monitor/blob/main/docs/setup.md). 

## versioning

Monitor uses a major.minor.patch versioning scheme. Every build will auto increment the patch number, and push the image to docker hub with the version tag as well as the "latest" tag. 


[next: deploying](https://github.com/mbecker20/monitor/blob/main/docs/deployments.md)

[back to table of contents](https://github.com/mbecker20/monitor/blob/main/readme.md)
