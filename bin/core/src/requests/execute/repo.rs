use anyhow::anyhow;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        repo::Repo,
        server::Server,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel,
    },
    monitor_timestamp, optional_string,
    requests::execute::*,
};
use mungos::mongodb::bson::doc;
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, resource::StateResource, state::State};

#[async_trait]
impl Resolve<CloneRepo, RequestUser> for State {
    async fn resolve(
        &self,
        CloneRepo { id }: CloneRepo,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let repo: Repo = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Execute)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            if repo.config.server_id.is_empty() {
                return Err(anyhow!("repo has no server attached"));
            }

            let server: Server = self.get_resource(&repo.config.server_id).await?;

            let mut update = Update {
                operation: Operation::CloneRepo,
                target: ResourceTarget::Repo(repo.id.clone()),
                start_ts,
                status: UpdateStatus::InProgress,
                operator: user.id.clone(),
                success: true,
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            let logs = match self
                .periphery_client(&server)
                .request(requests::CloneRepo {
                    args: (&repo).into(),
                })
                .await
            {
                Ok(logs) => logs,
                Err(e) => vec![Log::error("clone repo", format!("{e:#?}"))],
            };

            update.logs.extend(logs);
            update.finalize();

            if update.success {
                let res = self
                    .db
                    .repos
                    .update_one(
                        &repo.id,
                        mungos::Update::Set(doc! { "info.last_pulled_at": monitor_timestamp() }),
                    )
                    .await;
                if let Err(e) = res {
                    warn!(
                        "failed to update repo last_pulled_at | repo id: {} | {e:#?}",
                        repo.id
                    );
                }
            }

            self.update_update(update.clone()).await?;
            Ok(update)
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(&id, |entry| {
                entry.cloning = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(id, |entry| {
                entry.cloning = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<PullRepo, RequestUser> for State {
    async fn resolve(
        &self,
        PullRepo { id }: PullRepo,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let repo: Repo = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            if repo.config.server_id.is_empty() {
                return Err(anyhow!("repo has no server attached"));
            }

            let server: Server = self.get_resource(&repo.config.server_id).await?;

            let mut update = Update {
                operation: Operation::PullRepo,
                target: ResourceTarget::Repo(repo.id.clone()),
                start_ts,
                status: UpdateStatus::InProgress,
                operator: user.id.clone(),
                success: true,
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            let logs = match self
                .periphery_client(&server)
                .request(requests::PullRepo {
                    name: repo.name,
                    branch: optional_string(&repo.config.branch),
                    on_pull: repo.config.on_pull.into_option(),
                })
                .await
            {
                Ok(logs) => logs,
                Err(e) => vec![Log::error("pull repo", format!("{e:#?}"))],
            };

            update.logs.extend(logs);

            update.finalize();

            if update.success {
                let res = self
                    .db
                    .repos
                    .update_one(
                        &repo.id,
                        mungos::Update::Set(doc! { "last_pulled_at": monitor_timestamp() }),
                    )
                    .await;
                if let Err(e) = res {
                    warn!(
                        "failed to update repo last_pulled_at | repo id: {} | {e:#?}",
                        repo.id
                    );
                }
            }

            self.update_update(update.clone()).await?;
            Ok(update)
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(id.clone(), |entry| {
                entry.pulling = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(id, |entry| {
                entry.pulling = false;
            })
            .await;

        res
    }
}
