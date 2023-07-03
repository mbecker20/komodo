use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        repo::Repo,
        update::{Log, ResourceTarget, Update},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    permissioned::Permissioned,
    requests::api::{
        CloneRepo, CopyRepo, CreateRepo, DeleteRepo, GetRepo, ListRepos, PullRepo, UpdateRepo,
    },
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetRepo, RequestUser> for State {
    async fn resolve(&self, GetRepo { id }: GetRepo, user: RequestUser) -> anyhow::Result<Repo> {
        self.get_repo_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListRepos, RequestUser> for State {
    async fn resolve(
        &self,
        ListRepos { query }: ListRepos,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Repo>> {
        let repos = self
            .db
            .repos
            .get_some(query, None)
            .await
            .context("failed to pull repos from mongo")?;

        let repos = if user.is_admin {
            repos
        } else {
            repos
                .into_iter()
                .filter(|repo| repo.get_user_permissions(&user.id) > PermissionLevel::None)
                .collect()
        };

        Ok(repos)
    }
}

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
                    CloneRepo {
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
            let start_ts = monitor_timestamp();

            todo!()
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(repo.id.clone(), |entry| {
                entry.deleting = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(repo.id, |entry| {
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

            todo!()
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(repo.id.clone(), |entry| {
                entry.updating = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(repo.id, |entry| {
                entry.updating = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<CloneRepo, RequestUser> for State {
    async fn resolve(
        &self,
        CloneRepo { id }: CloneRepo,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let repo = self
            .get_repo_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            todo!()
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(repo.id.clone(), |entry| {
                entry.cloning = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(repo.id, |entry| {
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
        let repo = self
            .get_repo_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            todo!()
        };

        if self.action_states.repo.busy(&id).await {
            return Err(anyhow!("repo busy"));
        }

        self.action_states
            .repo
            .update_entry(repo.id.clone(), |entry| {
                entry.pulling = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .repo
            .update_entry(repo.id, |entry| {
                entry.pulling = false;
            })
            .await;

        res
    }
}
