use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_client::{
  entities::{
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
    repo::{Repo, RepoInfo, RepoListItem},
    server::{Server, ServerListItem, ServerListItemInfo},
    update::ResourceTargetVariant,
    PermissionLevel,
  },
  permissioned::Permissioned,
};
use mungos::{
  aggregate::aggregate_collect,
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection,
  },
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{auth::RequestUser, state::State};

#[async_trait]
pub trait StateResource<
  T: Send + Sync + Unpin + Serialize + DeserializeOwned + Permissioned,
>
{
  type ListItem: Serialize + Send;

  fn name() -> &'static str;
  fn coll(&self) -> &Collection<T>;
  async fn to_list_item(
    &self,
    resource: T,
  ) -> anyhow::Result<Self::ListItem>;

  async fn get_resource(&self, id: &str) -> anyhow::Result<T> {
    find_one_by_id(self.coll(), id)
      .await
      .context("failed to query db for resource")?
      .with_context(|| {
        format!("did not find any {} with id {id}", Self::name())
      })
  }

  async fn get_resource_check_permissions(
    &self,
    id: &str,
    user: &RequestUser,
    permission_level: PermissionLevel,
  ) -> anyhow::Result<T> {
    let resource = self.get_resource(id).await?;
    let permissions = resource.get_user_permissions(&user.id);
    if user.is_admin || permissions >= permission_level {
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
    let resource = self.get_resource(resource_id).await?;
    Ok(resource.get_user_permissions(user_id))
  }

  async fn get_resource_ids_for_non_admin(
    &self,
    user_id: &str,
  ) -> anyhow::Result<Vec<String>> {
    use mungos::aggregate::AggStage::*;
    aggregate_collect(
      self.coll(),
      [
          Match(doc! {
              format!("permissions.{}", user_id): { "$in": ["update", "execute", "read"] }
          }),
          Project(doc! { "_id": 1 }),
      ], None)
      .await
      .with_context(|| format!("failed to get {} ids for non admin | aggregation", Self::name()))?
      .into_iter()
      .map(|d| {
        let id = d
          .get("_id")
          .context("no _id field")?
          .as_object_id()
          .context("_id not ObjectId")?
          .to_string();
        anyhow::Ok(id)
      })
    .collect::<anyhow::Result<Vec<_>>>()
    .with_context(|| format!(
        "failed to get {} ids for non admin | extract id from document",
        Self::name()
    ))
  }

  async fn list_resources_for_user(
    &self,
    query: Option<Document>,
    user: &RequestUser,
  ) -> anyhow::Result<Vec<Self::ListItem>> {
    let mut query = query.unwrap_or_default();
    if !user.is_admin {
      query.insert(
        format!("permissions.{}", user.id),
        doc! { "$in": ["read", "execute", "update"] },
      );
    }
    let list = find_collect(self.coll(), query, None)
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
    user: &RequestUser,
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
      .update_one(
        doc! { "_id": ObjectId::from_str(id)? },
        doc! { "$set": { "description": description } },
        None,
      )
      .await?;
    Ok(())
  }
}

#[async_trait]
impl StateResource<Server> for State {
  type ListItem = ServerListItem;

  fn name() -> &'static str {
    "server"
  }

  fn coll(&self) -> &Collection<Server> {
    &self.db.servers
  }

  async fn to_list_item(
    &self,
    server: Server,
  ) -> anyhow::Result<ServerListItem> {
    let status = self.server_status_cache.get(&server.id).await;
    Ok(ServerListItem {
      id: server.id,
      name: server.name,
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

  fn coll(&self) -> &Collection<Deployment> {
    &self.db.deployments
  }

  async fn to_list_item(
    &self,
    deployment: Deployment,
  ) -> anyhow::Result<DeploymentListItem> {
    let status =
      self.deployment_status_cache.get(&deployment.id).await;
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
      id: deployment.id,
      name: deployment.name,
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

  fn coll(&self) -> &Collection<Build> {
    &self.db.builds
  }

  async fn to_list_item(
    &self,
    build: Build,
  ) -> anyhow::Result<BuildListItem> {
    Ok(BuildListItem {
      id: build.id,
      name: build.name,
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

  fn coll(&self) -> &Collection<Repo> {
    &self.db.repos
  }

  async fn to_list_item(
    &self,
    repo: Repo,
  ) -> anyhow::Result<RepoListItem> {
    Ok(RepoListItem {
      id: repo.id,
      name: repo.name,
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

  fn coll(&self) -> &Collection<Builder> {
    &self.db.builders
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
      id: builder.id,
      name: builder.name,
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

  fn coll(&self) -> &Collection<Alerter> {
    &self.db.alerters
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
      id: alerter.id,
      name: alerter.name,
      tags: alerter.tags,
      resource_type: ResourceTargetVariant::Alerter,
      info: AlerterListItemInfo {
        alerter_type: alerter_type.to_string(),
        is_default: alerter.info.is_default,
      },
    })
  }
}
