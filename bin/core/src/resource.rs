use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_types::{
    entities::{
        alerter::Alerter, build::Build, builder::Builder, deployment::Deployment, repo::Repo,
        server::Server, PermissionLevel,
    },
    permissioned::Permissioned,
    requests::read::{BuildListItem, DeploymentListItem, RepoListItem, ServerListItem},
};
use mungos::{
    mongodb::bson::{doc, Document},
    AggStage::*,
    Collection, Indexed,
};
use serde::Serialize;

use crate::{auth::RequestUser, state::State};

#[async_trait]
pub trait Resource<T: Indexed + Send + Unpin + Permissioned> {
    type ListItem: Serialize + Send;

    fn name() -> &'static str;
    fn coll(&self) -> &Collection<T>;
    async fn to_list_item(&self, resource: T) -> Self::ListItem;

    async fn get_resource(&self, id: &str) -> anyhow::Result<T> {
        self.coll()
            .find_one_by_id(id)
            .await
            .context(format!("failed to get {} from db", Self::name()))?
            .context(format!("did not find any {} with id {id}", Self::name()))
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

    async fn get_resource_ids_for_non_admin(&self, user_id: &str) -> anyhow::Result<Vec<String>> {
        self.coll()
            .aggregate_collect(
                [
                    Match(doc! {
                        format!("permissions.{}", user_id): { "$in": ["update", "execute", "read"] }
                    }),
                    Project(doc! { "_id": 1 }),
                ],
                None,
            )
            .await
            .context(format!(
                "failed to get {} ids for non admin | aggregation",
                Self::name()
            ))?
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
            .context(format!(
                "failed to get {} ids for non admin | extract id from document",
                Self::name()
            ))
    }

    async fn list_resources_for_user(
        &self,
        user: &RequestUser,
        query: Option<Document>,
    ) -> anyhow::Result<Vec<Self::ListItem>> {
        let mut query = query.unwrap_or_default();
        if !user.is_admin {
            query.insert(
                format!("permissions.{}", user.id),
                doc! { "$in": ["read", "execute", "update"] },
            );
        }
        let list = self
            .coll()
            .get_some(query, None)
            .await
            .context(format!("failed to pull {}s from mongo", Self::name()))?
            .into_iter()
            .map(|resource| self.to_list_item(resource));

        let list = join_all(list).await;

        Ok(list)
    }
}

#[async_trait]
impl Resource<Server> for State {
    type ListItem = ServerListItem;

    fn name() -> &'static str {
        "server"
    }

    fn coll(&self) -> &Collection<Server> {
        &self.db.servers
    }

    async fn to_list_item(&self, server: Server) -> ServerListItem {
        let status = self.server_status_cache.get(&server.id).await;
        ServerListItem {
            id: server.id,
            name: server.name,
            tags: server.tags,
            status: status.map(|s| s.status).unwrap_or_default(),
        }
    }
}

#[async_trait]
impl Resource<Deployment> for State {
    type ListItem = DeploymentListItem;

    fn name() -> &'static str {
        "deployment"
    }

    fn coll(&self) -> &Collection<Deployment> {
        &self.db.deployments
    }

    async fn to_list_item(&self, deployment: Deployment) -> DeploymentListItem {
        let status = self.deployment_status_cache.get(&deployment.id).await;
        DeploymentListItem {
            id: deployment.id,
            name: deployment.name,
            tags: deployment.tags,
            state: status.as_ref().map(|s| s.state).unwrap_or_default(),
            status: status
                .as_ref()
                .and_then(|s| s.container.as_ref().and_then(|c| c.status.to_owned())),
            image: String::new(),
            version: String::new(),
        }
    }
}

#[async_trait]
impl Resource<Build> for State {
    type ListItem = BuildListItem;

    fn name() -> &'static str {
        "build"
    }

    fn coll(&self) -> &Collection<Build> {
        &self.db.builds
    }

    async fn to_list_item(&self, build: Build) -> BuildListItem {
        BuildListItem {
            id: build.id,
            name: build.name,
            last_built_at: build.last_built_at,
            version: build.config.version,
            tags: build.tags,
        }
    }
}

#[async_trait]
impl Resource<Repo> for State {
    type ListItem = RepoListItem;

    fn name() -> &'static str {
        "repo"
    }

    fn coll(&self) -> &Collection<Repo> {
        &self.db.repos
    }

    async fn to_list_item(&self, repo: Repo) -> RepoListItem {
        RepoListItem {
            id: repo.id,
            name: repo.name,
            last_pulled_at: repo.last_pulled_at,
            tags: repo.tags,
        }
    }
}

#[async_trait]
impl Resource<Builder> for State {
    type ListItem = Builder;

    fn name() -> &'static str {
        "builder"
    }

    fn coll(&self) -> &Collection<Builder> {
        &self.db.builders
    }

    async fn to_list_item(&self, builder: Builder) -> Builder {
        builder
    }
}

#[async_trait]
impl Resource<Alerter> for State {
    type ListItem = Alerter;

    fn name() -> &'static str {
        "alerter"
    }

    fn coll(&self) -> &Collection<Alerter> {
        &self.db.alerters
    }

    async fn to_list_item(&self, alerter: Alerter) -> Alerter {
        alerter
    }
}
