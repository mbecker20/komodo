use std::{
  collections::{HashMap, HashSet},
  str::FromStr,
};

use anyhow::{anyhow, Context};
use formatting::format_serror;
use futures::{future::join_all, FutureExt};
use komodo_client::{
  api::{read::ExportResourcesToToml, write::CreateTag},
  entities::{
    komodo_timestamp,
    permission::PermissionLevel,
    resource::{AddFilters, Resource, ResourceQuery},
    tag::Tag,
    to_komodo_name,
    update::Update,
    user::{system_user, User},
    Operation, ResourceTarget, ResourceTargetVariant,
  },
  parsers::parse_string_list,
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, to_document, Document},
    options::FindOptions,
    Collection,
  },
};
use partial_derive2::{Diff, FieldDiff, MaybeNone, PartialDiff};
use resolver_api::Resolve;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
  api::{read::ReadArgs, write::WriteArgs},
  config::core_config,
  helpers::{
    create_permission, flatten_document,
    query::{
      get_tag, get_user_user_groups, id_or_name_filter,
      user_target_query,
    },
    update::{add_update, make_update},
  },
  state::db_client,
};

mod action;
mod alerter;
mod build;
mod builder;
mod deployment;
mod procedure;
mod refresh;
mod repo;
mod server;
mod server_template;
mod stack;
mod sync;

pub use action::{
  refresh_action_state_cache, spawn_action_state_refresh_loop,
};
pub use build::{
  refresh_build_state_cache, spawn_build_state_refresh_loop,
};
pub use procedure::{
  refresh_procedure_state_cache, spawn_procedure_state_refresh_loop,
};
pub use refresh::spawn_resource_refresh_loop;
pub use repo::{
  refresh_repo_state_cache, spawn_repo_state_refresh_loop,
};
pub use sync::{
  refresh_resource_sync_state_cache,
  spawn_resource_sync_state_refresh_loop,
};

/// Implement on each Komodo resource for common methods
pub trait KomodoResource {
  type ListItem: Serialize + Send;
  type Config: Clone
    + Default
    + Send
    + Sync
    + Unpin
    + Serialize
    + DeserializeOwned
    + From<Self::PartialConfig>
    + PartialDiff<Self::PartialConfig, Self::ConfigDiff>
    + 'static;
  type PartialConfig: Clone
    + Default
    + From<Self::Config>
    + Serialize
    + MaybeNone;
  type ConfigDiff: Into<Self::PartialConfig>
    + Serialize
    + Diff
    + MaybeNone;
  type Info: Clone
    + Send
    + Sync
    + Unpin
    + Default
    + Serialize
    + DeserializeOwned
    + 'static;
  type QuerySpecifics: AddFilters + Default + std::fmt::Debug;

  fn resource_type() -> ResourceTargetVariant;

  fn coll() -> &'static Collection<Resource<Self::Config, Self::Info>>;

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
  // RENAME
  // =======

  fn rename_operation() -> Operation;

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

pub async fn get<T: KomodoResource>(
  id_or_name: &str,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  T::coll()
    .find_one(id_or_name_filter(id_or_name))
    .await
    .context("failed to query db for resource")?
    .with_context(|| {
      format!(
        "did not find any {} matching {id_or_name}",
        T::resource_type()
      )
    })
}

pub async fn get_check_permissions<T: KomodoResource>(
  id_or_name: &str,
  user: &User,
  permission_level: PermissionLevel,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  let resource = get::<T>(id_or_name).await?;
  if user.admin
    // Allow if its just read or below, and transparent mode enabled
    || (permission_level <= PermissionLevel::Read
      && core_config().transparent_mode)
    // Allow if resource has base permission level greater than or equal to required permission level
    || resource.base_permission >= permission_level
  {
    return Ok(resource);
  }
  let permissions =
    get_user_permission_on_resource::<T>(user, &resource.id).await?;
  if permissions >= permission_level {
    Ok(resource)
  } else {
    Err(anyhow!(
      "User does not have required permissions on this {}. Must have at least {permission_level} permissions",
      T::resource_type()
    ))
  }
}

#[instrument(level = "debug")]
pub async fn get_user_permission_on_resource<T: KomodoResource>(
  user: &User,
  resource_id: &str,
) -> anyhow::Result<PermissionLevel> {
  if user.admin {
    return Ok(PermissionLevel::Write);
  }

  let resource_type = T::resource_type();

  // Start with base of Read or None
  let mut base = if core_config().transparent_mode {
    PermissionLevel::Read
  } else {
    PermissionLevel::None
  };

  // Add in the resource level global base permission
  let resource_base = get::<T>(resource_id).await?.base_permission;
  if resource_base > base {
    base = resource_base;
  }

  // Overlay users base on resource variant
  if let Some(level) = user.all.get(&resource_type).cloned() {
    if level > base {
      base = level;
    }
  }
  if base == PermissionLevel::Write {
    // No reason to keep going if already Write at this point.
    return Ok(PermissionLevel::Write);
  }

  // Overlay any user groups base on resource variant
  let groups = get_user_user_groups(&user.id).await?;
  for group in &groups {
    if let Some(level) = group.all.get(&resource_type).cloned() {
      if level > base {
        base = level;
      }
    }
  }
  if base == PermissionLevel::Write {
    // No reason to keep going if already Write at this point.
    return Ok(PermissionLevel::Write);
  }

  // Overlay any specific permissions
  let permission = find_collect(
    &db_client().permissions,
    doc! {
      "$or": user_target_query(&user.id, &groups)?,
      "resource_target.type": resource_type.as_ref(),
      "resource_target.id": resource_id
    },
    None,
  )
  .await
  .context("failed to query db for permissions")?
  .into_iter()
  // get the max permission user has between personal / any user groups
  .fold(base, |level, permission| {
    if permission.level > level {
      permission.level
    } else {
      level
    }
  });
  Ok(permission)
}

// ======
// LIST
// ======

/// Returns None if still no need to filter by resource id (eg transparent mode, group membership with all access).
#[instrument(level = "debug")]
pub async fn get_resource_object_ids_for_user<T: KomodoResource>(
  user: &User,
) -> anyhow::Result<Option<Vec<ObjectId>>> {
  get_resource_ids_for_user::<T>(user).await.map(|ids| {
    ids.map(|ids| {
      ids
        .into_iter()
        .flat_map(|id| ObjectId::from_str(&id))
        .collect()
    })
  })
}

/// Returns None if still no need to filter by resource id (eg transparent mode, group membership with all access).
#[instrument(level = "debug")]
pub async fn get_resource_ids_for_user<T: KomodoResource>(
  user: &User,
) -> anyhow::Result<Option<Vec<String>>> {
  // Check admin or transparent mode
  if user.admin || core_config().transparent_mode {
    return Ok(None);
  }

  let resource_type = T::resource_type();

  // Check user 'all' on variant
  if let Some(level) = user.all.get(&resource_type).cloned() {
    if level > PermissionLevel::None {
      return Ok(None);
    }
  }

  // Check user groups 'all' on variant
  let groups = get_user_user_groups(&user.id).await?;
  for group in &groups {
    if let Some(level) = group.all.get(&resource_type).cloned() {
      if level > PermissionLevel::None {
        return Ok(None);
      }
    }
  }

  let (base, perms) = tokio::try_join!(
    // Get any resources with non-none base permission,
    find_collect(
      T::coll(),
      doc! { "base_permission": { "$exists": true, "$ne": "None" } },
      None,
    )
    .map(|res| res.with_context(|| format!(
      "failed to query {resource_type} on db"
    ))),
    // And any ids using the permissions table
    find_collect(
      &db_client().permissions,
      doc! {
        "$or": user_target_query(&user.id, &groups)?,
        "resource_target.type": resource_type.as_ref(),
        "level": { "$exists": true, "$ne": "None" }
      },
      None,
    )
    .map(|res| res.context("failed to query permissions on db"))
  )?;

  // Add specific ids
  let ids = perms
    .into_iter()
    .map(|p| p.resource_target.extract_variant_id().1.to_string())
    // Chain in the ones with non-None base permissions
    .chain(base.into_iter().map(|res| res.id))
    // collect into hashset first to remove any duplicates
    .collect::<HashSet<_>>();

  Ok(Some(ids.into_iter().collect()))
}

#[instrument(level = "debug")]
pub async fn list_for_user<T: KomodoResource>(
  mut query: ResourceQuery<T::QuerySpecifics>,
  user: &User,
  all_tags: &[Tag],
) -> anyhow::Result<Vec<T::ListItem>> {
  validate_resource_query_tags(&mut query, all_tags)?;
  let mut filters = Document::new();
  query.add_filters(&mut filters);
  list_for_user_using_document::<T>(filters, user).await
}

#[instrument(level = "debug")]
pub async fn list_for_user_using_pattern<T: KomodoResource>(
  pattern: &str,
  query: ResourceQuery<T::QuerySpecifics>,
  user: &User,
  all_tags: &[Tag],
) -> anyhow::Result<Vec<T::ListItem>> {
  let list = list_full_for_user_using_pattern::<T>(
    pattern, query, user, all_tags,
  )
  .await?
  .into_iter()
  .map(|resource| T::to_list_item(resource));
  Ok(join_all(list).await)
}

#[instrument(level = "debug")]
pub async fn list_for_user_using_document<T: KomodoResource>(
  filters: Document,
  user: &User,
) -> anyhow::Result<Vec<T::ListItem>> {
  let list = list_full_for_user_using_document::<T>(filters, user)
    .await?
    .into_iter()
    .map(|resource| T::to_list_item(resource));
  Ok(join_all(list).await)
}

/// Lists full resource matching wildcard syntax,
/// or regex if wrapped with "\\"
///
/// ## Example
/// ```
/// let items = list_full_for_user_using_match_string::<Build>("foo-*", Default::default(), user, all_tags).await?;
/// let items = list_full_for_user_using_match_string::<Build>("\\^foo-.*$\\", Default::default(), user, all_tags).await?;
/// ```
#[instrument(level = "debug")]
pub async fn list_full_for_user_using_pattern<T: KomodoResource>(
  pattern: &str,
  query: ResourceQuery<T::QuerySpecifics>,
  user: &User,
  all_tags: &[Tag],
) -> anyhow::Result<Vec<Resource<T::Config, T::Info>>> {
  let resources =
    list_full_for_user::<T>(query, user, all_tags).await?;

  let patterns = parse_string_list(pattern);
  let mut names = HashSet::<String>::new();

  for pattern in patterns {
    if pattern.starts_with('\\') && pattern.ends_with('\\') {
      let regex = regex::Regex::new(&pattern[1..(pattern.len() - 1)])
        .context("Regex matching string invalid")?;
      for resource in &resources {
        if regex.is_match(&resource.name) {
          names.insert(resource.name.clone());
        }
      }
    } else {
      let wildcard = wildcard::Wildcard::new(pattern.as_bytes())
        .context("Wildcard matching string invalid")?;
      for resource in &resources {
        if wildcard.is_match(resource.name.as_bytes()) {
          names.insert(resource.name.clone());
        }
      }
    };
  }

  Ok(
    resources
      .into_iter()
      .filter(|resource| names.contains(resource.name.as_str()))
      .collect(),
  )
}

#[instrument(level = "debug")]
pub async fn list_full_for_user<T: KomodoResource>(
  mut query: ResourceQuery<T::QuerySpecifics>,
  user: &User,
  all_tags: &[Tag],
) -> anyhow::Result<Vec<Resource<T::Config, T::Info>>> {
  validate_resource_query_tags(&mut query, all_tags)?;
  let mut filters = Document::new();
  query.add_filters(&mut filters);
  list_full_for_user_using_document::<T>(filters, user).await
}

#[instrument(level = "debug")]
pub async fn list_full_for_user_using_document<T: KomodoResource>(
  mut filters: Document,
  user: &User,
) -> anyhow::Result<Vec<Resource<T::Config, T::Info>>> {
  if let Some(ids) =
    get_resource_object_ids_for_user::<T>(user).await?
  {
    filters.insert("_id", doc! { "$in": ids });
  }
  find_collect(
    T::coll(),
    filters,
    FindOptions::builder().sort(doc! { "name": 1 }).build(),
  )
  .await
  .with_context(|| {
    format!("failed to pull {}s from mongo", T::resource_type())
  })
}

pub type IdResourceMap<T> = HashMap<
  String,
  Resource<
    <T as KomodoResource>::Config,
    <T as KomodoResource>::Info,
  >,
>;

#[instrument(level = "debug")]
pub async fn get_id_to_resource_map<T: KomodoResource>(
  id_to_tags: &HashMap<String, Tag>,
  match_tags: &[String],
) -> anyhow::Result<IdResourceMap<T>> {
  let res = find_collect(T::coll(), None, None)
    .await
    .with_context(|| {
      format!("failed to pull {}s from mongo", T::resource_type())
    })?
    .into_iter()
    .filter(|resource| {
      if match_tags.is_empty() {
        return true;
      }
      for tag in match_tags.iter() {
        for resource_tag in &resource.tags {
          match ObjectId::from_str(resource_tag) {
            Ok(_) => match id_to_tags
              .get(resource_tag)
              .map(|tag| tag.name.as_str())
            {
              Some(name) => {
                if tag != name {
                  return false;
                }
              }
              None => return false,
            },
            Err(_) => {
              if resource_tag != tag {
                return false;
              }
            }
          }
        }
      }
      true
    })
    .map(|r| (r.id.clone(), r))
    .collect();
  Ok(res)
}

// =======
// CREATE
// =======

pub async fn create<T: KomodoResource>(
  name: &str,
  mut config: T::PartialConfig,
  user: &User,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  if !T::user_can_create(user) {
    return Err(anyhow!(
      "User does not have permissions to create {}.",
      T::resource_type()
    ));
  }

  if name.is_empty() {
    return Err(anyhow!("Must provide non-empty name for resource."));
  }

  let name = to_komodo_name(name);

  if ObjectId::from_str(&name).is_ok() {
    return Err(anyhow!("valid ObjectIds cannot be used as names."));
  }

  // Ensure an existing resource with same name doesn't already exist
  // The database indexing also ensures this but doesn't give a good error message.
  if list_full_for_user::<T>(Default::default(), system_user(), &[])
    .await
    .context("Failed to list all resources for duplicate name check")?
    .into_iter()
    .any(|r| r.name == name)
  {
    return Err(anyhow!("Must provide unique name for resource."));
  }

  let start_ts = komodo_timestamp();

  T::validate_create_config(&mut config, user).await?;

  let resource = Resource::<T::Config, T::Info> {
    id: Default::default(),
    name,
    updated_at: start_ts,
    description: Default::default(),
    tags: Default::default(),
    config: config.into(),
    info: T::default_info().await?,
    base_permission: PermissionLevel::None,
  };

  let resource_id = T::coll()
    .insert_one(&resource)
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

pub async fn update<T: KomodoResource>(
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
    return Ok(resource);
  }

  let mut diff_log = String::from("diff");

  for FieldDiff { field, from, to } in diff.iter_field_diffs() {
    diff_log.push_str(&format!(
      "\n\n<span class=\"text-muted-foreground\">field</span>: '{field}'\n<span class=\"text-muted-foreground\">from</span>:  <span class=\"text-red-700 dark:text-red-400\">{from}</span>\n<span class=\"text-muted-foreground\">to</span>:    <span class=\"text-green-700 dark:text-green-400\">{to}</span>",
    ));
  }

  // This minimizes the update against the existing config
  let config: T::PartialConfig = diff.into();

  let id = resource.id.clone();

  let config_doc = T::update_document(resource, config)
    .context("failed to serialize config to bson document")?;

  let update_doc = flatten_document(doc! { "config": config_doc });

  update_one_by_id(T::coll(), &id, doc! { "$set": update_doc }, None)
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

fn resource_target<T: KomodoResource>(id: String) -> ResourceTarget {
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
    ResourceTargetVariant::Stack => ResourceTarget::Stack(id),
    ResourceTargetVariant::Action => ResourceTarget::Action(id),
  }
}

pub async fn update_description<T: KomodoResource>(
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
    .update_one(
      id_or_name_filter(id_or_name),
      doc! { "$set": { "description": description } },
    )
    .await?;
  Ok(())
}

pub async fn update_tags<T: KomodoResource>(
  id_or_name: &str,
  tags: Vec<String>,
  args: &WriteArgs,
) -> anyhow::Result<()> {
  let futures = tags.iter().map(|tag| async {
    match get_tag(tag).await {
      Ok(tag) => Ok(tag.id),
      Err(_) => CreateTag {
        name: tag.to_string(),
      }
      .resolve(args)
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
    .update_one(
      id_or_name_filter(id_or_name),
      doc! { "$set": { "tags": tags } },
    )
    .await?;
  Ok(())
}

pub async fn remove_tag_from_all<T: KomodoResource>(
  tag_id: &str,
) -> anyhow::Result<()> {
  T::coll()
    .update_many(doc! {}, doc! { "$pull": { "tags": tag_id } })
    .await
    .context("failed to remove tag from resources")?;
  Ok(())
}

// =======
// RENAME
// =======

pub async fn rename<T: KomodoResource>(
  id_or_name: &str,
  name: &str,
  user: &User,
) -> anyhow::Result<Update> {
  let resource = get_check_permissions::<T>(
    id_or_name,
    user,
    PermissionLevel::Write,
  )
  .await?;

  let mut update = make_update(
    resource_target::<T>(resource.id.clone()),
    T::rename_operation(),
    user,
  );

  let name = to_komodo_name(name);

  update_one_by_id(
    T::coll(),
    &resource.id,
    mungos::update::Update::Set(
      doc! { "name": &name, "updated_at": komodo_timestamp() },
    ),
    None,
  )
  .await
  .with_context(|| {
    format!(
      "Failed to update {ty} on db. This name may already be taken.",
      ty = T::resource_type()
    )
  })?;

  update.push_simple_log(
    &format!("Rename {}", T::resource_type()),
    format!(
      "Renamed {ty} {id} from {prev_name} to {name}",
      ty = T::resource_type(),
      id = resource.id,
      prev_name = resource.name
    ),
  );

  update.finalize();
  update.id = add_update(update.clone()).await?;
  Ok(update)
}

// =======
// DELETE
// =======

pub async fn delete<T: KomodoResource>(
  id_or_name: &str,
  args: &WriteArgs,
) -> anyhow::Result<Resource<T::Config, T::Info>> {
  let resource = get_check_permissions::<T>(
    id_or_name,
    &args.user,
    PermissionLevel::Write,
  )
  .await?;

  if T::busy(&resource.id).await? {
    return Err(anyhow!("{} busy", T::resource_type()));
  }

  let target = resource_target::<T>(resource.id.clone());
  let toml = ExportResourcesToToml {
    targets: vec![target.clone()],
    ..Default::default()
  }
  .resolve(&ReadArgs {
    user: args.user.clone(),
  })
  .await
  .map_err(|e| e.error)?
  .toml;

  let mut update =
    make_update(target.clone(), T::delete_operation(), &args.user);

  T::pre_delete(&resource, &mut update).await?;

  delete_all_permissions_on_resource(target.clone()).await;
  remove_from_recently_viewed(target.clone()).await;

  delete_one_by_id(T::coll(), &resource.id, None)
    .await
    .with_context(|| {
      format!("Failed to delete {} from database", T::resource_type())
    })?;

  update.push_simple_log(
    &format!("Delete {}", T::resource_type()),
    format!("Deleted {} {}", T::resource_type(), resource.name),
  );
  update.push_simple_log("Deleted Toml", toml);

  if let Err(e) = T::post_delete(&resource, &mut update).await {
    update.push_error_log("post delete", format_serror(&e.into()));
  }

  update.finalize();
  add_update(update).await?;

  Ok(resource)
}

// =======

#[instrument(level = "debug")]
pub fn validate_resource_query_tags<T: Default + std::fmt::Debug>(
  query: &mut ResourceQuery<T>,
  all_tags: &[Tag],
) -> anyhow::Result<()> {
  query.tags = query
    .tags
    .iter()
    .map(|tag| {
      all_tags
        .iter()
        .find(|t| t.name == *tag || t.id == *tag)
        .map(|tag| tag.id.clone())
        .with_context(|| {
          format!("No tag found matching name or id: {}", tag)
        })
    })
    .collect::<anyhow::Result<Vec<_>>>()?;
  Ok(())
}

#[instrument]
pub async fn delete_all_permissions_on_resource<T>(target: T)
where
  T: Into<ResourceTarget> + std::fmt::Debug,
{
  let target: ResourceTarget = target.into();
  let (variant, id) = target.extract_variant_id();
  if let Err(e) = db_client()
    .permissions
    .delete_many(doc! {
      "resource_target.type": variant.as_ref(),
      "resource_target.id": &id
    })
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
    ResourceTarget::Server(id) => ("recents.Server", id),
    ResourceTarget::Deployment(id) => ("recents.Deployment", id),
    ResourceTarget::Build(id) => ("recents.Build", id),
    ResourceTarget::Repo(id) => ("recents.Repo", id),
    ResourceTarget::Procedure(id) => ("recents.Procedure", id),
    ResourceTarget::Action(id) => ("recents.Action", id),
    ResourceTarget::Stack(id) => ("recents.Stack", id),
    ResourceTarget::Builder(id) => ("recents.Builder", id),
    ResourceTarget::Alerter(id) => ("recents.Alerter", id),
    ResourceTarget::ServerTemplate(id) => {
      ("recents.ServerTemplate", id)
    }
    ResourceTarget::ResourceSync(id) => ("recents.ResourceSync", id),
    ResourceTarget::System(_) => return,
  };
  if let Err(e) = db_client()
    .users
    .update_many(
      doc! {},
      doc! {
        "$pull": {
          recent_field: id
        }
      },
    )
    .await
    .context("failed to remove resource from users recently viewed")
  {
    warn!("{e:#}");
  }
}
