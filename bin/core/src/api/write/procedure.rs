use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    monitor_timestamp, procedure::Procedure, to_monitor_name,
    update::Log, Operation, PermissionLevel,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_document},
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser,
  helpers::{make_update, resource::StateResource},
  state::State,
};

#[async_trait]
impl Resolve<CreateProcedure, RequestUser> for State {
  async fn resolve(
    &self,
    CreateProcedure { name, config }: CreateProcedure,
    user: RequestUser,
  ) -> anyhow::Result<CreateProcedureResponse> {
    let name = to_monitor_name(&name);
    let start_ts = monitor_timestamp();
    let procedure = Procedure {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description: Default::default(),
      tags: Default::default(),
      info: Default::default(),
      config,
    };
    let procedure_id = self
      .db
      .procedures
      .insert_one(procedure, None)
      .await
      .context("failed to add procedure to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let procedure: Procedure =
      self.get_resource(&procedure_id).await?;

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

    self.add_update(update).await?;

    Ok(procedure)
  }
}

#[async_trait]
impl Resolve<CopyProcedure, RequestUser> for State {
  async fn resolve(
    &self,
    CopyProcedure { name, id }: CopyProcedure,
    user: RequestUser,
  ) -> anyhow::Result<CopyProcedureResponse> {
    let name = to_monitor_name(&name);
    let Procedure {
      config,
      description,
      tags,
      ..
    } = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;
    let start_ts = monitor_timestamp();
    let build = Procedure {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description,
      tags,
      config,
      info: Default::default(),
    };
    let procedure_id = self
      .db
      .procedures
      .insert_one(build, None)
      .await
      .context("failed to add build to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let procedure: Procedure =
      self.get_resource(&procedure_id).await?;

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

    self.add_update(update).await?;

    Ok(procedure)
  }
}

#[async_trait]
impl Resolve<UpdateProcedure, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateProcedure { id, config }: UpdateProcedure,
    user: RequestUser,
  ) -> anyhow::Result<UpdateProcedureResponse> {
    let procedure: Procedure = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    update_one_by_id(
      &self.db.procedures,
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

    self.add_update(update).await?;

    let procedure: Procedure =
      self.get_resource(&procedure.id).await?;

    Ok(procedure)
  }
}

#[async_trait]
impl Resolve<DeleteProcedure, RequestUser> for State {
  async fn resolve(
    &self,
    DeleteProcedure { id }: DeleteProcedure,
    user: RequestUser,
  ) -> anyhow::Result<DeleteProcedureResponse> {
    // needs to pull its id from all container procedures
    if self.action_states.procedure.busy(&id).await {
      return Err(anyhow!("procedure busy"));
    }

    let procedure: Procedure = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let mut update =
      make_update(&procedure, Operation::DeleteProcedure, &user);
    update.in_progress();
    update.id = self.add_update(update.clone()).await?;

    let res = delete_one_by_id(&self.db.procedures, &id, None)
      .await
      .context("failed to delete build from database");

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
    self.update_update(update).await?;

    self.remove_from_recently_viewed(&procedure).await?;

    Ok(procedure)
  }
}