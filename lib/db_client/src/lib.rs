use std::time::Duration;

use anyhow::{anyhow, Context};
use collections::{
    actions_collection, builds_collection, deployments_collection, groups_collection,
    procedures_collection, server_stats_collection, servers_collection, updates_collection,
    users_collection,
};
use mungos::{Collection, Mungos};
use types::{
    Action, Build, Deployment, Group, MongoConfig, PermissionLevel, Procedure, Server,
    SystemStatsRecord, Update, User,
};

mod collections;

pub struct DbClient {
    pub users: Collection<User>,
    pub servers: Collection<Server>,
    pub deployments: Collection<Deployment>,
    pub builds: Collection<Build>,
    pub procedures: Collection<Procedure>,
    pub actions: Collection<Action>,
    pub groups: Collection<Group>,
    pub updates: Collection<Update>,
    pub stats: Collection<SystemStatsRecord>,
}

impl DbClient {
    pub async fn new(config: MongoConfig) -> DbClient {
        let db_name = &config.db_name;
        let mungos = Mungos::builder()
            .uri(&config.uri)
            .app_name(&config.app_name)
            .build()
            .await
            .expect("failed to initialize mungos");
        DbClient {
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
            actions: actions_collection(&mungos, db_name)
                .await
                .expect("failed to make actions collection"),
            groups: groups_collection(&mungos, db_name)
                .await
                .expect("failed to make groups collection"),
            stats: server_stats_collection(&mungos, db_name)
                .await
                .expect("failed to make stats collection"),
        }
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

    pub async fn get_procedure(&self, procedure_id: &str) -> anyhow::Result<Procedure> {
        let procedure = self
            .procedures
            .find_one_by_id(procedure_id)
            .await
            .context(format!(
                "failed at mongo query for procedure {procedure_id}"
            ))?
            .ok_or(anyhow!("procedure at {procedure_id} doesn't exist"))?;
        Ok(procedure)
    }

    pub async fn get_user_permission_on_procedure(
        &self,
        user_id: &str,
        procedure_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let permissions = *self
            .get_procedure(procedure_id)
            .await?
            .permissions
            .get(user_id)
            .unwrap_or_default();
        Ok(permissions)
    }

    pub async fn get_group(&self, group_id: &str) -> anyhow::Result<Group> {
        let group = self
            .groups
            .find_one_by_id(group_id)
            .await
            .context(format!("failed at mongo query for group {group_id}"))?
            .ok_or(anyhow!("group at {group_id} doesn't exist"))?;
        Ok(group)
    }

    pub async fn get_user_permission_on_group(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let permissions = *self
            .get_group(group_id)
            .await?
            .permissions
            .get(user_id)
            .unwrap_or_default();
        Ok(permissions)
    }
}
