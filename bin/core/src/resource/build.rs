use anyhow::Context;
use monitor_client::entities::{
  build::{
    Build, BuildConfig, BuildConfigDiff, BuildInfo, BuildListItem,
    BuildListItemInfo, BuildQuerySpecifics, PartialBuildConfig,
  },
  builder::Builder,
  permission::PermissionLevel,
  resource::Resource,
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::Collection;

use crate::{
  helpers::empty_or_only_spaces,
  state::{action_states, db_client},
};

impl super::MonitorResource for Build {
  type Config = BuildConfig;
  type PartialConfig = PartialBuildConfig;
  type ConfigDiff = BuildConfigDiff;
  type Info = BuildInfo;
  type ListItem = BuildListItem;
  type QuerySpecifics = BuildQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Build
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.builds
  }

  async fn to_list_item(
    build: Resource<Self::Config, Self::Info>,
  ) -> anyhow::Result<Self::ListItem> {
    Ok(BuildListItem {
      name: build.name,
      id: build.id,
      tags: build.tags,
      resource_type: ResourceTargetVariant::Build,
      info: BuildListItemInfo {
        last_built_at: build.info.last_built_at,
        version: build.config.version,
        repo: build.config.repo,
        branch: build.config.branch,
      },
    })
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .build
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateBuild
  }

  fn user_can_create(user: &User) -> bool {
    user.admin || user.create_build_permissions
  }

  async fn validate_create_config(
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateBuild
  }

  async fn validate_update_config(
    _id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteBuild
  }

  async fn pre_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}

#[instrument(skip(user))]
async fn validate_config(
  config: &mut PartialBuildConfig,
  user: &User,
) -> anyhow::Result<()> {
  if let Some(builder_id) = &config.builder_id {
    let builder = super::get_check_permissions::<Builder>(builder_id, user, PermissionLevel::Read)
      .await
      .context("cannot create build using this builder. user must have at least read permissions on the builder.")?;
    config.builder_id = Some(builder.id)
  }
  if let Some(build_args) = &mut config.build_args {
    build_args.retain(|v| {
      !empty_or_only_spaces(&v.variable)
        && !empty_or_only_spaces(&v.value)
    })
  }
  if let Some(extra_args) = &mut config.extra_args {
    extra_args.retain(|v| !empty_or_only_spaces(v))
  }
  Ok(())
}
