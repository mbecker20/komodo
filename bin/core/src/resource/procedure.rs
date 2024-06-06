use std::time::Duration;

use anyhow::{anyhow, Context};
use monitor_client::{
  api::execute::Execution,
  entities::{
    build::Build,
    deployment::Deployment,
    permission::PermissionLevel,
    procedure::{
      PartialProcedureConfig, Procedure, ProcedureConfig,
      ProcedureConfigDiff, ProcedureListItem, ProcedureListItemInfo,
      ProcedureQuerySpecifics, ProcedureState,
    },
    repo::Repo,
    resource::Resource,
    server::Server,
    update::{ResourceTargetVariant, Update},
    user::User,
    Operation,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOneOptions, Collection},
};

use crate::state::{action_states, db_client, procedure_state_cache};

impl super::MonitorResource for Procedure {
  type Config = ProcedureConfig;
  type PartialConfig = PartialProcedureConfig;
  type ConfigDiff = ProcedureConfigDiff;
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
  ) -> Self::ListItem {
    let state = get_procedure_state(&procedure.id).await;
    ProcedureListItem {
      name: procedure.name,
      id: procedure.id,
      tags: procedure.tags,
      resource_type: ResourceTargetVariant::Procedure,
      info: ProcedureListItemInfo {
        stages: procedure.config.stages.len() as i64,
        state,
      },
    }
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
    refresh_procedure_state_cache().await;
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateProcedure
  }

  async fn validate_update_config(
    id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    validate_config(config, user, Some(id)).await
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    refresh_procedure_state_cache().await;
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
  let Some(stages) = &mut config.stages else {
    return Ok(());
  };
  for stage in stages {
    for exec in &mut stage.executions {
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
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::StartContainer(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::StopContainer(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
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
          let deployment =
            super::get_check_permissions::<Deployment>(
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
        Execution::PruneNetworks(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PruneImages(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PruneContainers(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::RunSync(params) => {
          todo!()
        }
      }
    }
  }

  Ok(())
}

pub fn spawn_procedure_state_refresh_loop() {
  tokio::spawn(async move {
    loop {
      refresh_procedure_state_cache().await;
      tokio::time::sleep(Duration::from_secs(60)).await;
    }
  });
}

pub async fn refresh_procedure_state_cache() {
  let _ = async {
    let procedures =
      find_collect(&db_client().await.procedures, None, None)
        .await
        .context("failed to get procedures from db")?;
    let cache = procedure_state_cache();
    for procedure in procedures {
      let state = get_procedure_state_from_db(&procedure.id).await;
      cache.insert(procedure.id, state).await;
    }
    anyhow::Ok(())
  }
  .await
  .inspect_err(|e| {
    error!("failed to refresh build state cache | {e:#}")
  });
}

async fn get_procedure_state(id: &String) -> ProcedureState {
  if action_states()
    .procedure
    .get(id)
    .await
    .map(|s| s.get().map(|s| s.running))
    .transpose()
    .ok()
    .flatten()
    .unwrap_or_default()
  {
    return ProcedureState::Running;
  }
  procedure_state_cache().get(id).await.unwrap_or_default()
}

async fn get_procedure_state_from_db(id: &str) -> ProcedureState {
  async {
    let state = db_client()
      .await
      .updates
      .find_one(
        doc! {
          "target.type": "Procedure",
          "target.id": id,
          "operation": "RunProcedure"
        },
        FindOneOptions::builder()
          .sort(doc! { "start_ts": -1 })
          .build(),
      )
      .await?
      .map(|u| {
        if u.success {
          ProcedureState::Ok
        } else {
          ProcedureState::Failed
        }
      })
      .unwrap_or(ProcedureState::Ok);
    anyhow::Ok(state)
  }
  .await
  .inspect_err(|e| {
    warn!("failed to get procedure state for {id} | {e:#}")
  })
  .unwrap_or(ProcedureState::Unknown)
}
