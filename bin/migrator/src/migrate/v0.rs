use anyhow::Context;
use monitor_client::entities::{
  build::Build,
  deployment::Deployment,
  permission::{Permission, UserTarget},
  server::Server,
  update::{ResourceTarget, Update},
  user::User,
};
use mungos::{
  find::find_collect, mongodb::options::InsertManyOptions,
};

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
  let existing = find_collect(&target_db.users, None, None)
    .await
    .context("failed to get existing target users")?;

  let users = find_collect(&legacy_db.users, None, None)
    .await
    .context("failed to get legacy users")?
    .into_iter()
    .filter_map(|user| {
      if existing.iter().any(|u| u.username == user.username) {
        return None;
      }

      let username = user.username.clone();
      user
        .try_into()
        .inspect_err(|e| {
          warn!("failed to convert user {username} | {e:#}")
        })
        .ok()
    })
    .collect::<Vec<User>>();

  info!("migrating {} users...", users.len());

  target_db
    .users
    .insert_many(users)
    .await
    .context("failed to insert users on target")?;

  info!("users have been migrated\n");

  Ok(())
}

pub async fn migrate_servers(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let existing = find_collect(&target_db.servers, None, None)
    .await
    .context("failed to get existing target servers")?;

  let servers = find_collect(&legacy_db.servers, None, None)
    .await
    .context("failed to get legacy servers")?;

  let mut new_servers = Vec::<Server>::new();
  let mut permissions = Vec::<Permission>::new();

  for server in servers {
    if existing.iter().any(|s| s.name == server.name) {
      continue;
    }

    for (user_id, level) in &server.permissions {
      let permission = Permission {
        id: Default::default(),
        user_target: UserTarget::User(user_id.clone()),
        resource_target: ResourceTarget::Server(server.id.clone()),
        level: (*level).into(),
      };
      permissions.push(permission);
    }
    let name = server.name.clone();
    server
      .try_into()
      .inspect_err(|e| {
        warn!("failed to convert server {name} | {e:#}")
      })
      .map(|s| new_servers.push(s))
      .ok();
  }

  info!("migrating {} servers...", new_servers.len());

  if !new_servers.is_empty() {
    target_db
      .servers
      .insert_many(new_servers)
      .with_options(
        InsertManyOptions::builder().ordered(false).build(),
      )
      .await
      .context("failed to insert servers on target")?;
  }

  if !permissions.is_empty() {
    target_db
      .permissions
      .insert_many(permissions)
      .with_options(
        InsertManyOptions::builder().ordered(false).build(),
      )
      .await
      .context("failed to insert server permissions on target")?;
  }

  info!("servers have been migrated\n");

  Ok(())
}

pub async fn migrate_deployments(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let existing = find_collect(&target_db.deployments, None, None)
    .await
    .context("failed to get existing target deployments")?;

  let deployments = find_collect(&legacy_db.deployments, None, None)
    .await
    .context("failed to get legacy deployments")?;

  let mut new_deployments = Vec::<Deployment>::new();
  let mut permissions = Vec::<Permission>::new();

  for deployment in deployments {
    if existing.iter().any(|d| d.name == deployment.name) {
      continue;
    }

    for (user_id, level) in &deployment.permissions {
      let permission = Permission {
        id: Default::default(),
        user_target: UserTarget::User(user_id.clone()),
        resource_target: ResourceTarget::Deployment(
          deployment.id.clone(),
        ),
        level: (*level).into(),
      };
      permissions.push(permission);
    }
    let name = deployment.name.clone();
    deployment
      .try_into()
      .inspect_err(|e| {
        warn!("failed to convert deployment {name} | {e:#}")
      })
      .map(|s| new_deployments.push(s))
      .ok();
  }

  info!("migrating {} deployments...", new_deployments.len());

  if !new_deployments.is_empty() {
    target_db
      .deployments
      .insert_many(new_deployments)
      .with_options(
        InsertManyOptions::builder().ordered(false).build(),
      )
      .await
      .context("failed to insert deployments on target")?;
  }

  if !permissions.is_empty() {
    target_db
      .permissions
      .insert_many(permissions)
      .with_options(
        InsertManyOptions::builder().ordered(false).build(),
      )
      .await
      .context("failed to insert deployment permissions on target")?;
  }

  info!("deployments have been migrated\n");

  Ok(())
}

pub async fn migrate_builds(
  legacy_db: &v0::DbClient,
  target_db: &crate::DbClient,
) -> anyhow::Result<()> {
  let existing = find_collect(&target_db.builds, None, None)
    .await
    .context("failed to get existing target builds")?;

  let builds = find_collect(&legacy_db.builds, None, None)
    .await
    .context("failed to get legacy builds")?;

  let mut new_builds = Vec::<Build>::new();
  let mut permissions = Vec::<Permission>::new();

  for build in builds {
    if existing.iter().any(|b| b.name == build.name) {
      continue;
    }

    for (user_id, level) in &build.permissions {
      let permission = Permission {
        id: Default::default(),
        user_target: UserTarget::User(user_id.clone()),
        resource_target: ResourceTarget::Build(build.id.clone()),
        level: (*level).into(),
      };
      permissions.push(permission);
    }
    let name = build.name.clone();
    build
      .try_into()
      .inspect_err(|e| {
        warn!("failed to convert build {name} | {e:#}")
      })
      .map(|s| new_builds.push(s))
      .ok();
  }

  info!("migrating {} builds...", new_builds.len());

  if !new_builds.is_empty() {
    target_db
      .builds
      .insert_many(new_builds)
      .with_options(
        InsertManyOptions::builder().ordered(false).build(),
      )
      .await
      .inspect_err(|e| {
        warn!("failed to insert builds on target | {e}")
      })
      .ok();
  }

  if !permissions.is_empty() {
    target_db
      .permissions
      .insert_many(permissions)
      .with_options(
        InsertManyOptions::builder().ordered(false).build(),
      )
      .await
      .inspect_err(|e| {
        warn!("failed to insert build permissions on target | {e}")
      })
      .ok();
  }

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
    .insert_many(updates)
    .with_options(InsertManyOptions::builder().ordered(false).build())
    .await
    .context("failed to insert updates on target")?;

  info!("updates have been migrated\n");

  Ok(())
}
