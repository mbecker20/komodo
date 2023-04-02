# setup monitor periphery

The easiest way to do this is to follow the [monitor guide](https://github.com/mbecker20/monitor-guide). This is a repo containing directions and scripts enabling command line installation via ssh or remotely.

### manual install steps

 1. Download the periphery binary from the latest [release](https://github.com/mbecker20/monitor/releases).

 2. Create and edit your config files, following the [config example](https://github.com/mbecker20/monitor/blob/main/config_example/periphery.config.example.toml). The monitor cli can be used to add the boilerplate: ```monitor periphery gen-config --path /path/to/config.toml```. The files can be anywhere, and can be passed to periphery via the ```--config-path``` argument.

 3. Ensure that inbound connectivity is allowed on the port specified in periphery.config.toml (default 8000).

 4. Install docker. Make sure whatever user periphery is run as has access to the docker group without sudo.

 5. Start the periphery binary with your preferred process manager, like systemd. The config read from the file is printed on startup, ensure that it is as expected.

### example periphery start command

```
periphery \
	--config-path /path/to/periphery.config.base.toml \
	--config-path /other_path/to/overide-periphery-config-directory \
	--config-keyword periphery \
	--config-keyword config \
	--merge-nested-config \
	--home_dir /home/username
```

### passing config files

Either file paths or directory paths can be passed to ```--config-path```.

When using directories, the file entries can be filtered by name with the ```--config-keyword``` argument, which can be passed multiple times to add more keywords. If passed, then only config files with file names that contain all keywords will be merged.

When passing multiple config files, later --config-path given in the command will always overide previous ones. Directory config files are merged in alphabetical order by name, so ```config_b.toml``` will overide ```config_a.toml```.

There are two ways to merge config files. The default behavior is to completely replace any base fields with whatever fields are present in the overide config. So if you pass ```allowed_ips = []``` in your overide config, the final allowed_ips will be an empty list as well. 

```--merge-nested-config``` will merge config fields recursively and extend config array fields. 

For example, with ```--merge-nested-config``` you can specify an allowed ip in the base config, and another in the overide config, they will both be present in the final config.

Similarly, you can specify a base docker / github account pair, and extend them with additional accounts in the overide config.