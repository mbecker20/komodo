# Migrator

Performs schema changes on the Monitor database

## v1.7 - v1.11 migration 
Run this before upgrading to latest from versions 1.7 to 1.11.
```sh
docker run --rm --name monitor-migrator \
	--network "host" \
	--env MIGRATION="v1.11" \
	--env TARGET_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env TARGET_DB_NAME="<DB_NAME>" \
	ghcr.io/mbecker20/monitor_migrator
```

## v1.0 - v1.6 migration
Run this before upgrading to latest from versions 1.0 to 1.6.
```sh
docker run --rm --name monitor-migrator \
	--network "host" \
	--env MIGRATION="v1.6" \
	--env TARGET_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env TARGET_DB_NAME="<DB_NAME>" \
	ghcr.io/mbecker20/monitor_migrator
```