use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        alerter::Alerter, build::Build, builder::Builder, deployment::Deployment, repo::Repo,
        server::Server, PermissionLevel,
    },
    permissioned::Permissioned,
};
use mungos::{
    mongodb::bson::{doc, Document},
    AggStage::*,
    Collection, Indexed,
};

use crate::{auth::RequestUser, state::State};

#[async_trait]
pub trait Resource<T: Indexed + Send + Unpin + Permissioned> {
    fn name() -> &'static str;
    fn coll(&self) -> &Collection<T>;

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
    ) -> anyhow::Result<Vec<T>> {
        let mut query = query.unwrap_or_default();
        if !user.is_admin {
            query.insert(
                format!("permissions.{}", user.id),
                doc! { "$in": ["read", "execute", "update"] },
            );
        }
        self.coll()
            .get_some(query, None)
            .await
            .context(format!("failed to pull {}s from mongo", Self::name()))
    }
}

impl Resource<Server> for State {
    fn name() -> &'static str {
        "server"
    }

    fn coll(&self) -> &Collection<Server> {
        &self.db.servers
    }
}

impl Resource<Deployment> for State {
    fn name() -> &'static str {
        "deployment"
    }

    fn coll(&self) -> &Collection<Deployment> {
        &self.db.deployments
    }
}

impl Resource<Build> for State {
    fn name() -> &'static str {
        "build"
    }

    fn coll(&self) -> &Collection<Build> {
        &self.db.builds
    }
}

impl Resource<Builder> for State {
    fn name() -> &'static str {
        "builder"
    }

    fn coll(&self) -> &Collection<Builder> {
        &self.db.builders
    }
}

impl Resource<Repo> for State {
    fn name() -> &'static str {
        "repo"
    }

    fn coll(&self) -> &Collection<Repo> {
        &self.db.repos
    }
}

impl Resource<Alerter> for State {
    fn name() -> &'static str {
        "alerter"
    }

    fn coll(&self) -> &Collection<Alerter> {
        &self.db.alerters
    }
}
