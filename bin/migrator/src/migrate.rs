use anyhow::Context;
use monitor_types::entities::{
  build::Build,
  deployment::{Deployment, DeploymentConfig},
  server::Server,
  update::{ResourceTarget, Update},
  user::User,
};
use mungos::Indexed;

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
    let users = self
      .legacy
      .users
      .get_some(None, None)
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
      .create_many(users)
      .await
      .context("failed to create users on target")?;

    info!("users have been migrated\n");

    Ok(())
  }

  pub async fn migrate_servers(&self) -> anyhow::Result<()> {
    let servers = self
      .legacy
      .servers
      .get_some(None, None)
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
      .create_many(servers)
      .await
      .context("failed to create servers on target")?;

    info!("servers have been migrated\n");

    Ok(())
  }

  pub async fn migrate_deployments(&self) -> anyhow::Result<()> {
    let deployments = self
      .legacy
      .deployments
      .get_some(None, None)
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
      .create_many(deployments)
      .await
      .context("failed to create deployments on target")?;

    info!("deployments have been migrated\n");

    Ok(())
  }

  pub async fn migrate_builds(&self) -> anyhow::Result<()> {
    let builds = self
      .legacy
      .builds
      .get_some(None, None)
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
      .create_many(builds)
      .await
      .context("failed to create builds on target")?;

    info!("builds have been migrated\n");

    Ok(())
  }

  pub async fn migrate_updates(&self) -> anyhow::Result<()> {
    let updates = self
      .legacy
      .updates
      .get_some(None, None)
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
      .create_many(updates)
      .await
      .context("failed to create updates on target")?;

    info!("updates have been migrated\n");

    Ok(())
  }
}
