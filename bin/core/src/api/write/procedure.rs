use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::{execute::Execution, write::*},
  entities::{
    build::Build,
    deployment::Deployment,
    monitor_timestamp,
    permission::PermissionLevel,
    procedure::{Procedure, ProcedureConfig},
    repo::Repo,
    server::Server,
    update::Log,
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId, to_document},
};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    add_update, create_permission, make_update,
    remove_from_recently_viewed,
    resource::{delete_all_permissions_on_resource, StateResource},
    update_update,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<CreateProcedure, User> for State {
  #[instrument(name = "CreateProcedure", skip(self))]
  async fn resolve(
    &self,
    CreateProcedure { name, mut config }: CreateProcedure,
    user: User,
  ) -> anyhow::Result<CreateProcedureResponse> {
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("valid ObjectIds cannot be used as names"));
    }

    let start_ts = monitor_timestamp();

    validate_procedure_config(&mut config, &user, None).await?;

    let procedure = Procedure {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description: Default::default(),
      tags: Default::default(),
      info: Default::default(),
      config,
    };
    let procedure_id = db_client()
      .await
      .procedures
      .insert_one(procedure, None)
      .await
      .context("failed to add procedure to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let procedure = Procedure::get_resource(&procedure_id).await?;

    create_permission(&user, &procedure, PermissionLevel::Write)
      .await;

    let mut update =
      make_update(&procedure, Operation::CreateProcedure, &user);

    update.push_simple_log(
      "create procedure",
      format!(
        "created procedure\nid: {}\nname: {}",
        procedure.id, procedure.name
      ),
    );

    update
      .push_simple_log("config", format!("{:#?}", procedure.config));

    update.finalize();

    add_update(update).await?;

    Ok(procedure)
  }
}

#[instrument]
async fn validate_procedure_config(
  config: &mut ProcedureConfig,
  user: &User,
  id: Option<&str>,
) -> anyhow::Result<()> {
  let execs = match config {
    ProcedureConfig::Sequence(execs) => execs,
    ProcedureConfig::Parallel(execs) => execs,
  };
  for exec in execs {
    match &mut exec.execution {
      Execution::None(_) => {}
      Execution::RunProcedure(params) => {
        let procedure = Procedure::get_resource_check_permissions(
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
        let build = Build::get_resource_check_permissions(
          &params.build,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.build = build.id;
      }
      Execution::Deploy(params) => {
        let deployment = Deployment::get_resource_check_permissions(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::StartContainer(params) => {
        let deployment = Deployment::get_resource_check_permissions(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::StopContainer(params) => {
        let deployment = Deployment::get_resource_check_permissions(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::StopAllContainers(params) => {
        let server = Server::get_resource_check_permissions(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
      Execution::RemoveContainer(params) => {
        let deployment = Deployment::get_resource_check_permissions(
          &params.deployment,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.deployment = deployment.id;
      }
      Execution::CloneRepo(params) => {
        let repo = Repo::get_resource_check_permissions(
          &params.repo,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.repo = repo.id;
      }
      Execution::PullRepo(params) => {
        let repo = Repo::get_resource_check_permissions(
          &params.repo,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.repo = repo.id;
      }
      Execution::PruneDockerNetworks(params) => {
        let server = Server::get_resource_check_permissions(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
      Execution::PruneDockerImages(params) => {
        let server = Server::get_resource_check_permissions(
          &params.server,
          user,
          PermissionLevel::Execute,
        )
        .await?;
        params.server = server.id;
      }
      Execution::PruneDockerContainers(params) => {
        let server = Server::get_resource_check_permissions(
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

#[async_trait]
impl Resolve<CopyProcedure, User> for State {
  #[instrument(name = "CopyProcedure", skip(self))]
  async fn resolve(
    &self,
    CopyProcedure { name, id }: CopyProcedure,
    user: User,
  ) -> anyhow::Result<CopyProcedureResponse> {
    let Procedure {
      config,
      description,
      tags,
      ..
    } = Procedure::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    let start_ts = monitor_timestamp();
    let build = Procedure {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description,
      tags,
      config,
      info: Default::default(),
    };
    let procedure_id = db_client()
      .await
      .procedures
      .insert_one(build, None)
      .await
      .context("failed to add build to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let procedure = Procedure::get_resource(&procedure_id).await?;

    create_permission(&user, &procedure, PermissionLevel::Write)
      .await;

    let mut update =
      make_update(&procedure, Operation::CreateProcedure, &user);

    update.push_simple_log(
      "create procedure",
      format!(
        "created procedure\nid: {}\nname: {}",
        procedure.id, procedure.name
      ),
    );
    update.push_simple_log(
      "config",
      serde_json::to_string_pretty(&procedure)?,
    );

    update.finalize();

    add_update(update).await?;

    Ok(procedure)
  }
}

#[async_trait]
impl Resolve<UpdateProcedure, User> for State {
  #[instrument(name = "UpdateProcedure", skip(self))]
  async fn resolve(
    &self,
    UpdateProcedure { id, mut config }: UpdateProcedure,
    user: User,
  ) -> anyhow::Result<UpdateProcedureResponse> {
    let procedure = Procedure::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    validate_procedure_config(
      &mut config,
      &user,
      Some(&procedure.id),
    )
    .await?;

    update_one_by_id(
      &db_client().await.procedures,
      &procedure.id,
      mungos::update::Update::FlattenSet(
        doc! { "config": to_document(&config)? },
      ),
      None,
    )
    .await
    .context("failed to update procedure on database")?;

    let mut update =
      make_update(&procedure, Operation::UpdateProcedure, &user);

    update.push_simple_log(
      "procedure update",
      serde_json::to_string_pretty(&config)?,
    );

    update.finalize();

    add_update(update).await?;

    let procedure = Procedure::get_resource(&procedure.id).await?;

    Ok(procedure)
  }
}

#[async_trait]
impl Resolve<DeleteProcedure, User> for State {
  #[instrument(name = "DeleteProcedure", skip(self))]
  async fn resolve(
    &self,
    DeleteProcedure { id }: DeleteProcedure,
    user: User,
  ) -> anyhow::Result<DeleteProcedureResponse> {
    // needs to pull its id from all container procedures
    if action_states().procedure.busy(&id).await {
      return Err(anyhow!("procedure busy"));
    }

    let procedure = Procedure::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    let mut update =
      make_update(&procedure, Operation::DeleteProcedure, &user);
    update.in_progress();
    update.id = add_update(update.clone()).await?;

    let res =
      delete_one_by_id(&db_client().await.procedures, &id, None)
        .await
        .context("failed to delete build from database");

    delete_all_permissions_on_resource(&procedure).await;

    let log = match res {
      Ok(_) => Log::simple(
        "delete procedure",
        format!("deleted procedure {}", procedure.name),
      ),
      Err(e) => Log::error(
        "delete procedure",
        format!("failed to delete procedure\n{e:#?}"),
      ),
    };

    update.logs.push(log);
    update.finalize();
    update_update(update).await?;

    remove_from_recently_viewed(&procedure).await?;

    Ok(procedure)
  }
}
