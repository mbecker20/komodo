use std::time::Duration;

use anyhow::{anyhow, Context};
use komodo_client::{
  api::execute::Execution,
  entities::{
    action::Action,
    alerter::Alerter,
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
    stack::Stack,
    sync::ResourceSync,
    update::Update,
    user::User,
    Operation, ResourceTargetVariant,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{bson::doc, options::FindOneOptions, Collection},
};

use crate::{
  config::core_config,
  state::{action_states, db_client, procedure_state_cache},
};

impl super::KomodoResource for Procedure {
  type Config = ProcedureConfig;
  type PartialConfig = PartialProcedureConfig;
  type ConfigDiff = ProcedureConfigDiff;
  type Info = ();
  type ListItem = ProcedureListItem;
  type QuerySpecifics = ProcedureQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Procedure
  }

  fn coll() -> &'static Collection<Resource<Self::Config, Self::Info>>
  {
    &db_client().procedures
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

  fn user_can_create(user: &User) -> bool {
    user.admin || !core_config().disable_non_admin_create
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
    updated: &Self,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    Self::post_create(updated, update).await
  }

  // RENAME

  fn rename_operation() -> Operation {
    Operation::RenameProcedure
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
        Execution::BatchRunProcedure(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::RunAction(params) => {
          let action = super::get_check_permissions::<Action>(
            &params.action,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.action = action.id;
        }
        Execution::BatchRunAction(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
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
        Execution::BatchRunBuild(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::CancelBuild(params) => {
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
        Execution::BatchDeploy(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::PullDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::StartDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::RestartDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::PauseDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::UnpauseDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::StopDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::DestroyDeployment(params) => {
          let deployment =
            super::get_check_permissions::<Deployment>(
              &params.deployment,
              user,
              PermissionLevel::Execute,
            )
            .await?;
          params.deployment = deployment.id;
        }
        Execution::BatchDestroyDeployment(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
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
        Execution::BatchCloneRepo(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
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
        Execution::BatchPullRepo(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::BuildRepo(params) => {
          let repo = super::get_check_permissions::<Repo>(
            &params.repo,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.repo = repo.id;
        }
        Execution::BatchBuildRepo(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::CancelRepoBuild(params) => {
          let repo = super::get_check_permissions::<Repo>(
            &params.repo,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.repo = repo.id;
        }
        Execution::StartContainer(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::RestartContainer(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PauseContainer(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::UnpauseContainer(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::StopContainer(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::DestroyContainer(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::StartAllContainers(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::RestartAllContainers(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PauseAllContainers(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::UnpauseAllContainers(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
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
        Execution::PruneContainers(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::DeleteNetwork(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
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
        Execution::DeleteImage(params) => {
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
        Execution::DeleteVolume(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PruneVolumes(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PruneDockerBuilders(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PruneBuildx(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::PruneSystem(params) => {
          let server = super::get_check_permissions::<Server>(
            &params.server,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.server = server.id;
        }
        Execution::RunSync(params) => {
          let sync = super::get_check_permissions::<ResourceSync>(
            &params.sync,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.sync = sync.id;
        }
        Execution::CommitSync(params) => {
          // This one is actually a write operation.
          let sync = super::get_check_permissions::<ResourceSync>(
            &params.sync,
            user,
            PermissionLevel::Write,
          )
          .await?;
          params.sync = sync.id;
        }
        Execution::DeployStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::BatchDeployStack(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::DeployStackIfChanged(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::BatchDeployStackIfChanged(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::PullStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::StartStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::RestartStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::PauseStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::UnpauseStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::StopStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::DestroyStack(params) => {
          let stack = super::get_check_permissions::<Stack>(
            &params.stack,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.stack = stack.id;
        }
        Execution::BatchDestroyStack(_params) => {
          if !user.admin {
            return Err(anyhow!(
              "Non admin user cannot configure Batch executions"
            ));
          }
        }
        Execution::TestAlerter(params) => {
          let alerter = super::get_check_permissions::<Alerter>(
            &params.alerter,
            user,
            PermissionLevel::Execute,
          )
          .await?;
          params.alerter = alerter.id;
        }
        Execution::Sleep(_) => {}
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
      find_collect(&db_client().procedures, None, None)
        .await
        .context("Failed to get Procedures from db")?;
    let cache = procedure_state_cache();
    for procedure in procedures {
      let state = get_procedure_state_from_db(&procedure.id).await;
      cache.insert(procedure.id, state).await;
    }
    anyhow::Ok(())
  }
  .await
  .inspect_err(|e| {
    error!("Failed to refresh Procedure state cache | {e:#}")
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
      .updates
      .find_one(doc! {
        "target.type": "Procedure",
        "target.id": id,
        "operation": "RunProcedure"
      })
      .with_options(
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
    warn!("Failed to get Procedure state for {id} | {e:#}")
  })
  .unwrap_or(ProcedureState::Unknown)
}
