use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_client::entities::{
  alerter::{
    Alerter, AlerterConfig, AlerterListItem, AlerterListItemInfo,
  },
  build::{Build, BuildListItem, BuildListItemInfo},
  builder::{
    Builder, BuilderConfig, BuilderListItem, BuilderListItemInfo,
  },
  deployment::{
    Deployment, DeploymentImage, DeploymentListItem,
    DeploymentListItemInfo,
  },
  permission::PermissionLevel,
  procedure::{Procedure, ProcedureListItem, ProcedureListItemInfo},
  repo::{Repo, RepoInfo, RepoListItem},
  server::{Server, ServerListItem, ServerListItemInfo},
  update::ResourceTargetVariant,
  user::User,
};
use mungos::{
  by_id::{find_one_by_id, update_one_by_id},
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
  },
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{db::db_client, state::State};

use super::cache::{deployment_status_cache, server_status_cache};

#[async_trait]
pub trait StateResource<
  T: Send + Sync + Unpin + Serialize + DeserializeOwned,
>
{
  type ListItem: Serialize + Send;

  fn name() -> &'static str;
  fn resource_target_variant(&self) -> ResourceTargetVariant;
  async fn coll(&self) -> &Collection<T>;
  async fn to_list_item(
    &self,
    resource: T,
  ) -> anyhow::Result<Self::ListItem>;

  async fn get_resource(&self, id: &str) -> anyhow::Result<T> {
    find_one_by_id(self.coll().await, id)
      .await
      .context("failed to query db for resource")?
      .with_context(|| {
        format!("did not find any {} with id {id}", Self::name())
      })
  }

  async fn get_resource_check_permissions(
    &self,
    id: &str,
    user: &User,
    permission_level: PermissionLevel,
  ) -> anyhow::Result<T> {
    let resource = self.get_resource(id).await?;
    let permissions =
      self.get_user_permission_on_resource(&user.id, id).await?;
    if user.admin || permissions >= permission_level {
      Ok(resource)
    } else {
      Err(anyhow!(
        "user does not have required permissions on this {}",
        Self::name()
      ))
    }
  }

  async fn get_user_permission_on_resource(
    &self,
    user_id: &str,
    resource_id: &str,
  ) -> anyhow::Result<PermissionLevel> {
    get_user_permission_on_resource(
      user_id,
      self.resource_target_variant(),
      resource_id,
    )
    .await
  }

  async fn get_resource_ids_for_non_admin(
    &self,
    user_id: &str,
  ) -> anyhow::Result<Vec<String>> {
    let permissions = find_collect(
      &db_client().await.permissions,
      doc! { "user_id": user_id, "target.type": self.resource_target_variant().as_ref() },
      None,
    )
    .await
    .context("failed to query permissions on db")?
    .into_iter()
    .map(|p| p.target.extract_variant_id().1.to_string())
    .collect();
    Ok(permissions)
  }

  async fn list_resources_for_user(
    &self,
    mut filters: Document,
    user: &User,
  ) -> anyhow::Result<Vec<Self::ListItem>> {
    if !user.admin {
      let ids = self
        .get_resource_ids_for_non_admin(&user.id)
        .await?
        .into_iter()
        .flat_map(|id| ObjectId::from_str(&id))
        .collect::<Vec<_>>();
      filters.insert("_id", doc! { "$in": ids });
    }
    let list = find_collect(self.coll().await, filters, None)
      .await
      .with_context(|| {
        format!("failed to pull {}s from mongo", Self::name())
      })?
      .into_iter()
      .map(|resource| self.to_list_item(resource));

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
    &self,
    id: &str,
    description: &str,
    user: &User,
  ) -> anyhow::Result<()> {
    self
      .get_resource_check_permissions(
        id,
        user,
        PermissionLevel::Update,
      )
      .await?;
    self
      .coll()
      .await
      .update_one(
        doc! { "_id": ObjectId::from_str(id)? },
        doc! { "$set": { "description": description } },
        None,
      )
      .await?;
    Ok(())
  }

  async fn update_tags_on_resource(
    &self,
    id: &str,
    tags: Vec<String>,
  ) -> anyhow::Result<()> {
    update_one_by_id(
      self.coll().await,
      id,
      doc! { "$set": { "tags": tags } },
      None,
    )
    .await?;
    Ok(())
  }

  async fn remove_tag_from_resources(
    &self,
    tag_id: &str,
  ) -> anyhow::Result<()> {
    self
      .coll()
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

#[async_trait]
impl StateResource<Server> for State {
  type ListItem = ServerListItem;

  fn name() -> &'static str {
    "server"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Server
  }

  async fn coll(&self) -> &Collection<Server> {
    &db_client().await.servers
  }

  async fn to_list_item(
    &self,
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
        send_temp_alerts: server.config.send_temp_alerts,
      },
    })
  }
}

#[async_trait]
impl StateResource<Deployment> for State {
  type ListItem = DeploymentListItem;

  fn name() -> &'static str {
    "deployment"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Deployment
  }

  async fn coll(&self) -> &Collection<Deployment> {
    &db_client().await.deployments
  }

  async fn to_list_item(
    &self,
    deployment: Deployment,
  ) -> anyhow::Result<DeploymentListItem> {
    let status = deployment_status_cache().get(&deployment.id).await;
    let (image, build_id) = match deployment.config.image {
      DeploymentImage::Build { build_id, version } => {
        let build: Build = self.get_resource(&build_id).await?;
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

#[async_trait]
impl StateResource<Build> for State {
  type ListItem = BuildListItem;

  fn name() -> &'static str {
    "build"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Build
  }

  async fn coll(&self) -> &Collection<Build> {
    &db_client().await.builds
  }

  async fn to_list_item(
    &self,
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
      },
    })
  }
}

#[async_trait]
impl StateResource<Repo> for State {
  type ListItem = RepoListItem;

  fn name() -> &'static str {
    "repo"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Repo
  }

  async fn coll(&self) -> &Collection<Repo> {
    &db_client().await.repos
  }

  async fn to_list_item(
    &self,
    repo: Repo,
  ) -> anyhow::Result<RepoListItem> {
    Ok(RepoListItem {
      name: repo.name,
      created_at: ObjectId::from_str(&repo.id)?
        .timestamp()
        .timestamp_millis(),
      id: repo.id,
      tags: repo.tags,
      resource_type: ResourceTargetVariant::Repo,
      info: RepoInfo {
        last_pulled_at: repo.info.last_pulled_at,
      },
    })
  }
}

#[async_trait]
impl StateResource<Builder> for State {
  type ListItem = BuilderListItem;

  fn name() -> &'static str {
    "builder"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Builder
  }

  async fn coll(&self) -> &Collection<Builder> {
    &db_client().await.builders
  }

  async fn to_list_item(
    &self,
    builder: Builder,
  ) -> anyhow::Result<BuilderListItem> {
    let (provider, instance_type) = match builder.config {
      BuilderConfig::Server(config) => {
        ("server".to_string(), Some(config.id))
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

#[async_trait]
impl StateResource<Alerter> for State {
  type ListItem = AlerterListItem;

  fn name() -> &'static str {
    "alerter"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Alerter
  }

  async fn coll(&self) -> &Collection<Alerter> {
    &db_client().await.alerters
  }

  async fn to_list_item(
    &self,
    alerter: Alerter,
  ) -> anyhow::Result<AlerterListItem> {
    let alerter_type = match alerter.config {
      AlerterConfig::Custom(_) => "custom",
      AlerterConfig::Slack(_) => "slack",
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
      },
    })
  }
}

#[async_trait]
impl StateResource<Procedure> for State {
  type ListItem = ProcedureListItem;

  fn name() -> &'static str {
    "procedure"
  }

  fn resource_target_variant(&self) -> ResourceTargetVariant {
    ResourceTargetVariant::Procedure
  }

  async fn coll(&self) -> &Collection<Procedure> {
    &db_client().await.procedures
  }

  async fn to_list_item(
    &self,
    procedure: Procedure,
  ) -> anyhow::Result<ProcedureListItem> {
    Ok(ProcedureListItem {
      name: procedure.name,
      created_at: ObjectId::from_str(&procedure.id)?
        .timestamp()
        .timestamp_millis(),
      id: procedure.id,
      tags: procedure.tags,
      resource_type: ResourceTargetVariant::Alerter,
      info: ProcedureListItemInfo {
        procedure_type: (&procedure.config).into(),
      },
    })
  }
}

pub async fn get_user_permission_on_resource(
  user_id: &str,
  resource_variant: ResourceTargetVariant,
  resource_id: &str,
) -> anyhow::Result<PermissionLevel> {
  let permission = db_client()
    .await
    .permissions
    .find_one(
      doc! {
        "user_id": user_id,
        "target.type": resource_variant.as_ref(),
        "target.id": resource_id
      },
      None,
    )
    .await
    .context("failed to query permissions table")?
    .map(|permission| permission.level)
    .unwrap_or_default();
  Ok(permission)
}
