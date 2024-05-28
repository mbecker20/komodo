# Configuration

Monitor just needs a bit of information in order to build your image.

### Repo configuration
To specify the github repo to build, just give it the name of the repo and the branch under *repo config*. The name is given like ```mbecker20/monitor```, it includes the username / organization that owns the repo.

Many repos are private, in this case a Github access token is needed by the building server.
It can either come from and account defined in the core configuration,
or in the periphery configuration of the building server.
These are specified in the config like `username = "access_token"`.

### Docker build configuration

In order to docker build, monitor just needs to know the build directory and the path of the Dockerfile relative to the repo, you can configure these in the *build config* section.

If the build directory is the root of the repository, you pass the build path as `.`. If the build directory is some folder of the repo, just pass the name of the the folder. Do not pass the preceding "/". for example `build/directory`

The dockerfile's path is given relative to the build directory. So if your build directory is `build/directory` and the dockerfile is in `build/directory/Dockerfile.example`, you give the dockerfile path simply as `Dockerfile.example`.

Just as with private repos, you will need to select a docker account to use with `docker push`. 

### Adding build args

The Dockerfile may make use of [build args](https://docs.docker.com/engine/reference/builder/#arg). Build args can be passed using the gui by navigating to the `Build Args` tab in the config. They are passed in the menu just like in the would in a .env file:

```
BUILD_ARG1=some_value
BUILD_ARG2=some_other_value
```

Note that these values are visible in the final image using `docker history`, so shouldn't be used to pass build time secrets. Use [secret mounts](https://docs.docker.com/engine/reference/builder/#run---mounttypesecret) for this instead.