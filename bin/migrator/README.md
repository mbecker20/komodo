# Migrator

Performs schema changes on the Monitor database

## v1.7 - v1.11 migration 
Run this before upgrading to latest from versions 1.7 to 1.11.
```sh
docker run --rm --name monitor-migrator \
	--env MIGRATION="v1.11" \
	--env TARGET_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env TARGET_DB_NAME="<DB_NAME>" \
	ghcr.io/mbecker20/monitor_migrator
```

## v1.0 - v1.6 migration
Run this before upgrading to latest from versions 1.0 to 1.6.
```sh
docker run --rm --name monitor-migrator \
	--env MIGRATION="v1.6" \
	--env TARGET_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env TARGET_DB_NAME="<DB_NAME>" \
	ghcr.io/mbecker20/monitor_migrator
```

## v0.X migration
Run this before upgrading to latest from version 0.X.

Note. As this is a major upgrade, this migration is not "in place". 
It will create another database (TARGET) and migrate resources over, leaving the original database (LEGACY) unchanged.

```sh
docker run --rm --name monitor-migrator \
	--env MIGRATION="v0" \
	--env TARGET_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env TARGET_DB_NAME="<TARGET_DB_NAME>" \
	--env LEGACY_URI="mongodb://<USERNAME>:<PASSWORD>@<ADDRESS>" \
	--env LEGACY_DB_NAME="<LEGACY_DB_NAME>" \
	ghcr.io/mbecker20/monitor_migrator
```