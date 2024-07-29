use anyhow::Context;
use monitor_client::entities::build::Build;
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, to_document},
};

use crate::legacy::v1_11;

pub async fn migrate_all_in_place(
  db: &v1_11::DbClient,
) -> anyhow::Result<()> {
  migrate_builds_in_place(db).await?;
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
