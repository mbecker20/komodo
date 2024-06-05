use anyhow::Context;
use monitor_client::entities::{
  build::Build,
  deployment::{
    Deployment, DeploymentConfig, DeploymentConfigDiff,
    DeploymentImage, DeploymentListItem, DeploymentListItemInfo,
    DeploymentQuerySpecifics, DeploymentState,
    PartialDeploymentConfig,
  },
  permission::PermissionLevel,
  resource::Resource,
  server::Server,
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::Collection;
use periphery_client::api::container::RemoveContainer;
use serror::serialize_error_pretty;

use crate::{
  helpers::{
    empty_or_only_spaces, periphery_client,
    query::get_deployment_state,
  },
  monitor::update_cache_for_server,
  resource,
  state::{action_states, db_client, deployment_status_cache},
};

use super::get_check_permissions;

impl super::MonitorResource for Deployment {
  type Config = DeploymentConfig;
  type PartialConfig = PartialDeploymentConfig;
  type ConfigDiff = DeploymentConfigDiff;
  type Info = ();
  type ListItem = DeploymentListItem;
  type QuerySpecifics = DeploymentQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Deployment
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.deployments
  }

  async fn to_list_item(
    deployment: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    let status = deployment_status_cache().get(&deployment.id).await;
    let (build_image, build_id) = match deployment.config.image {
      DeploymentImage::Build { build_id, version } => {
        let (build_name, build_id, build_version) =
          super::get::<Build>(&build_id)
            .await
            .map(|b| (b.name, b.id, b.config.version))
            .unwrap_or((
              String::from("unknown"),
              String::new(),
              Default::default(),
            ));
        let version = if version.is_none() {
          build_version.to_string()
        } else {
          version.to_string()
        };
        (format!("{build_name}:{version}"), Some(build_id))
      }
      DeploymentImage::Image { image } => (image, None),
    };
    DeploymentListItem {
      name: deployment.name,
      id: deployment.id,
      tags: deployment.tags,
      resource_type: ResourceTargetVariant::Deployment,
      info: DeploymentListItemInfo {
        state: status
          .as_ref()
          .map(|s| s.curr.state)
          .unwrap_or_default(),
        status: status.as_ref().and_then(|s| {
          s.curr.container.as_ref().and_then(|c| c.status.to_owned())
        }),
        image: status
          .as_ref()
          .and_then(|s| {
            s.curr.container.as_ref().map(|c| c.image.clone())
          })
          .unwrap_or(build_image),
        server_id: deployment.config.server_id,
        build_id,
      },
    }
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .deployment
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateDeployment
  }

  fn user_can_create(_user: &User) -> bool {
    true
  }

  async fn validate_create_config(
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_create(
    created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    if !created.config.server_id.is_empty() {
      let server =
        resource::get::<Server>(&created.config.server_id).await?;
      update_cache_for_server(&server).await;
    }
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateDeployment
  }

  async fn validate_update_config(
    _id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user).await
  }

  async fn post_update(
    updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    if !updated.config.server_id.is_empty() {
      let server =
        resource::get::<Server>(&updated.config.server_id).await?;
      update_cache_for_server(&server).await;
    }
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteDeployment
  }

  async fn pre_delete(
    deployment: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    let state = get_deployment_state(deployment)
      .await
      .context("failed to get container state")?;
    if !matches!(
      state,
      DeploymentState::NotDeployed | DeploymentState::Unknown
    ) {
      // container needs to be destroyed
      let server = match super::get::<Server>(
        &deployment.config.server_id,
      )
      .await
      {
        Ok(server) => server,
        Err(e) => {
          update.push_error_log(
            "remove container",
            format!(
              "failed to retrieve server at {} from db.\n\nerror: {}",
              deployment.config.server_id,
              serialize_error_pretty(&e)
            ),
          );
          return Ok(());
        }
      };
      if !server.config.enabled {
        // Don't need to
        update.push_simple_log(
          "remove container",
          "skipping container removal, server is disabled.",
        );
        return Ok(());
      }
      let periphery = match periphery_client(&server) {
        Ok(periphery) => periphery,
        Err(e) => {
          // This case won't ever happen, as periphery_client only fallible if the server is disabled.
          // Leaving it for completeness sake
          update.push_error_log(
            "remove container",
            format!(
              "failed to remove container on periphery.\n\nerror: {}",
              serialize_error_pretty(&e),
            ),
          );
          return Ok(());
        }
      };
      match periphery
        .request(RemoveContainer {
          name: deployment.name.clone(),
          signal: deployment.config.termination_signal.into(),
          time: deployment.config.termination_timeout.into(),
        })
        .await
      {
        Ok(log) => update.logs.push(log),
        Err(e) => update.push_error_log(
          "remove container",
          format!(
            "failed to remove container.\n\nerror: {}",
            serialize_error_pretty(&e)
          ),
        ),
      };
    }
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
  config: &mut PartialDeploymentConfig,
  user: &User,
) -> anyhow::Result<()> {
  if let Some(server_id) = &config.server_id {
    if !server_id.is_empty() {
      let server = get_check_permissions::<Server>(server_id, user, PermissionLevel::Write)
          .await
          .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
      config.server_id = Some(server.id);
    }
  }
  if let Some(DeploymentImage::Build { build_id, version }) =
    &config.image
  {
    if !build_id.is_empty() {
      let build = get_check_permissions::<Build>(build_id, user, PermissionLevel::Read)
          .await
          .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
      config.image = Some(DeploymentImage::Build {
        build_id: build.id,
        version: version.clone(),
      });
    }
  }
  if let Some(volumes) = &mut config.volumes {
    volumes.retain(|v| {
      !empty_or_only_spaces(&v.local)
        && !empty_or_only_spaces(&v.container)
    })
  }
  if let Some(ports) = &mut config.ports {
    ports.retain(|v| {
      !empty_or_only_spaces(&v.local)
        && !empty_or_only_spaces(&v.container)
    })
  }
  if let Some(environment) = &mut config.environment {
    environment.retain(|v| {
      !empty_or_only_spaces(&v.variable)
        && !empty_or_only_spaces(&v.value)
    })
  }
  if let Some(extra_args) = &mut config.extra_args {
    extra_args.retain(|v| !empty_or_only_spaces(v))
  }
  Ok(())
}
