use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
  entities::{
    alerter::{Alerter, AlerterInfo},
    Operation, PermissionLevel,
  },
  monitor_timestamp,
  requests::write::{
    CopyAlerter, CreateAlerter, DeleteAlerter, UpdateAlerter,
  },
};
use mungos::mongodb::bson::{doc, to_bson};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser,
  helpers::{make_update, resource::StateResource},
  state::State,
};

#[async_trait]
impl Resolve<CreateAlerter, RequestUser> for State {
  async fn resolve(
    &self,
    CreateAlerter { name, config }: CreateAlerter,
    user: RequestUser,
  ) -> anyhow::Result<Alerter> {
    let start_ts = monitor_timestamp();
    let is_default =
      self.db.alerters.find_one(None, None).await?.is_none();
    let alerter = Alerter {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: AlerterInfo { is_default },
    };
    let alerter_id = self
      .db
      .alerters
      .create_one(alerter)
      .await
      .context("failed to add alerter to db")?;
    let alerter: Alerter = self.get_resource(&alerter_id).await?;

    let mut update =
      make_update(&alerter, Operation::CreateAlerter, &user);

    update.push_simple_log(
      "create alerter",
      format!(
        "created alerter\nid: {}\nname: {}",
        alerter.id, alerter.name
      ),
    );
    update
      .push_simple_log("config", format!("{:#?}", alerter.config));

    update.finalize();

    self.add_update(update).await?;

    Ok(alerter)
  }
}

#[async_trait]
impl Resolve<CopyAlerter, RequestUser> for State {
  async fn resolve(
    &self,
    CopyAlerter { name, id }: CopyAlerter,
    user: RequestUser,
  ) -> anyhow::Result<Alerter> {
    let Alerter {
      config,
      description,
      ..
    } = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;
    let start_ts = monitor_timestamp();
    let alerter = Alerter {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description,
      config,
      tags: Default::default(),
      info: Default::default(),
    };
    let alerter_id = self
      .db
      .alerters
      .create_one(alerter)
      .await
      .context("failed to add alerter to db")?;
    let alerter: Alerter = self.get_resource(&alerter_id).await?;

    let mut update =
      make_update(&alerter, Operation::CreateAlerter, &user);

    update.push_simple_log(
      "create alerter",
      format!(
        "created alerter\nid: {}\nname: {}",
        alerter.id, alerter.name
      ),
    );

    update
      .push_simple_log("config", format!("{:#?}", alerter.config));

    update.finalize();

    self.add_update(update).await?;

    Ok(alerter)
  }
}

#[async_trait]
impl Resolve<DeleteAlerter, RequestUser> for State {
  async fn resolve(
    &self,
    DeleteAlerter { id }: DeleteAlerter,
    user: RequestUser,
  ) -> anyhow::Result<Alerter> {
    let alerter: Alerter = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let mut update =
      make_update(&alerter, Operation::DeleteAlerter, &user);

    self
      .db
      .alerters
      .delete_one(&id)
      .await
      .context("failed to delete alerter from database")?;

    update.push_simple_log(
      "delete alerter",
      format!("deleted alerter {}", alerter.name),
    );

    update.finalize();

    self.add_update(update).await?;

    self.remove_from_recently_viewed(&alerter).await?;

    Ok(alerter)
  }
}

#[async_trait]
impl Resolve<UpdateAlerter, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateAlerter { id, config }: UpdateAlerter,
    user: RequestUser,
  ) -> anyhow::Result<Alerter> {
    let alerter: Alerter = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let mut update =
      make_update(&alerter, Operation::UpdateAlerter, &user);

    update.push_simple_log(
      "alerter update",
      serde_json::to_string_pretty(&config)?,
    );

    let config = alerter.config.merge_partial(config);

    self
      .db
      .alerters
      .update_one(
        &id,
        mungos::Update::FlattenSet(
          doc! { "config": to_bson(&config)? },
        ),
      )
      .await?;

    let alerter: Alerter = self.get_resource(&id).await?;

    update.finalize();
    self.add_update(update).await?;

    Ok(alerter)
  }
}
