use std::time::Duration;

use anyhow::{anyhow, Context};
use monitor_types::{
    entities::{
        alerter::Alerter,
        build::Build,
        builder::Builder,
        deployment::{Deployment, DockerContainerState},
        repo::Repo,
        server::{Server, ServerStatus},
        tag::CustomTag,
        update::{ResourceTarget, Update},
        user::User,
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    permissioned::Permissioned,
};
use periphery_client::{requests, PeripheryClient};
use rand::{thread_rng, Rng};

use crate::{auth::RequestUser, state::State};

pub mod cache;
pub mod channel;
pub mod alert;
pub mod db;

pub fn empty_or_only_spaces(word: &str) -> bool {
    if word.is_empty() {
        return true;
    }
    for char in word.chars() {
        if char != ' ' {
            return false;
        }
    }
    true
}

pub fn random_duration(min_ms: u64, max_ms: u64) -> Duration {
    Duration::from_millis(thread_rng().gen_range(min_ms..max_ms))
}

pub fn make_update(
    target: impl Into<ResourceTarget>,
    operation: Operation,
    user: &RequestUser,
) -> Update {
    Update {
        start_ts: monitor_timestamp(),
        target: target.into(),
        operation,
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    }
}

impl State {
    
    // USER

    pub async fn get_user(&self, user_id: &str) -> anyhow::Result<User> {
        self.db
            .users
            .find_one_by_id(user_id)
            .await?
            .context(format!("no user exists with id {user_id}"))
    }

    // SERVER

    pub async fn get_server(&self, server_id: &str) -> anyhow::Result<Server> {
        self.db
            .servers
            .find_one_by_id(server_id)
            .await
            .context("failed to get server from db")?
            .context(format!("did not find any server with id {server_id}"))
    }

    pub async fn get_server_with_status(
        &self,
        server_id: &str,
    ) -> anyhow::Result<(Server, ServerStatus)> {
        let server = self.get_server(server_id).await?;
        if !server.config.enabled {
            return Ok((server, ServerStatus::Disabled));
        }
        let status = match self
            .periphery_client(&server)
            .request(requests::GetHealth {})
            .await
        {
            Ok(_) => ServerStatus::Ok,
            Err(_) => ServerStatus::NotOk,
        };
        Ok((server, status))
    }

    pub async fn get_server_check_permissions(
        &self,
        server_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Server> {
        let server = self.get_server(server_id).await?;
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

    // DEPLOYMENT

    pub async fn get_deployment(&self, deployment_id: &str) -> anyhow::Result<Deployment> {
        self.db
            .deployments
            .find_one_by_id(deployment_id)
            .await
            .context("failed to get deployment from db")?
            .context(format!(
                "did not find any deployment with id {deployment_id}"
            ))
    }

    pub async fn get_deployment_state(
        &self,
        deployment: &Deployment,
    ) -> anyhow::Result<DockerContainerState> {
        if deployment.config.server_id.is_empty() {
            return Ok(DockerContainerState::NotDeployed);
        }
        let (server, status) = self
            .get_server_with_status(&deployment.config.server_id)
            .await?;
        if status != ServerStatus::Ok {
            return Ok(DockerContainerState::Unknown);
        }
        let container = self
            .periphery_client(&server)
            .request(requests::GetContainerList {})
            .await?
            .into_iter()
            .find(|container| container.name == deployment.name);

        let state = match container {
            Some(container) => container.state,
            None => DockerContainerState::NotDeployed,
        };

        Ok(state)
    }

    pub async fn get_deployment_check_permissions(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Deployment> {
        let deployment = self.get_deployment(deployment_id).await?;
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

    // BUILD

    pub async fn get_build(&self, build_id: &str) -> anyhow::Result<Build> {
        self.db
            .builds
            .find_one_by_id(build_id)
            .await
            .context("failed to get build from db")?
            .context(format!("did not find any build with id {build_id}"))
    }

    pub async fn get_build_check_permissions(
        &self,
        build_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Build> {
        let build = self.get_build(build_id).await?;
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

    // BUILDER

    pub async fn get_builder(&self, builder_id: &str) -> anyhow::Result<Builder> {
        self.db
            .builders
            .find_one_by_id(builder_id)
            .await
            .context("failed to get builder from db")?
            .context(format!("did not find any builder with id {builder_id}"))
    }

    pub async fn get_builder_check_permissions(
        &self,
        builder_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Builder> {
        let builder = self.get_builder(builder_id).await?;
        let permissions = builder.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(builder)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this builder"
            ))
        }
    }

    pub async fn get_user_permission_on_builder(
        &self,
        user_id: &str,
        builder_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let builder = self.get_builder(builder_id).await?;
        Ok(builder.get_user_permissions(user_id))
    }

    // REPO

    pub async fn get_repo(&self, repo_id: &str) -> anyhow::Result<Repo> {
        self.db
            .repos
            .find_one_by_id(repo_id)
            .await
            .context("failed to get repo from db")?
            .context(format!("did not find any repo with id {repo_id}"))
    }

    pub async fn get_repo_check_permissions(
        &self,
        repo_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Repo> {
        let repo = self.get_repo(repo_id).await?;
        let permissions = repo.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(repo)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this repo"
            ))
        }
    }

    pub async fn get_user_permission_on_repo(
        &self,
        user_id: &str,
        repo_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let repo = self.get_repo(repo_id).await?;
        Ok(repo.get_user_permissions(user_id))
    }

    // ALERTER

    pub async fn get_alerter(&self, alerter_id: &str) -> anyhow::Result<Alerter> {
        self.db
            .alerters
            .find_one_by_id(alerter_id)
            .await
            .context("failed to get alerter from mongo")?
            .context(format!("did not find any alerter with id {alerter_id}"))
    }

    pub async fn get_alerter_check_permissions(
        &self,
        alerter_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Alerter> {
        let alerter = self.get_alerter(alerter_id).await?;
        let permissions = alerter.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(alerter)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this alerter"
            ))
        }
    }

    pub async fn get_user_permission_on_alerter(
        &self,
        user_id: &str,
        alerter_id: &str,
    ) -> anyhow::Result<PermissionLevel> {
        let alerter = self.get_alerter(alerter_id).await?;
        Ok(alerter.get_user_permissions(user_id))
    }

    // TAG

    pub async fn get_tag(&self, tag_id: &str) -> anyhow::Result<CustomTag> {
        self.db
            .tags
            .find_one_by_id(tag_id)
            .await
            .context("failed to get tag from db")?
            .context(format!("did not find any tag with id {tag_id}"))
    }

    pub async fn get_tag_check_owner(
        &self,
        tag_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<CustomTag> {
        let tag = self.get_tag(tag_id).await?;
        if !user.is_admin && tag.owner != user.id {
            return Err(anyhow!("user must be tag owner or admin"));
        }
        Ok(tag)
    }

    // UPDATE

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
            .update_one(&update_id, mungos::Update::Regular(&update))
            .await
            .context("failed to update the update on db. the update build process was deleted")?;
        std::mem::swap(&mut update.id, &mut update_id);
        let _ = self.send_update(update).await;
        Ok(())
    }

    //

    pub fn periphery_client(&self, server: &Server) -> PeripheryClient {
        PeripheryClient::new(&server.config.address, &self.config.passkey)
    }
}
