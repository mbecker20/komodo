use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Context};
use axum::Extension;
use collections::{
    builds_collection, deployments_collection, procedures_collection, servers_collection,
    updates_collection, users_collection,
};
use mungos::{Collection, Mungos};
use types::{Build, Deployment, MongoConfig, PermissionLevel, Procedure, Server, Update, User};

mod collections;

pub type DbExtension = Extension<Arc<DbClient>>;

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub procedures: Collection<Procedure>,
    pub updates: Collection<Update>,
}

impl DbClient {
    pub async fn extension(config: MongoConfig) -> DbExtension {
        let db_name = &config.db_name;
        let mungos = Mungos::new(&config.uri, &config.app_name, Duration::from_secs(3), None)
            .await
            .expect("failed to initialize mungos");
        let client = DbClient {
            users: users_collection(&mungos, db_name)
                .await
                .expect("failed to make users collection"),
            servers: servers_collection(&mungos, db_name)
                .await
                .expect("failed to make servers collection"),
            deployments: deployments_collection(&mungos, db_name)
                .await
                .expect("failed to make deployments collection"),
            builds: builds_collection(&mungos, db_name)
                .await
                .expect("failed to make builds collection"),
            updates: updates_collection(&mungos, db_name)
                .await
                .expect("failed to make updates collection"),
            procedures: procedures_collection(&mungos, db_name)
                .await
                .expect("failed to make procedures collection"),
        };
        Extension(Arc::new(client))
    }

    pub async fn get_user(&self, user_id: &str) -> anyhow::Result<User> {
        let user = self
            .users
            .find_one_by_id(user_id)
            .await
            .context(format!("failed at mongo query for user {user_id}"))?
            .ok_or(anyhow!("user at {user_id} doesn't exist"))?;
        Ok(user)
    }

    pub async fn get_deployment(&self, deployment_id: &str) -> anyhow::Result<Deployment> {
        let deployment = self
            .deployments
            .find_one_by_id(deployment_id)
            .await
            .context(format!(
                "failed at mongo query for deployment {deployment_id}"
            ))?
            .ok_or(anyhow!("deployment at {deployment_id} doesn't exist"))?;
        Ok(deployment)
    }

    pub async fn get_user_permission_on_deployment(
        &self,
        user_id: &str,
        deployment_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let permissions = *self
            .get_deployment(deployment_id)
            .await?
            .permissions
            .get(user_id)
            .unwrap_or_default();
        Ok(permissions)
    }

    pub async fn get_build(&self, build_id: &str) -> anyhow::Result<Build> {
        let build = self
            .builds
            .find_one_by_id(build_id)
            .await
            .context(format!("failed at mongo query for build {build_id}"))?
            .ok_or(anyhow!("build at {build_id} doesn't exist"))?;
        Ok(build)
    }

    pub async fn get_user_permission_on_build(
        &self,
        user_id: &str,
        build_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let permissions = *self
            .get_build(build_id)
            .await?
            .permissions
            .get(user_id)
            .unwrap_or_default();
        Ok(permissions)
    }

    pub async fn get_server(&self, server_id: &str) -> anyhow::Result<Server> {
        let server = self
            .servers
            .find_one_by_id(server_id)
            .await
            .context(format!("failed at mongo query for server {server_id}"))?
            .ok_or(anyhow!("server at {server_id} doesn't exist"))?;
        Ok(server)
    }

    pub async fn get_user_permission_on_server(
        &self,
        user_id: &str,
        server_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let permissions = *self
            .get_server(server_id)
            .await?
            .permissions
            .get(user_id)
            .unwrap_or_default();
        Ok(permissions)
    }
}
