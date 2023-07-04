use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        repo::Repo,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    requests::{execute, write::*},
};
use mungos::mongodb::bson::{doc, to_bson};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<CreateRepo, RequestUser> for State {
    async fn resolve(
        &self,
        CreateRepo { name, config }: CreateRepo,
        user: RequestUser,
    ) -> anyhow::Result<Repo> {
        if let Some(server_id) = &config.server_id {
            if !server_id.is_empty() {
                self.get_server_check_permissions(
                        server_id,
                        &user,
                        PermissionLevel::Update,
                    )
                    .await
                    .context("cannot create repo on this server. user must have update permissions on the server.")?;
            }
        }
        let start_ts = monitor_timestamp();
        let repo = Repo {
            id: Default::default(),
            name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            config: config.into(),
        };
        let repo_id = self
            .db
            .repos
            .create_one(repo)
            .await
            .context("failed to add repo to db")?;

        let repo = self.get_repo(&repo_id).await?;

        let update = Update {
            target: ResourceTarget::Repo(repo_id),
            operation: Operation::CreateRepo,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create repo",
                    format!("created repo\nid: {}\nname: {}", repo.id, repo.name),
                ),
                Log::simple("config", format!("{:#?}", repo.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        if !repo.config.repo.is_empty() && !repo.config.server_id.is_empty() {
            let _ = self
                .resolve(
                    execute::CloneRepo {
                        id: repo.id.clone(),
                    },
                    user,
                )
                .await;
        }

        Ok(repo)
    }
}

#[async_trait]
impl Resolve<CopyRepo, RequestUser> for State {
    async fn resolve(
        &self,
        CopyRepo { name, id }: CopyRepo,
        user: RequestUser,
    ) -> anyhow::Result<Repo> {
        let Repo {
            config,
            description,
            ..
        } = self
            .get_repo_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        if !config.server_id.is_empty() {
            self.get_server_check_permissions(
                    &config.server_id,
                    &user,
                    PermissionLevel::Update,
                )
                .await
                .context("cannot create repo on this server. user must have update permissions on the server.")?;
        }
        let start_ts = monitor_timestamp();
        let repo = Repo {
            id: Default::default(),
            name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description,
            config,
        };
        let repo_id = self
            .db
            .repos
            .create_one(repo)
            .await
            .context("failed to add repo to db")?;
        let repo = self.get_repo(&repo_id).await?;
        let update = Update {
            target: ResourceTarget::Repo(repo_id),
            operation: Operation::CreateRepo,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create repo",
                    format!("created repo\nid: {}\nname: {}", repo.id, repo.name),
                ),
                Log::simple("config", format!("{:#?}", repo.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(repo)
    }
}

#[async_trait]
impl Resolve<DeleteRepo, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteRepo { id }: DeleteRepo,
        user: RequestUser,
    ) -> anyhow::Result<Repo> {
        let repo = self
            .get_repo_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let mut update = Update {
                operation: Operation::DeleteRepo,
                target: ResourceTarget::Repo(repo.id.clone()),
                start_ts: monitor_timestamp(),
                status: UpdateStatus::InProgress,
                operator: user.id.clone(),
                success: true,
                ..Default::default()
            };
            update.id = self.add_update(update.clone()).await?;

            let res = self
                .db
                .repos
                .delete_one(&repo.id)
                .await
                .context("failed to delete repo from database");

            let log = match res {
                Ok(_) => Log::simple("delete repo", format!("deleted repo {}", repo.name)),
                Err(e) => Log::error("delete repo", format!("failed to delete repo\n{e:#?}")),
            };

            update.logs.push(log);

            if !repo.config.server_id.is_empty() {
                let server = self.get_server(&repo.config.server_id).await;
                if let Ok(server) = server {
                    match self
                        .periphery_client(&server)
                        .request(requests::DeleteRepo {
                            name: repo.name.clone(),
                        })
                        .await
                    {
                        Ok(log) => update.logs.push(log),
                        Err(e) => update
                            .logs
                            .push(Log::error("delete repo on periphery", format!("{e:#?}"))),
                    }
                }
            }

            update.finalize();
            self.update_update(update).await?;

            Ok(repo)
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(id.clone(), |entry| {
                entry.deleting = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(id, |entry| {
                entry.deleting = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<UpdateRepo, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateRepo { id, config }: UpdateRepo,
        user: RequestUser,
    ) -> anyhow::Result<Repo> {
        let repo = self
            .get_repo_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            if let Some(server_id) = &config.server_id {
                if !server_id.is_empty() {
                    self.get_server_check_permissions(
                            server_id,
                            &user,
                            PermissionLevel::Update,
                        )
                        .await
                        .context("cannot move repo to this server. user must have update permissions on the server.")?;
                }
            }

            self.db
                .repos
                .update_one(
                    &repo.id,
                    mungos::Update::Set(doc! { "config": to_bson(&config)? }),
                )
                .await
                .context("failed to update repo on database")?;

            let update = Update {
                operation: Operation::UpdateRepo,
                target: ResourceTarget::Repo(repo.id.clone()),
                start_ts,
                end_ts: Some(monitor_timestamp()),
                status: UpdateStatus::Complete,
                logs: vec![Log::simple(
                    "repo update",
                    serde_json::to_string_pretty(&config).unwrap(),
                )],
                operator: user.id.clone(),
                success: true,
                ..Default::default()
            };

            self.add_update(update).await?;

            if let Some(new_server_id) = config.server_id {
                if new_server_id != repo.config.server_id {
                    if !repo.config.server_id.is_empty() {
                        // clean up old server
                        let old_server = self.get_server(&repo.config.server_id).await?;
                        let res = self
                            .periphery_client(&old_server)
                            .request(requests::DeleteRepo { name: repo.name })
                            .await;
                        if let Err(e) = res {
                            warn!(
                                "failed to delete repo ({}) off old server ({}) | {e:#?}",
                                repo.id, old_server.id
                            );
                        }
                    }
                    if !new_server_id.is_empty() {
                        // clone on new server
                        let _ = self
                            .resolve(
                                execute::CloneRepo {
                                    id: repo.id.clone(),
                                },
                                user,
                            )
                            .await;
                    }
                }
            }

            let repo = self.get_repo(&repo.id).await?;

            anyhow::Ok(repo)
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(id.clone(), |entry| {
                entry.updating = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(id, |entry| {
                entry.updating = false;
            })
            .await;

        res
    }
}
