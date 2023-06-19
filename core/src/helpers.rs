use std::collections::HashMap;

use anyhow::{anyhow, Context};
use monitor_types::{
    busy::Busy,
    entities::{
        build::Build, deployment::Deployment, server::Server, update::Update, user::User,
        PermissionLevel,
    },
    permissioned::Permissioned,
};
use periphery_client::PeripheryClient;
use tokio::sync::RwLock;

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_user(&self, user_id: &str) -> anyhow::Result<User> {
        self.db
            .users
            .find_one_by_id(user_id)
            .await?
            .context(format!("no user exists with id {user_id}"))
    }

    pub async fn get_server(&self, server_id: &str) -> anyhow::Result<Server> {
        self.db
            .servers
            .find_one_by_id(server_id)
            .await?
            .context(format!("did not find any server with id {server_id}"))
    }

    pub async fn get_server_check_permissions(
        &self,
        server_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Server> {
        let server = self
            .db
            .servers
            .find_one_by_id(server_id)
            .await?
            .context(format!("did not find any server with id {server_id}"))?;
        let permissions = server.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(server)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this server"
            ))
        }
    }

    pub async fn get_user_permission_on_server(
        &self,
        user_id: &str,
        server_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let server = self.get_server(server_id).await?;
        Ok(server.get_user_permissions(user_id))
    }

    pub async fn get_deployment(&self, deployment_id: &str) -> anyhow::Result<Deployment> {
        self.db
            .deployments
            .find_one_by_id(deployment_id)
            .await?
            .context(format!(
                "did not find any deployment with id {deployment_id}"
            ))
    }

    pub async fn get_deployment_check_permissions(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Deployment> {
        let deployment = self
            .db
            .deployments
            .find_one_by_id(deployment_id)
            .await?
            .context(format!(
                "did not find any deployment with id {deployment_id}"
            ))?;
        let permissions = deployment.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(deployment)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this deployment"
            ))
        }
    }

    pub async fn get_user_permission_on_deployment(
        &self,
        user_id: &str,
        deployment_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let deployment = self.get_deployment(deployment_id).await?;
        Ok(deployment.get_user_permissions(user_id))
    }

    pub async fn get_build(&self, build_id: &str) -> anyhow::Result<Build> {
        self.db
            .builds
            .find_one_by_id(build_id)
            .await?
            .context(format!("did not find any build with id {build_id}"))
    }

    pub async fn get_build_check_permissions(
        &self,
        build_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Build> {
        let build = self
            .db
            .builds
            .find_one_by_id(build_id)
            .await?
            .context(format!("did not find any build with id {build_id}"))?;
        let permissions = build.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(build)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this build"
            ))
        }
    }

    pub async fn get_user_permission_on_build(
        &self,
        user_id: &str,
        build_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let build = self.get_build(build_id).await?;
        Ok(build.get_user_permissions(user_id))
    }

    pub async fn send_update(&self, update: Update) -> anyhow::Result<()> {
        self.update.sender.lock().await.send(update)?;
        Ok(())
    }

    pub async fn add_update(&self, mut update: Update) -> anyhow::Result<String> {
        update.id = self
            .db
            .updates
            .create_one(update.clone())
            .await
            .context("failed to insert update into db")?;
        let id = update.id.clone();
        let _ = self.send_update(update).await;
        Ok(id)
    }

    pub async fn update_update(&self, mut update: Update) -> anyhow::Result<()> {
        let mut update_id = String::new();
        std::mem::swap(&mut update.id, &mut update_id);
        self.db
            .updates
            .update_one(&update_id, mungos::Update::Regular(update.clone()))
            .await
            .context("failed to update the update on db. the update build process was deleted")?;
        std::mem::swap(&mut update.id, &mut update_id);
        let _ = self.send_update(update).await;
        Ok(())
    }

    pub fn periphery_client(&self, server: &Server) -> PeripheryClient {
        PeripheryClient::new(&server.config.address, &self.config.passkey)
    }
}

#[derive(Default)]
pub struct Cache<T: Clone + Default> {
    cache: RwLock<HashMap<String, T>>,
}

impl<T: Clone + Default> Cache<T> {
    pub async fn get(&self, key: &str) -> Option<T> {
        self.cache.read().await.get(key).cloned()
    }

    pub async fn get_or_default(&self, key: String) -> T {
        let mut cache = self.cache.write().await;
        cache.entry(key).or_default().clone()
    }

    pub async fn _get_list(&self, filter: Option<impl Fn(&String, &T) -> bool>) -> Vec<T> {
        let cache = self.cache.read().await;
        match filter {
            Some(filter) => cache
                .iter()
                .filter(|(k, v)| filter(k, v))
                .map(|(_, e)| e.clone())
                .collect(),
            None => cache.iter().map(|(_, e)| e.clone()).collect(),
        }
    }

    pub async fn insert(&self, key: String, val: T) {
        self.cache.write().await.insert(key, val);
    }

    pub async fn update_entry(&self, key: String, handler: impl Fn(&mut T)) {
        let mut cache = self.cache.write().await;
        handler(cache.entry(key).or_default());
    }

    pub async fn clear(&self) {
        self.cache.write().await.clear();
    }

    pub async fn remove(&self, key: &str) {
        self.cache.write().await.remove(key);
    }
}

impl<T: Clone + Default + Busy> Cache<T> {
    pub async fn busy(&self, id: &str) -> bool {
        match self.get(id).await {
            Some(state) => state.busy(),
            None => false,
        }
    }
}
