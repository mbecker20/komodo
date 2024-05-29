use anyhow::Context;
use monitor_client::entities::{
  build::Build, deployment::Deployment, server::Server,
  update::Update, user::User,
};
use mungos::find::find_collect;

use crate::legacy::v0;

pub async fn migrate_all(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  migrate_users(legacy_db, target_db).await?;
  migrate_servers(legacy_db, target_db).await?;
  migrate_deployments(legacy_db, target_db).await?;
  migrate_builds(legacy_db, target_db).await?;
  migrate_updates(legacy_db, target_db).await?;
  Ok(())
}

#[allow(unused)]
pub async fn migrate_users(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let users = find_collect(&legacy_db.users, None, None)
    .await
    .context("failed to get legacy users")?
    .into_iter()
    .map(|s| {
      let context = format!("failed to convert user {}", s.username);
      s.try_into().context(context)
    })
    .collect::<anyhow::Result<Vec<User>>>()?;

  info!("migrating {} users...", users.len());

  target_db
    .users
    .insert_many(users, None)
    .await
    .context("failed to insert users on target")?;

  info!("users have been migrated\n");

  Ok(())
}

pub async fn migrate_servers(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let servers = find_collect(&legacy_db.servers, None, None)
    .await
    .context("failed to get legacy servers")?
    .into_iter()
    .map(|s| {
      let context = format!("failed to convert server {}", s.name);
      s.try_into().context(context)
    })
    .collect::<anyhow::Result<Vec<Server>>>()?;

  info!("migrating {} servers...", servers.len());

  target_db
    .servers
    .insert_many(servers, None)
    .await
    .context("failed to insert servers on target")?;

  info!("servers have been migrated\n");

  Ok(())
}

pub async fn migrate_deployments(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let deployments = find_collect(&legacy_db.deployments, None, None)
    .await
    .context("failed to get legacy deployments")?
    .into_iter()
    .map(|s| {
      let context =
        format!("failed to convert deployment {}", s.name);
      s.try_into().context(context)
    })
    .collect::<anyhow::Result<Vec<Deployment>>>()?;

  info!("migrating {} deployments...", deployments.len());

  target_db
    .deployments
    .insert_many(deployments, None)
    .await
    .context("failed to insert deployments on target")?;

  info!("deployments have been migrated\n");

  Ok(())
}

pub async fn migrate_builds(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let builds = find_collect(&legacy_db.builds, None, None)
    .await
    .context("failed to get legacy builds")?
    .into_iter()
    .map(|s| {
      let context = format!("failed to convert build {}", s.name);
      s.try_into().context(context)
    })
    .collect::<anyhow::Result<Vec<Build>>>()?;

  info!("migrating {} builds...", builds.len());

  target_db
    .builds
    .insert_many(builds, None)
    .await
    .context("failed to insert builds on target")?;

  info!("builds have been migrated\n");

  Ok(())
}

#[allow(unused)]
pub async fn migrate_updates(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let updates = find_collect(&legacy_db.updates, None, None)
    .await
    .context("failed to get legacy updates")?
    .into_iter()
    .map(|s| {
      let context =
        format!("failed to convert update | _id {}", s.id);
      s.try_into().context(context)
    })
    .collect::<anyhow::Result<Vec<Update>>>()?;

  info!("migrating {} updates...", updates.len());

  target_db
    .updates
    .insert_many(updates, None)
    .await
    .context("failed to insert updates on target")?;

  info!("updates have been migrated\n");

  Ok(())
}
