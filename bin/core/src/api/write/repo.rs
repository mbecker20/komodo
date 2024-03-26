use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::{execute, write::*},
  entities::{
    monitor_timestamp,
    permission::PermissionLevel,
    repo::Repo,
    server::Server,
    to_monitor_name,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_bson},
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    add_update, create_permission, make_update, periphery_client,
    remove_from_recently_viewed, resource::StateResource,
    update_update,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<CreateRepo, User> for State {
  async fn resolve(
    &self,
    CreateRepo { name, config }: CreateRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    let name = to_monitor_name(&name);
    if let Some(server_id) = &config.server_id {
      if !server_id.is_empty() {
        let _: Server = self.get_resource_check_permissions(
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
      updated_at: start_ts,
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: Default::default(),
    };
    let repo_id = db_client()
      .await
      .repos
      .insert_one(repo, None)
      .await
      .context("failed to add repo to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();

    let repo: Repo = self.get_resource(&repo_id).await?;

    create_permission(&user, &repo, PermissionLevel::Update).await;

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
          format!(
            "created repo\nid: {}\nname: {}",
            repo.id, repo.name
          ),
        ),
        Log::simple("config", format!("{:#?}", repo.config)),
      ],
      ..Default::default()
    };

    add_update(update).await?;

    if !repo.config.repo.is_empty()
      && !repo.config.server_id.is_empty()
    {
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
impl Resolve<CopyRepo, User> for State {
  async fn resolve(
    &self,
    CopyRepo { name, id }: CopyRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    let Repo {
      config,
      description,
      tags,
      ..
    } = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;
    if !config.server_id.is_empty() {
      let _: Server = self.get_resource_check_permissions(
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
      updated_at: start_ts,
      description,
      tags,
      config,
      info: Default::default(),
    };
    let repo_id = db_client()
      .await
      .repos
      .insert_one(repo, None)
      .await
      .context("failed to add repo to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let repo: Repo = self.get_resource(&repo_id).await?;
    create_permission(&user, &repo, PermissionLevel::Update).await;
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
          format!(
            "created repo\nid: {}\nname: {}",
            repo.id, repo.name
          ),
        ),
        Log::simple("config", format!("{:#?}", repo.config)),
      ],
      ..Default::default()
    };

    add_update(update).await?;

    Ok(repo)
  }
}

#[async_trait]
impl Resolve<DeleteRepo, User> for State {
  async fn resolve(
    &self,
    DeleteRepo { id }: DeleteRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    let repo: Repo = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let periphery = if repo.config.server_id.is_empty() {
      None
    } else {
      let server: Server =
        self.get_resource(&repo.config.server_id).await?;
      let periphery = periphery_client(&server)?;
      Some(periphery)
    };

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
      update.id = add_update(update.clone()).await?;

      let res =
        delete_one_by_id(&db_client().await.repos, &repo.id, None)
          .await
          .context("failed to delete repo from database");

      let log = match res {
        Ok(_) => Log::simple(
          "delete repo",
          format!("deleted repo {}", repo.name),
        ),
        Err(e) => Log::error(
          "delete repo",
          format!("failed to delete repo\n{e:#?}"),
        ),
      };

      update.logs.push(log);

      if let Some(periphery) = periphery {
        match periphery
          .request(api::git::DeleteRepo {
            name: repo.name.clone(),
          })
          .await
        {
          Ok(log) => update.logs.push(log),
          Err(e) => update.logs.push(Log::error(
            "delete repo on periphery",
            format!("{e:#?}"),
          )),
        }
      }

      update.finalize();
      update_update(update).await?;

      remove_from_recently_viewed(&repo).await?;

      Ok(repo)
    };

    if action_states().repo.busy(&id).await {
      return Err(anyhow!("repo busy"));
    }

    action_states()
      .repo
      .update_entry(id.clone(), |entry| {
        entry.deleting = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .repo
      .update_entry(id, |entry| {
        entry.deleting = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<UpdateRepo, User> for State {
  async fn resolve(
    &self,
    UpdateRepo { id, config }: UpdateRepo,
    user: User,
  ) -> anyhow::Result<Repo> {
    if let Some(server_id) = &config.server_id {
      if !server_id.is_empty() {
        let _: Server = self.get_resource_check_permissions(
          server_id,
          &user,
          PermissionLevel::Update,
        )
        .await
        .context("cannot move repo to this server. user must have update permissions on the server.")?;
      }
    }

    let repo: Repo = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let inner = || async move {
      update_one_by_id(
        &db_client().await.repos,
        &repo.id,
        mungos::update::Update::FlattenSet(
          doc! { "config": to_bson(&config)? },
        ),
        None,
      )
      .await
      .context("failed to update repo on database")?;

      let mut update =
        make_update(&repo, Operation::UpdateRepo, &user);
      update.in_progress();
      update.push_simple_log(
        "repo update",
        serde_json::to_string_pretty(&config).unwrap(),
      );
      update.id = add_update(update.clone()).await?;

      if let Some(new_server_id) = config.server_id {
        if new_server_id != repo.config.server_id {
          if !repo.config.server_id.is_empty() {
            let old_server: anyhow::Result<Server> =
              self.get_resource(&repo.config.server_id).await;
            let periphery =
              old_server.and_then(|server| periphery_client(&server));
            match periphery {
              Ok(periphery) => match periphery
                .request(api::git::DeleteRepo { name: repo.name })
                .await
              {
                Ok(mut log) => {
                  log.stage = String::from("cleanup previous server");
                  update.logs.push(log);
                }
                Err(e) => update.push_error_log(
                  "cleanup previous server",
                  format!(
                    "failed to cleanup previous server | {e:#?}"
                  ),
                ),
              },
              Err(e) => update.push_error_log(
                "cleanup previous server",
                format!("failed to cleanup previous server | {e:#?}"),
              ),
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

      update.finalize();
      update_update(update).await?;

      self.get_resource(&repo.id).await
    };

    if action_states().repo.busy(&id).await {
      return Err(anyhow!("repo busy"));
    }

    action_states()
      .repo
      .update_entry(id.clone(), |entry| {
        entry.updating = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .repo
      .update_entry(id, |entry| {
        entry.updating = false;
      })
      .await;

    res
  }
}
