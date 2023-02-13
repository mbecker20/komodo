# building images

Monitor builds docker images by cloning the source repository from Github and running ```docker build``` on the configured Dockerfile, which should be present in the source repository.

## repo configuration
Setting related to the github repo are under the *repo* tab on respective build's page.

To specify the github repo to build, just give it the name of the repo and the branch under *github config*. The name is given like ```mbecker20/monitor```, it includes the username / organization that owns the repo.

Many repos are private, in this case a Github access token is required in the periphery.config.toml of the building server. these are specified in the config like ```username = "access_token"```. An account which has access to the repo and is available on the periphery server can be selected to use via the *github account* dropdown menu.

Sometimes a command needs to be run when the repo is cloned, you can configure this in the *on clone* section. 

There are two fields to pass for *on clone*. the first is *path*, which changes to working directory. To run the command in the root of the repo, just pass ".". The second field is *command*, this is the shell command to be executed after the repo is cloned.

For example, say your repo had a folder in it called "scripts" with a shell script "on-clone.sh". You would give *path* as "scripts" and command as "sh on-clone.sh". Or you could make *path* just "." and then command would be "sh scripts/on-clone.sh". Either way works fine. 

## build configuration


## versioning

Monitor uses a major.minor.patch versioning scheme. Every build will auto increment the patch number, and push the image to docker hub with the version tag as well as the "latest" tag. 


[next: deploying](https://github.com/mbecker20/monitor/blob/main/docs/deployments.md)

[back to table of contents](https://github.com/mbecker20/monitor/blob/main/readme.md)