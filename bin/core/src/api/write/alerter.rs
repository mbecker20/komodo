use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::{
    CopyAlerter, CreateAlerter, DeleteAlerter, UpdateAlerter,
  },
  entities::{
    alerter::{Alerter, AlerterInfo},
    monitor_timestamp,
    permission::PermissionLevel,
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    create_permission, update::{add_update, make_update},
    remove_from_recently_viewed,
    resource::{delete_all_permissions_on_resource, StateResource},
  },
  state::State,
};

#[async_trait]
impl Resolve<CreateAlerter, User> for State {
  #[instrument(name = "CreateAlerter", skip(self, user))]
  async fn resolve(
    &self,
    CreateAlerter { name, config }: CreateAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("valid ObjectIds cannot be used as names"));
    }
    let start_ts = monitor_timestamp();
    let is_default = db_client()
      .await
      .alerters
      .find_one(None, None)
      .await?
      .is_none();
    let alerter = Alerter {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: AlerterInfo { is_default },
    };
    let alerter_id = db_client()
      .await
      .alerters
      .insert_one(alerter, None)
      .await
      .context("failed to add alerter to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let alerter = Alerter::get_resource(&alerter_id).await?;

    create_permission(&user, &alerter, PermissionLevel::Write).await;

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

    add_update(update).await?;

    Ok(alerter)
  }
}

#[async_trait]
impl Resolve<CopyAlerter, User> for State {
  #[instrument(name = "CopyAlerter", skip(self, user))]
  async fn resolve(
    &self,
    CopyAlerter { name, id }: CopyAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    let Alerter {
      config,
      description,
      ..
    } = Alerter::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    let start_ts = monitor_timestamp();
    let alerter = Alerter {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description,
      config,
      tags: Default::default(),
      info: Default::default(),
    };
    let alerter_id = db_client()
      .await
      .alerters
      .insert_one(alerter, None)
      .await
      .context("failed to add alerter to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let alerter = Alerter::get_resource(&alerter_id).await?;

    create_permission(&user, &alerter, PermissionLevel::Write).await;

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

    add_update(update).await?;

    Ok(alerter)
  }
}

#[async_trait]
impl Resolve<DeleteAlerter, User> for State {
  #[instrument(name = "DeleteAlerter", skip(self, user))]
  async fn resolve(
    &self,
    DeleteAlerter { id }: DeleteAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    let alerter = Alerter::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    let mut update =
      make_update(&alerter, Operation::DeleteAlerter, &user);

    delete_one_by_id(&db_client().await.alerters, &id, None)
      .await
      .context("failed to delete alerter from database")?;

    delete_all_permissions_on_resource(&alerter).await;

    update.push_simple_log(
      "delete alerter",
      format!("deleted alerter {}", alerter.name),
    );

    update.finalize();

    add_update(update).await?;

    remove_from_recently_viewed(&alerter).await?;

    Ok(alerter)
  }
}

#[async_trait]
impl Resolve<UpdateAlerter, User> for State {
  #[instrument(name = "UpdateAlerter", skip(self, user))]
  async fn resolve(
    &self,
    UpdateAlerter { id, config }: UpdateAlerter,
    user: User,
  ) -> anyhow::Result<Alerter> {
    let alerter = Alerter::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    let mut update =
      make_update(&alerter, Operation::UpdateAlerter, &user);

    update.push_simple_log(
      "alerter update",
      serde_json::to_string_pretty(&config)?,
    );

    let config = alerter.config.merge_partial(config);

    update_one_by_id(
      &db_client().await.alerters,
      &id,
      mungos::update::Update::FlattenSet(
        doc! { "config": to_bson(&config)? },
      ),
      None,
    )
    .await
    .with_context(|| format!("failed to update alerter {id}"))?;

    let alerter = Alerter::get_resource(&id).await?;

    update.finalize();
    add_update(update).await?;

    Ok(alerter)
  }
}
