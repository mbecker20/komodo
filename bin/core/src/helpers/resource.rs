use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Context};
use futures::future::join_all;
use monitor_client::{
  api::write::CreateTag,
  entities::{
    alerter::{
      Alerter, AlerterConfig, AlerterInfo, AlerterListItem,
      AlerterListItemInfo, AlerterQuerySpecifics,
    },
    build::{
      Build, BuildConfig, BuildInfo, BuildListItem,
      BuildListItemInfo, BuildQuerySpecifics,
    },
    builder::{
      Builder, BuilderConfig, BuilderListItem, BuilderListItemInfo,
      BuilderQuerySpecifics,
    },
    deployment::{
      Deployment, DeploymentConfig, DeploymentImage,
      DeploymentListItem, DeploymentListItemInfo,
      DeploymentQuerySpecifics,
    },
    permission::PermissionLevel,
    procedure::{
      Procedure, ProcedureConfig, ProcedureListItem,
      ProcedureListItemInfo, ProcedureQuerySpecifics,
    },
    repo::{
      Repo, RepoConfig, RepoInfo, RepoListItem, RepoListItemInfo,
      RepoQuerySpecifics,
    },
    resource::{AddFilters, Resource, ResourceQuery},
    server::{
      Server, ServerConfig, ServerListItem, ServerListItemInfo,
      ServerQuerySpecifics,
    },
    update::{ResourceTarget, ResourceTargetVariant},
    user::User,
  },
};
use mungos::{
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
  },
};
use resolver_api::Resolve;
use serde::{de::DeserializeOwned, Serialize};

use crate::{db::db_client, state::State};

use super::{
  cache::{deployment_status_cache, server_status_cache},
  query::{get_tag, get_user_user_group_ids},
};

pub trait StateResource {
  type ListItem: Serialize + Send;
  type Config: Send
    + Sync
    + Unpin
    + Serialize
    + DeserializeOwned
    + 'static;
  type Info: Send
    + Sync
    + Unpin
    + Default
    + Serialize
    + DeserializeOwned
    + 'static;
  type QuerySpecifics: AddFilters + Default + std::fmt::Debug;

  fn name() -> &'static str;

  fn resource_target_variant() -> ResourceTargetVariant;

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>>;

  async fn to_list_item(
    resource: Resource<Self::Config, Self::Info>,
  ) -> anyhow::Result<Self::ListItem>;

  async fn get_resource(
    id_or_name: &str,
  ) -> anyhow::Result<Resource<Self::Config, Self::Info>> {
    let filter = match ObjectId::from_str(id_or_name) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": id_or_name },
    };
    Self::coll()
      .await
      .find_one(filter, None)
      .await
      .context("failed to query db for resource")?
      .with_context(|| {
        format!(
          "did not find any {} matching {id_or_name}",
          Self::name()
        )
      })
  }

  async fn get_resource_check_permissions(
    id_or_name: &str,
    user: &User,
    permission_level: PermissionLevel,
  ) -> anyhow::Result<Resource<Self::Config, Self::Info>> {
    let resource = Self::get_resource(id_or_name).await?;
    if user.admin {
      return Ok(resource);
    }
    let permissions =
      Self::get_user_permission_on_resource(&user.id, &resource.id)
        .await?;
    if permissions >= permission_level {
      Ok(resource)
    } else {
      Err(anyhow!(
        "user does not have required permissions on this {}",
        Self::name()
      ))
    }
  }

  async fn get_user_permission_on_resource(
    user_id: &str,
    resource_id: &str,
  ) -> anyhow::Result<PermissionLevel> {
    get_user_permission_on_resource(
      user_id,
      Self::resource_target_variant(),
      resource_id,
    )
    .await
  }

  async fn get_resource_ids_for_non_admin(
    user_id: &str,
  ) -> anyhow::Result<Vec<String>> {
    get_resource_ids_for_non_admin(
      user_id,
      Self::resource_target_variant(),
    )
    .await
  }

  async fn list_resources_for_user(
    mut query: ResourceQuery<Self::QuerySpecifics>,
    user: &User,
  ) -> anyhow::Result<Vec<Self::ListItem>> {
    validate_resource_query_tags(&mut query).await;
    let mut filters = Document::new();
    query.add_filters(&mut filters);
    Self::query_resources_for_user(filters, user).await
  }

  async fn query_resources_for_user(
    mut filters: Document,
    user: &User,
  ) -> anyhow::Result<Vec<Self::ListItem>> {
    if !user.admin {
      let ids = Self::get_resource_ids_for_non_admin(&user.id)
        .await?
        .into_iter()
        .flat_map(|id| ObjectId::from_str(&id))
        .collect::<Vec<_>>();
      filters.insert("_id", doc! { "$in": ids });
    }
    let list = find_collect(Self::coll().await, filters, None)
      .await
      .with_context(|| {
        format!("failed to pull {}s from mongo", Self::name())
      })?
      .into_iter()
      .map(|resource| Self::to_list_item(resource));

    let list = join_all(list)
      .await
      .into_iter()
      .collect::<anyhow::Result<Vec<_>>>()
      .context(format!(
        "failed to convert {} list item",
        Self::name()
      ))?;

    Ok(list)
  }

  async fn update_description(
    id_or_name: &str,
    description: &str,
    user: &User,
  ) -> anyhow::Result<()> {
    Self::get_resource_check_permissions(
      id_or_name,
      user,
      PermissionLevel::Write,
    )
    .await?;
    let filter = match ObjectId::from_str(id_or_name) {
      Ok(id) => doc! { "_id": id },
      Err(_) => doc! { "name": id_or_name },
    };
    Self::coll()
      .await
      .update_one(
        filter,
        doc! { "$set": { "description": description } },
        None,
      )
      .await?;
    Ok(())
  }

  async fn update_tags_on_resource(
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
    Self::coll()
      .await
      .update_one(
        id_or_name_filter(id_or_name),
        doc! { "$set": { "tags": tags } },
        None,
      )
      .await?;
    Ok(())
  }

  async fn remove_tag_from_resources(
    tag_id: &str,
  ) -> anyhow::Result<()> {
    Self::coll()
      .await
      .update_many(
        doc! {},
        doc! { "$pull": { "tags": tag_id } },
        None,
      )
      .await
      .context("failed to remove tag from resources")?;
    Ok(())
  }
}

fn id_or_name_filter(id_or_name: &str) -> Document {
  match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
  }
}

impl StateResource for Server {
  type ListItem = ServerListItem;
  type Config = ServerConfig;
  type Info = ();
  type QuerySpecifics = ServerQuerySpecifics;

  fn name() -> &'static str {
    "server"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Server
  }

  async fn coll() -> &'static Collection<Server> {
    &db_client().await.servers
  }

  async fn to_list_item(
    server: Server,
  ) -> anyhow::Result<ServerListItem> {
    let status = server_status_cache().get(&server.id).await;
    Ok(ServerListItem {
      name: server.name,
      created_at: ObjectId::from_str(&server.id)?
        .timestamp()
        .timestamp_millis(),
      id: server.id,
      tags: server.tags,
      resource_type: ResourceTargetVariant::Server,
      info: ServerListItemInfo {
        status: status.map(|s| s.status).unwrap_or_default(),
        region: server.config.region,
        send_unreachable_alerts: server
          .config
          .send_unreachable_alerts,
        send_cpu_alerts: server.config.send_cpu_alerts,
        send_mem_alerts: server.config.send_mem_alerts,
        send_disk_alerts: server.config.send_disk_alerts,
      },
    })
  }
}

impl StateResource for Deployment {
  type ListItem = DeploymentListItem;
  type Config = DeploymentConfig;
  type Info = ();
  type QuerySpecifics = DeploymentQuerySpecifics;

  fn name() -> &'static str {
    "deployment"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Deployment
  }

  async fn coll() -> &'static Collection<Deployment> {
    &db_client().await.deployments
  }

  async fn to_list_item(
    deployment: Deployment,
  ) -> anyhow::Result<DeploymentListItem> {
    let status = deployment_status_cache().get(&deployment.id).await;
    let (image, build_id) = match deployment.config.image {
      DeploymentImage::Build { build_id, version } => {
        let build = Build::get_resource(&build_id).await?;
        let version = if version.is_none() {
          build.config.version.to_string()
        } else {
          version.to_string()
        };
        (format!("{}:{version}", build.name), Some(build_id))
      }
      DeploymentImage::Image { image } => (image, None),
    };
    Ok(DeploymentListItem {
      name: deployment.name,
      created_at: ObjectId::from_str(&deployment.id)?
        .timestamp()
        .timestamp_millis(),
      id: deployment.id,
      tags: deployment.tags,
      resource_type: ResourceTargetVariant::Deployment,
      info: DeploymentListItemInfo {
        state: status
          .as_ref()
          .map(|s| s.curr.state)
          .unwrap_or_default(),
        status: status.as_ref().and_then(|s| {
          s.curr.container.as_ref().and_then(|c| c.status.to_owned())
        }),
        image,
        server_id: deployment.config.server_id,
        build_id,
      },
    })
  }
}

impl StateResource for Build {
  type ListItem = BuildListItem;
  type Config = BuildConfig;
  type Info = BuildInfo;
  type QuerySpecifics = BuildQuerySpecifics;

  fn name() -> &'static str {
    "build"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Build
  }

  async fn coll() -> &'static Collection<Build> {
    &db_client().await.builds
  }

  async fn to_list_item(
    build: Build,
  ) -> anyhow::Result<BuildListItem> {
    Ok(BuildListItem {
      name: build.name,
      created_at: ObjectId::from_str(&build.id)?
        .timestamp()
        .timestamp_millis(),
      id: build.id,
      tags: build.tags,
      resource_type: ResourceTargetVariant::Build,
      info: BuildListItemInfo {
        last_built_at: build.info.last_built_at,
        version: build.config.version,
        repo: build.config.repo,
        branch: build.config.branch,
      },
    })
  }
}

impl StateResource for Repo {
  type ListItem = RepoListItem;
  type Config = RepoConfig;
  type Info = RepoInfo;
  type QuerySpecifics = RepoQuerySpecifics;

  fn name() -> &'static str {
    "repo"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Repo
  }

  async fn coll() -> &'static Collection<Repo> {
    &db_client().await.repos
  }

  async fn to_list_item(repo: Repo) -> anyhow::Result<RepoListItem> {
    Ok(RepoListItem {
      name: repo.name,
      created_at: ObjectId::from_str(&repo.id)?
        .timestamp()
        .timestamp_millis(),
      id: repo.id,
      tags: repo.tags,
      resource_type: ResourceTargetVariant::Repo,
      info: RepoListItemInfo {
        last_pulled_at: repo.info.last_pulled_at,
        repo: repo.config.repo,
        branch: repo.config.branch,
      },
    })
  }
}

impl StateResource for Builder {
  type ListItem = BuilderListItem;
  type Config = BuilderConfig;
  type Info = ();
  type QuerySpecifics = BuilderQuerySpecifics;

  fn name() -> &'static str {
    "builder"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Builder
  }

  async fn coll() -> &'static Collection<Builder> {
    &db_client().await.builders
  }

  async fn to_list_item(
    builder: Builder,
  ) -> anyhow::Result<BuilderListItem> {
    let (provider, instance_type) = match builder.config {
      BuilderConfig::Server(config) => {
        ("server".to_string(), Some(config.server_id))
      }
      BuilderConfig::Aws(config) => {
        ("aws ec2".to_string(), Some(config.instance_type))
      }
    };

    Ok(BuilderListItem {
      name: builder.name,
      created_at: ObjectId::from_str(&builder.id)?
        .timestamp()
        .timestamp_millis(),
      id: builder.id,
      tags: builder.tags,
      resource_type: ResourceTargetVariant::Builder,
      info: BuilderListItemInfo {
        provider,
        instance_type,
      },
    })
  }
}

impl StateResource for Alerter {
  type ListItem = AlerterListItem;
  type Config = AlerterConfig;
  type Info = AlerterInfo;
  type QuerySpecifics = AlerterQuerySpecifics;

  fn name() -> &'static str {
    "alerter"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Alerter
  }

  async fn coll() -> &'static Collection<Alerter> {
    &db_client().await.alerters
  }

  async fn to_list_item(
    alerter: Alerter,
  ) -> anyhow::Result<AlerterListItem> {
    let (alerter_type, enabled) = match alerter.config {
      AlerterConfig::Custom(config) => ("custom", config.enabled),
      AlerterConfig::Slack(config) => ("slack", config.enabled),
    };
    Ok(AlerterListItem {
      name: alerter.name,
      created_at: ObjectId::from_str(&alerter.id)?
        .timestamp()
        .timestamp_millis(),
      id: alerter.id,
      tags: alerter.tags,
      resource_type: ResourceTargetVariant::Alerter,
      info: AlerterListItemInfo {
        alerter_type: alerter_type.to_string(),
        is_default: alerter.info.is_default,
        enabled,
      },
    })
  }
}

impl StateResource for Procedure {
  type ListItem = ProcedureListItem;
  type Config = ProcedureConfig;
  type Info = ();
  type QuerySpecifics = ProcedureQuerySpecifics;

  fn name() -> &'static str {
    "procedure"
  }

  fn resource_target_variant() -> ResourceTargetVariant {
    ResourceTargetVariant::Procedure
  }

  async fn coll() -> &'static Collection<Procedure> {
    &db_client().await.procedures
  }

  async fn to_list_item(
    procedure: Procedure,
  ) -> anyhow::Result<ProcedureListItem> {
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
}

#[instrument(level = "debug")]
pub async fn get_user_permission_on_resource(
  user_id: &str,
  resource_variant: ResourceTargetVariant,
  resource_id: &str,
) -> anyhow::Result<PermissionLevel> {
  let permission = find_collect(
    &db_client().await.permissions,
    doc! {
      "$or": user_target_query(user_id).await?,
      "resource_target.type": resource_variant.as_ref(),
      "resource_target.id": resource_id
    },
    None,
  )
  .await
  .context("failed to query db for permissions")?
  .into_iter()
  // get the max permission user has between personal / any user groups
  .fold(PermissionLevel::None, |level, permission| {
    if permission.level > level {
      permission.level
    } else {
      level
    }
  });
  Ok(permission)
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

#[instrument(level = "debug")]
pub async fn get_resource_ids_for_non_admin(
  user_id: &str,
  resource_type: ResourceTargetVariant,
) -> anyhow::Result<Vec<String>> {
  let permissions = find_collect(
    &db_client().await.permissions,
    doc! {
      "$or": user_target_query(user_id).await?,
      "resource_target.type": resource_type.as_ref(),
      "level": { "$in": ["Read", "Execute", "Update"] }
    },
    None,
  )
  .await
  .context("failed to query permissions on db")?
  .into_iter()
  .map(|p| p.resource_target.extract_variant_id().1.to_string())
  // collect into hashset first to remove any duplicates
  .collect::<HashSet<_>>();
  Ok(permissions.into_iter().collect())
}

#[instrument(level = "debug")]
async fn user_target_query(
  user_id: &str,
) -> anyhow::Result<Vec<Document>> {
  let mut user_target_query = vec![
    doc! { "user_target.type": "User", "user_target.id": user_id },
  ];
  let user_groups = get_user_user_group_ids(user_id)
    .await?
    .into_iter()
    .map(|ug_id| {
      doc! {
        "user_target.type": "UserGroup", "user_target.id": ug_id,
      }
    });
  user_target_query.extend(user_groups);
  Ok(user_target_query)
}

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
