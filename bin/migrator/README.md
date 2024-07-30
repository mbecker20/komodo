# Migrator

Performs schema changes on the Monitor database

## v1.7 - v1.11 migration (Run this to before upgrading to latest from version 1.7 to 1.11)

```sh
docker run --name monitor-migrator \
	--env MIGRATION="v1.11" \
	--env TARGET_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env TARGET_DB_NAME="<DB_NAME>" \
	ghcr.io/mbecker20/monitor_migrator
```