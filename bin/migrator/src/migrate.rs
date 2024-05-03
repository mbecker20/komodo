use anyhow::Context;
use mongo_indexed::Indexed;
use monitor_client::entities::{
  build::Build,
  deployment::{Deployment, DeploymentConfig},
  server::Server,
  update::{ResourceTarget, Update},
  user::User,
};
use mungos::find::find_collect;

use crate::{
  legacy::{self, v0},
  state::State,
};

impl State {
  pub async fn migrate_all(&self) -> anyhow::Result<()> {
    self.migrate_users().await?;
    self.migrate_servers().await?;
    self.migrate_deployments().await?;
    self.migrate_builds().await?;
    self.migrate_updates().await?;
    Ok(())
  }

  pub async fn migrate_users(&self) -> anyhow::Result<()> {
    let users = find_collect(&self.legacy.users, None, None)
      .await
      .context("failed to get legacy users")?
      .into_iter()
      .map(|s| {
        let context =
          format!("failed to convert user {}", s.username);
        s.try_into().context(context)
      })
      .collect::<anyhow::Result<Vec<User>>>()?;

    info!("migrating {} users...", users.len());

    self
      .target
      .users
      .insert_many(users, None)
      .await
      .context("failed to insert users on target")?;

    info!("users have been migrated\n");

    Ok(())
  }

  pub async fn migrate_servers(&self) -> anyhow::Result<()> {
    let servers = find_collect(&self.legacy.servers, None, None)
      .await
      .context("failed to get legacy servers")?
      .into_iter()
      .map(|s| {
        let context = format!("failed to convert server {}", s.name);
        s.try_into().context(context)
      })
      .collect::<anyhow::Result<Vec<Server>>>()?;

    info!("migrating {} servers...", servers.len());

    self
      .target
      .servers
      .insert_many(servers, None)
      .await
      .context("failed to insert servers on target")?;

    info!("servers have been migrated\n");

    Ok(())
  }

  pub async fn migrate_deployments(&self) -> anyhow::Result<()> {
    let deployments =
      find_collect(&self.legacy.deployments, None, None)
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

    self
      .target
      .deployments
      .insert_many(deployments, None)
      .await
      .context("failed to insert deployments on target")?;

    info!("deployments have been migrated\n");

    Ok(())
  }

  pub async fn migrate_builds(&self) -> anyhow::Result<()> {
    let builds = find_collect(&self.legacy.builds, None, None)
      .await
      .context("failed to get legacy builds")?
      .into_iter()
      .map(|s| {
        let context = format!("failed to convert build {}", s.name);
        s.try_into().context(context)
      })
      .collect::<anyhow::Result<Vec<Build>>>()?;

    info!("migrating {} builds...", builds.len());

    self
      .target
      .builds
      .insert_many(builds, None)
      .await
      .context("failed to insert builds on target")?;

    info!("builds have been migrated\n");

    Ok(())
  }

  pub async fn migrate_updates(&self) -> anyhow::Result<()> {
    let updates = find_collect(&self.legacy.updates, None, None)
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

    self
      .target
      .updates
      .insert_many(updates, None)
      .await
      .context("failed to insert updates on target")?;

    info!("updates have been migrated\n");

    Ok(())
  }
}
