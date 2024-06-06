use std::str::FromStr;

use anyhow::{anyhow, Context};
use futures::future::join_all;
use monitor_client::{
  api::write::CreateTag,
  entities::{
    monitor_timestamp,
    permission::PermissionLevel,
    resource::{AddFilters, Resource, ResourceQuery},
    to_monitor_name,
    update::{ResourceTarget, ResourceTargetVariant, Update},
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, to_document, Document},
    Collection,
  },
};
use partial_derive2::{Diff, FieldDiff, MaybeNone, PartialDiff};
use resolver_api::Resolve;
use serde::{de::DeserializeOwned, Serialize};
use serror::serialize_error_pretty;

use crate::{
  config::core_config,
  helpers::{
    create_permission,
    query::{
      get_resource_ids_for_non_admin, get_tag,
      get_user_permission_on_resource, id_or_name_filter,
    },
    update::{add_update, make_update},
  },
  state::{db_client, State},
};

mod alerter;
mod build;
mod builder;
mod deployment;
mod procedure;
mod repo;
mod server;
mod server_template;
mod sync;

pub use build::{
  refresh_build_state_cache, spawn_build_state_refresh_loop,
};
pub use procedure::{
  refresh_procedure_state_cache, spawn_procedure_state_refresh_loop,
};
pub use repo::{
  refresh_repo_state_cache, spawn_repo_state_refresh_loop,
};

/// Implement on each monitor resource for common methods
pub trait MonitorResource {
  type ListItem: Serialize + Send;
  type Config: Send
    + Sync
    + Unpin
    + Serialize
    + DeserializeOwned
    + PartialDiff<Self::PartialConfig, Self::ConfigDiff>
    + 'static;
  type PartialConfig: Into<Self::Config> + Serialize;
  type ConfigDiff: Into<Self::PartialConfig>
    + Serialize
    + Diff
    + MaybeNone;
  type Info: Send
    + Sync
    + Unpin
    + Default
    + Serialize
    + DeserializeOwned
    + 'static;
  type QuerySpecifics: AddFilters + Default + std::fmt::Debug;

  fn resource_type() -> ResourceTargetVariant;

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>>;

  async fn to_list_item(
    resource: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem;

  #[allow(clippy::ptr_arg)]
  async fn busy(id: &String) -> anyhow::Result<bool>;

  // =======
  // CREATE
  // =======

  fn create_operation() -> Operation;

  fn user_can_create(user: &User) -> bool;

  async fn validate_create_config(
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()>;

  async fn default_info() -> anyhow::Result<Self::Info> {
    Ok(Default::default())
  }

  async fn post_create(
    created: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()>;

  // =======
  // UPDATE
  // =======

  fn update_operation() -> Operation;

  async fn validate_update_config(
    id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()>;

  /// Should be overridden for enum configs, eg Alerter, Builder, ...
  fn update_document(
    _original: Resource<Self::Config, Self::Info>,
    config: Self::PartialConfig,
  ) -> Result<Document, mungos::mongodb::bson::ser::Error> {
    to_document(&config)
  }

  /// Run any required task after resource updated in database but
  /// before the request resolves.
  async fn post_update(
    updated: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()>;

  // =======
  // DELETE
  // =======

  fn delete_operation() -> Operation;

  /// Clean up all links to this resource before deleting it.
  async fn pre_delete(
    resource: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()>;

  /// Run any required task after resource deleted from database but
  /// before the request resolves.
  async fn post_delete(
    resource: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()>;
}

// Methods

// ======
// GET
// ======

pub async fn get<T: MonitorResource>(
  id_or_name: &str,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  T::coll()
    .await
    .find_one(id_or_name_filter(id_or_name), None)
    .await
    .context("failed to query db for resource")?
    .with_context(|| {
      format!(
        "did not find any {} matching {id_or_name}",
        T::resource_type()
      )
    })
}

pub async fn get_check_permissions<T: MonitorResource>(
  id_or_name: &str,
  user: &User,
  permission_level: PermissionLevel,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  let resource = get::<T>(id_or_name).await?;
  if user.admin
    || (permission_level <= PermissionLevel::Read
      && core_config().transparent_mode)
  {
    return Ok(resource);
  }
  let permissions = get_user_permission_on_resource(
    &user.id,
    T::resource_type(),
    &resource.id,
  )
  .await?;
  if permissions >= permission_level {
    Ok(resource)
  } else {
    Err(anyhow!(
      "user does not have required permissions on this {}",
      T::resource_type()
    ))
  }
}

// ======
// LIST
// ======

pub async fn list_for_user<T: MonitorResource>(
  mut query: ResourceQuery<T::QuerySpecifics>,
  user: &User,
) -> anyhow::Result<Vec<T::ListItem>> {
  validate_resource_query_tags(&mut query).await;
  let mut filters = Document::new();
  query.add_filters(&mut filters);
  list_for_user_using_document::<T>(filters, user).await
}

pub async fn list_for_user_using_document<T: MonitorResource>(
  filters: Document,
  user: &User,
) -> anyhow::Result<Vec<T::ListItem>> {
  let list = list_full_for_user_using_document::<T>(filters, user)
    .await?
    .into_iter()
    .map(|resource| T::to_list_item(resource));
  Ok(join_all(list).await)
}

pub async fn list_full_for_user<T: MonitorResource>(
  mut query: ResourceQuery<T::QuerySpecifics>,
  user: &User,
) -> anyhow::Result<Vec<Resource<T::Config, T::Info>>> {
  validate_resource_query_tags(&mut query).await;
  let mut filters = Document::new();
  query.add_filters(&mut filters);
  list_full_for_user_using_document::<T>(filters, user).await
}

async fn list_full_for_user_using_document<T: MonitorResource>(
  mut filters: Document,
  user: &User,
) -> anyhow::Result<Vec<Resource<T::Config, T::Info>>> {
  if !user.admin && !core_config().transparent_mode {
    let ids =
      get_resource_ids_for_non_admin(&user.id, T::resource_type())
        .await?
        .into_iter()
        .flat_map(|id| ObjectId::from_str(&id))
        .collect::<Vec<_>>();
    filters.insert("_id", doc! { "$in": ids });
  }
  find_collect(T::coll().await, filters, None)
    .await
    .with_context(|| {
      format!("failed to pull {}s from mongo", T::resource_type())
    })
}

// =======
// CREATE
// =======

pub async fn create<T: MonitorResource>(
  name: &str,
  mut config: T::PartialConfig,
  user: &User,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  if !T::user_can_create(user) {
    return Err(anyhow!(
      "User does not have permissions to create {}",
      T::resource_type()
    ));
  }

  let name = to_monitor_name(name);

  if ObjectId::from_str(&name).is_ok() {
    return Err(anyhow!("valid ObjectIds cannot be used as names"));
  }

  let start_ts = monitor_timestamp();

  T::validate_create_config(&mut config, user).await?;

  let resource = Resource::<T::Config, T::Info> {
    id: Default::default(),
    name,
    updated_at: start_ts,
    description: Default::default(),
    tags: Default::default(),
    config: config.into(),
    info: T::default_info().await?,
  };

  let resource_id = T::coll()
    .await
    .insert_one(&resource, None)
    .await
    .with_context(|| {
      format!("failed to add {} to db", T::resource_type())
    })?
    .inserted_id
    .as_object_id()
    .context("inserted_id is not ObjectId")?
    .to_string();

  let resource = get::<T>(&resource_id).await?;
  let target = resource_target::<T>(resource_id);

  create_permission(user, target.clone(), PermissionLevel::Write)
    .await;

  let mut update = make_update(target, T::create_operation(), user);
  update.start_ts = start_ts;
  update.push_simple_log(
    &format!("create {}", T::resource_type()),
    format!(
      "created {}\nid: {}\nname: {}",
      T::resource_type(),
      resource.id,
      resource.name
    ),
  );
  update.push_simple_log(
    "config",
    serde_json::to_string_pretty(&resource.config)
      .context("failed to serialize resource config to JSON")?,
  );

  T::post_create(&resource, &mut update).await?;

  update.finalize();
  add_update(update).await?;

  Ok(resource)
}

// =======
// UPDATE
// =======

pub async fn update<T: MonitorResource>(
  id_or_name: &str,
  mut config: T::PartialConfig,
  user: &User,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  let resource = get_check_permissions::<T>(
    id_or_name,
    user,
    PermissionLevel::Write,
  )
  .await?;

  if T::busy(&resource.id).await? {
    return Err(anyhow!("{} busy", T::resource_type()));
  }

  T::validate_update_config(&resource.id, &mut config, user).await?;

  // Gets a diff object.
  let diff = resource.config.partial_diff(config);

  if diff.is_none() {
    return Err(anyhow!("update has no changes"));
  }

  let mut diff_log = String::from("diff");

  for FieldDiff { field, from, to } in diff.iter_field_diffs() {
    diff_log.push_str(&format!(
      "\n\n<span class=\"text-muted-foreground\">field</span>: '{field}'\n<span class=\"text-muted-foreground\">from</span>:  <span class=\"text-red-500\">{from}</span>\n<span class=\"text-muted-foreground\">to</span>:    <span class=\"text-green-500\">{to}</span>",
    ));
  }

  // This minimizes the update against the existing config
  let config: T::PartialConfig = diff.into();

  let id = resource.id.clone();

  let config_doc = T::update_document(resource, config)
    .context("failed to serialize config to bson document")?;

  update_one_by_id(
    T::coll().await,
    &id,
    mungos::update::Update::FlattenSet(doc! { "config": config_doc }),
    None,
  )
  .await
  .context("failed to update resource on database")?;

  let mut update = make_update(
    resource_target::<T>(id),
    T::update_operation(),
    user,
  );

  update.push_simple_log("update config", diff_log);

  let updated = get::<T>(id_or_name).await?;

  T::post_update(&updated, &mut update).await?;

  update.finalize();

  add_update(update).await?;

  Ok(updated)
}

fn resource_target<T: MonitorResource>(id: String) -> ResourceTarget {
  match T::resource_type() {
    ResourceTargetVariant::System => ResourceTarget::System(id),
    ResourceTargetVariant::Build => ResourceTarget::Build(id),
    ResourceTargetVariant::Builder => ResourceTarget::Builder(id),
    ResourceTargetVariant::Deployment => {
      ResourceTarget::Deployment(id)
    }
    ResourceTargetVariant::Server => ResourceTarget::Server(id),
    ResourceTargetVariant::Repo => ResourceTarget::Repo(id),
    ResourceTargetVariant::Alerter => ResourceTarget::Alerter(id),
    ResourceTargetVariant::Procedure => ResourceTarget::Procedure(id),
    ResourceTargetVariant::ServerTemplate => {
      ResourceTarget::ServerTemplate(id)
    }
    ResourceTargetVariant::ResourceSync => {
      ResourceTarget::ResourceSync(id)
    }
  }
}

pub async fn update_description<T: MonitorResource>(
  id_or_name: &str,
  description: &str,
  user: &User,
) -> anyhow::Result<()> {
  get_check_permissions::<T>(
    id_or_name,
    user,
    PermissionLevel::Write,
  )
  .await?;
  T::coll()
    .await
    .update_one(
      id_or_name_filter(id_or_name),
      doc! { "$set": { "description": description } },
      None,
    )
    .await?;
  Ok(())
}

pub async fn update_tags<T: MonitorResource>(
  id_or_name: &str,
  tags: Vec<String>,
  user: User,
) -> anyhow::Result<()> {
  let futures = tags.iter().map(|tag| async {
    match get_tag(tag).await {
      Ok(tag) => Ok(tag.id),
      Err(_) => State
        .resolve(
          CreateTag {
            name: tag.to_string(),
          },
          user.clone(),
        )
        .await
        .map(|tag| tag.id),
    }
  });
  let tags = join_all(futures)
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
  T::coll()
    .await
    .update_one(
      id_or_name_filter(id_or_name),
      doc! { "$set": { "tags": tags } },
      None,
    )
    .await?;
  Ok(())
}

pub async fn remove_tag_from_all<T: MonitorResource>(
  tag_id: &str,
) -> anyhow::Result<()> {
  T::coll()
    .await
    .update_many(doc! {}, doc! { "$pull": { "tags": tag_id } }, None)
    .await
    .context("failed to remove tag from resources")?;
  Ok(())
}

// =======
// DELETE
// =======

pub async fn delete<T: MonitorResource>(
  id_or_name: &str,
  user: &User,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  let resource = get_check_permissions::<T>(
    id_or_name,
    user,
    PermissionLevel::Write,
  )
  .await?;

  if T::busy(&resource.id).await? {
    return Err(anyhow!("{} busy", T::resource_type()));
  }

  let target = resource_target::<T>(resource.id.clone());

  let mut update =
    make_update(target.clone(), T::delete_operation(), user);

  T::pre_delete(&resource, &mut update).await?;

  delete_all_permissions_on_resource(target.clone()).await;
  remove_from_recently_viewed(target.clone()).await;

  delete_one_by_id(T::coll().await, &resource.id, None)
    .await
    .with_context(|| {
      format!("failed to delete {} from database", T::resource_type())
    })?;

  update.push_simple_log(
    &format!("delete {}", T::resource_type()),
    format!("deleted {} {}", T::resource_type(), resource.name),
  );

  if let Err(e) = T::post_delete(&resource, &mut update).await {
    update.push_error_log("post delete", serialize_error_pretty(&e));
  }

  update.finalize();
  add_update(update).await?;

  Ok(resource)
}

// =======

#[instrument(level = "debug")]
pub async fn validate_resource_query_tags<
  T: Default + std::fmt::Debug,
>(
  query: &mut ResourceQuery<T>,
) {
  let futures = query.tags.iter().map(|tag| get_tag(tag));
  let res = join_all(futures).await;
  query.tags = res.into_iter().flatten().map(|tag| tag.id).collect();
}

#[instrument]
pub async fn delete_all_permissions_on_resource<T>(target: T)
where
  T: Into<ResourceTarget> + std::fmt::Debug,
{
  let target: ResourceTarget = target.into();
  let (variant, id) = target.extract_variant_id();
  if let Err(e) = db_client()
    .await
    .permissions
    .delete_many(
      doc! {
        "resource_target.type": variant.as_ref(),
        "resource_target.id": &id
      },
      None,
    )
    .await
  {
    warn!("failed to delete_many permissions matching target {target:?} | {e:#}");
  }
}

#[instrument]
pub async fn remove_from_recently_viewed<T>(resource: T)
where
  T: Into<ResourceTarget> + std::fmt::Debug,
{
  let resource: ResourceTarget = resource.into();
  let (recent_field, id) = match resource {
    ResourceTarget::Server(id) => ("recent_servers", id),
    ResourceTarget::Deployment(id) => ("recent_deployments", id),
    ResourceTarget::Build(id) => ("recent_builds", id),
    ResourceTarget::Repo(id) => ("recent_repos", id),
    ResourceTarget::Procedure(id) => ("recent_procedures", id),
    // Don't need to do anything for others
    _ => return,
  };
  if let Err(e) = db_client()
    .await
    .users
    .update_many(
      doc! {},
      doc! {
        "$pull": {
          recent_field: id
        }
      },
      None,
    )
    .await
    .context("failed to remove resource from users recently viewed")
  {
    warn!("{e:#}");
  }
}
