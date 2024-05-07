use std::str::FromStr;

use anyhow::anyhow;
use monitor_client::{
  api::execute::Execution,
  entities::{
    build::Build,
    deployment::Deployment,
    permission::PermissionLevel,
    procedure::{
      PartialProcedureConfig, Procedure, ProcedureConfig,
      ProcedureListItem, ProcedureListItemInfo,
      ProcedureQuerySpecifics,
    },
    repo::Repo,
    resource::Resource,
    server::Server,
    update::{ResourceTargetVariant, Update},
    user::User,
    Operation,
  },
};
use mungos::mongodb::{bson::oid::ObjectId, Collection};

use crate::state::{action_states, db_client};

impl super::MonitorResource for Procedure {
  type Config = ProcedureConfig;
  type PartialConfig = PartialProcedureConfig;
  type Info = ();
  type ListItem = ProcedureListItem;
  type QuerySpecifics = ProcedureQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Procedure
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.procedures
  }

  async fn to_list_item(
    procedure: Resource<Self::Config, Self::Info>,
  ) -> anyhow::Result<Self::ListItem> {
    Ok(ProcedureListItem {
      name: procedure.name,
      created_at: ObjectId::from_str(&procedure.id)?
        .timestamp()
        .timestamp_millis(),
      id: procedure.id,
      tags: procedure.tags,
      resource_type: ResourceTargetVariant::Procedure,
      info: ProcedureListItemInfo {
        procedure_type: procedure.config.procedure_type,
      },
    })
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .procedure
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateProcedure
  }

  fn user_can_create(_user: &User) -> bool {
    true
  }

  async fn validate_create_config(
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user, None).await
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateProcedure
  }

  async fn validate_update_config(
    original: Resource<Self::Config, Self::Info>,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user, Some(&original.id)).await
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteProcedure
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
  config: &mut PartialProcedureConfig,
  user: &User,
  id: Option<&str>,
) -> anyhow::Result<()> {
  let Some(executions) = &mut config.executions else {
    return Ok(());
  };
  for exec in executions {
    match &mut exec.execution {
      Execution::None(_) => {}
      Execution::RunProcedure(params) => {
        let procedure = super::get_check_permissions::<Procedure>(
          &params.procedure,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        match id {
          Some(id) if procedure.id == id => {
            return Err(anyhow!(
              "Cannot have self-referential procedure"
            ))
          }
          _ => {}
        }
        params.procedure = procedure.id;
      }
      Execution::RunBuild(params) => {
        let build = super::get_check_permissions::<Build>(
          &params.build,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.build = build.id;
      }
      Execution::Deploy(params) => {
        let deployment = super::get_check_permissions::<Deployment>(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::StartContainer(params) => {
        let deployment = super::get_check_permissions::<Deployment>(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::StopContainer(params) => {
        let deployment = super::get_check_permissions::<Deployment>(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::StopAllContainers(params) => {
        let server = super::get_check_permissions::<Server>(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
      Execution::RemoveContainer(params) => {
        let deployment = super::get_check_permissions::<Deployment>(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::CloneRepo(params) => {
        let repo = super::get_check_permissions::<Repo>(
          &params.repo,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.repo = repo.id;
      }
      Execution::PullRepo(params) => {
        let repo = super::get_check_permissions::<Repo>(
          &params.repo,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.repo = repo.id;
      }
      Execution::PruneDockerNetworks(params) => {
        let server = super::get_check_permissions::<Server>(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
      Execution::PruneDockerImages(params) => {
        let server = super::get_check_permissions::<Server>(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
      Execution::PruneDockerContainers(params) => {
        let server = super::get_check_permissions::<Server>(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
    }
  }
  Ok(())
}
