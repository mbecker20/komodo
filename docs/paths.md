# File Paths

When working with monitor, you might have to configure file or directory paths.

## Relative Paths

Where possible, it is better to use relative file paths. Using relative file paths removes the connection between the process being run and the particular server it runs one, making it easier to move things between servers.

Where you see relative paths:

 - setting the build directory and path of the Dockerfile
 - setting a pre build command path
 - configuring a frontend mount (used for web apps)

For all of the above, the path can be given relative to the root of the configured repo

The one exception is the Dockerfile path, which is given relative to the build directory (This is done by Docker itself, and this pattern matches usage of the Docker CLI).

There are 3 kinds of paths to pass:

 1. to specify the root of the repo, use ```.``` as the path
 2. to specify a folder in the repo, pass it with **no** preceding ```/```. For example, ```example_folder``` or ```folder1/folder2```
 3. to specify an absolute path on the servers filesystem, use a preceding slash, eg. ```/home/ubuntu/example/Dockerfile```. This way should only be used if absolutely necessary.

### Implementation

relative file paths are joined with the path of the repo on the system using a Rust [PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push).

## Docker Volume Paths

These are passed directly to the Docker CLI using ```--volume /path/on/system:/path/in/container```. So for these, the same rules apply as when using Docker on the command line. Paths here should be given as absolute, don't use ```~``` or even ```$HOME```.







