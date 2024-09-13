use anyhow::Context;
use komodo_client::entities::{build::Build, deployment::Deployment};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, to_document},
};

use crate::legacy::v1_11;

pub async fn migrate_all_in_place(
  db: &v1_11::DbClient,
) -> anyhow::Result<()> {
  migrate_builds_in_place(db).await?;
  migrate_deployments_in_place(db).await?;
  Ok(())
}

pub async fn migrate_builds_in_place(
  db: &v1_11::DbClient,
) -> anyhow::Result<()> {
  let builds = find_collect(&db.builds, None, None)
    .await
    .context("failed to get builds")?
    .into_iter()
    .map(Into::into)
    .collect::<Vec<Build>>();

  info!("migrating {} builds...", builds.len());

  for build in builds {
    db.builds
      .update_one(
        doc! { "name": &build.name },
        doc! { "$set": to_document(&build)? },
      )
      .await
      .context("failed to insert builds on target")?;
  }

  info!("builds have been migrated\n");

  Ok(())
}

pub async fn migrate_deployments_in_place(
  db: &v1_11::DbClient,
) -> anyhow::Result<()> {
  let deployments = find_collect(&db.deployments, None, None)
    .await
    .context("failed to get deployments")?
    .into_iter()
    .map(Into::into)
    .collect::<Vec<Deployment>>();

  info!("migrating {} deployments...", deployments.len());

  for deployment in deployments {
    db.deployments
      .update_one(
        doc! { "name": &deployment.name },
        doc! { "$set": to_document(&deployment)? },
      )
      .await
      .context("failed to insert deployments on target")?;
  }

  info!("deployments have been migrated\n");

  Ok(())
}
