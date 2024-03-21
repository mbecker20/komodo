use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    build::Build,
    builder::Builder,
    monitor_timestamp, to_monitor_name,
    update::{Log, UpdateStatus},
    Operation, PermissionLevel,
  },
};
use mungos::{
  by_id::update_one_by_id,
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser,
  db::db_client,
  helpers::{
    add_update, empty_or_only_spaces, make_update,
    remove_from_recently_viewed, resource::StateResource,
    update_update,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<CreateBuild, RequestUser> for State {
  async fn resolve(
    &self,
    CreateBuild { name, config }: CreateBuild,
    user: RequestUser,
  ) -> anyhow::Result<Build> {
    let name = to_monitor_name(&name);
    if let Some(builder_id) = &config.builder_id {
      let _: Builder = self
        .get_resource_check_permissions(builder_id, &user, PermissionLevel::Read)
        .await
        .context("cannot create build using this builder. user must have at least read permissions on the builder.")?;
    }
    let start_ts = monitor_timestamp();
    let build = Build {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: Default::default(),
    };
    let build_id = db_client()
      .await
      .builds
      .insert_one(build, None)
      .await
      .context("failed to add build to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let build: Build = self.get_resource(&build_id).await?;

    let mut update =
      make_update(&build, Operation::CreateBuild, &user);

    update.push_simple_log(
      "create build",
      format!(
        "created build\nid: {}\nname: {}",
        build.id, build.name
      ),
    );

    update.push_simple_log("config", format!("{:#?}", build.config));

    update.finalize();

    add_update(update).await?;

    Ok(build)
  }
}

#[async_trait]
impl Resolve<CopyBuild, RequestUser> for State {
  async fn resolve(
    &self,
    CopyBuild { name, id }: CopyBuild,
    user: RequestUser,
  ) -> anyhow::Result<Build> {
    let name = to_monitor_name(&name);
    let Build {
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
    let _: Builder = self.get_resource_check_permissions(&config.builder_id, &user, PermissionLevel::Read).await.context("cannot create build using this builder. user must have at least read permissions on the builder.")?;
    let start_ts = monitor_timestamp();
    let build = Build {
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
    let build_id = db_client()
      .await
      .builds
      .insert_one(build, None)
      .await
      .context("failed to add build to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let build: Build = self.get_resource(&build_id).await?;

    let mut update =
      make_update(&build, Operation::CreateBuild, &user);

    update.push_simple_log(
      "create build",
      format!(
        "created build\nid: {}\nname: {}",
        build.id, build.name
      ),
    );
    update.push_simple_log(
      "config",
      serde_json::to_string_pretty(&build)?,
    );

    update.finalize();

    add_update(update).await?;

    Ok(build)
  }
}

#[async_trait]
impl Resolve<DeleteBuild, RequestUser> for State {
  async fn resolve(
    &self,
    DeleteBuild { id }: DeleteBuild,
    user: RequestUser,
  ) -> anyhow::Result<Build> {
    if action_states().build.busy(&id).await {
      return Err(anyhow!("build busy"));
    }

    let build: Build = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let mut update =
      make_update(&build, Operation::DeleteBuild, &user);
    update.status = UpdateStatus::InProgress;
    update.id = add_update(update.clone()).await?;

    let res = db_client()
      .await
      .builds
      .delete_one(doc! { "_id": ObjectId::from_str(&id)? }, None)
      .await
      .context("failed to delete build from database");

    let log = match res {
      Ok(_) => Log::simple(
        "delete build",
        format!("deleted build {}", build.name),
      ),
      Err(e) => Log::error(
        "delete build",
        format!("failed to delete build\n{e:#?}"),
      ),
    };

    update.logs.push(log);
    update.finalize();
    update_update(update).await?;

    remove_from_recently_viewed(&build).await?;

    Ok(build)
  }
}

#[async_trait]
impl Resolve<UpdateBuild, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateBuild { id, mut config }: UpdateBuild,
    user: RequestUser,
  ) -> anyhow::Result<Build> {
    if action_states().build.busy(&id).await {
      return Err(anyhow!("build busy"));
    }

    let build: Build = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let inner = || async move {
      if let Some(builder_id) = &config.builder_id {
        let _: Builder = self.get_resource_check_permissions(builder_id, &user, PermissionLevel::Read).await.context("cannot create build using this builder. user must have at least read permissions on the builder.")?;
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

      update_one_by_id(
        &db_client().await.builds,
        &build.id,
        mungos::update::Update::FlattenSet(
          doc! { "config": to_bson(&config)? },
        ),
        None,
      )
      .await
      .context("failed to update build on database")?;

      let mut update =
        make_update(&build, Operation::UpdateBuild, &user);

      update.push_simple_log(
        "build update",
        serde_json::to_string_pretty(&config)?,
      );

      update.finalize();

      add_update(update).await?;

      let build: Build = self.get_resource(&build.id).await?;

      anyhow::Ok(build)
    };

    action_states()
      .build
      .update_entry(id.clone(), |entry| {
        entry.updating = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .build
      .update_entry(id, |entry| {
        entry.updating = false;
      })
      .await;

    res
  }
}
