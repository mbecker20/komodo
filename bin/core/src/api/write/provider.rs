use anyhow::{anyhow, Context};
use monitor_client::{
  api::write::*,
  entities::{
    provider::{DockerRegistryAccount, GitProviderAccount},
    update::ResourceTarget,
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, find_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_document},
};
use resolver_api::Resolve;

use crate::{
  helpers::update::{add_update, make_update},
  state::{db_client, State},
};

impl Resolve<CreateGitProviderAccount, User> for State {
  async fn resolve(
    &self,
    CreateGitProviderAccount { account }: CreateGitProviderAccount,
    user: User,
  ) -> anyhow::Result<CreateGitProviderAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "only admins can create git provider accounts"
      ));
    }

    let mut account: GitProviderAccount = account.into();

    if account.domain.is_empty() {
      return Err(anyhow!("domain cannot be empty string."));
    }

    if account.username.is_empty() {
      return Err(anyhow!("username cannot be empty string."));
    }

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::CreateGitProviderAccount,
      &user,
    );

    account.id = db_client()
      .await
      .git_accounts
      .insert_one(&account)
      .await
      .context("failed to create git provider account on db")?
      .inserted_id
      .as_object_id()
      .context("inserted id is not ObjectId")?
      .to_string();

    update.push_simple_log(
      "create git provider account",
      format!(
        "Created git provider account for {} with username {}",
        account.domain, account.username
      ),
    );

    update.finalize();

    add_update(update)
      .await
      .inspect_err(|e| {
        error!("failed to add update for create git provider account | {e:#}")
      })
      .ok();

    Ok(account)
  }
}

impl Resolve<UpdateGitProviderAccount, User> for State {
  async fn resolve(
    &self,
    UpdateGitProviderAccount { id, account }: UpdateGitProviderAccount,
    user: User,
  ) -> anyhow::Result<UpdateGitProviderAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "only admins can update git provider accounts"
      ));
    }

    if let Some(domain) = &account.domain {
      if domain.is_empty() {
        return Err(anyhow!(
          "cannot update git provider with empty domain"
        ));
      }
    }

    if let Some(username) = &account.username {
      if username.is_empty() {
        return Err(anyhow!(
          "cannot update git provider with empty username"
        ));
      }
    }

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::UpdateGitProviderAccount,
      &user,
    );

    let account = to_document(&account).context(
      "failed to serialize partial git provider account to bson",
    )?;
    let db = db_client().await;
    update_one_by_id(
      &db.git_accounts,
      &id,
      doc! { "$set": account },
      None,
    )
    .await
    .context("failed to update git provider account on db")?;

    let Some(account) =
      find_one_by_id(&db.git_accounts, &id)
        .await
        .context("failed to query db for git accounts")?
    else {
      return Err(anyhow!("no account found with given id"));
    };

    update.push_simple_log(
      "update git provider account",
      format!(
        "Updated git provider account for {} with username {}",
        account.domain, account.username
      ),
    );

    update.finalize();

    add_update(update)
      .await
      .inspect_err(|e| {
        error!("failed to add update for update git provider account | {e:#}")
      })
      .ok();

    Ok(account)
  }
}

impl Resolve<DeleteGitProviderAccount, User> for State {
  async fn resolve(
    &self,
    DeleteGitProviderAccount { id }: DeleteGitProviderAccount,
    user: User,
  ) -> anyhow::Result<DeleteGitProviderAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "only admins can delete git provider accounts"
      ));
    }

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::UpdateGitProviderAccount,
      &user,
    );

    let db = db_client().await;
    let Some(account) =
      find_one_by_id(&db.git_accounts, &id)
        .await
        .context("failed to query db for git accounts")?
    else {
      return Err(anyhow!("no account found with given id"));
    };
    delete_one_by_id(&db.git_accounts, &id, None)
      .await
      .context("failed to delete git account on db")?;

    update.push_simple_log(
      "delete git provider account",
      format!(
        "Deleted git provider account for {} with username {}",
        account.domain, account.username
      ),
    );

    update.finalize();

    Ok(account)
  }
}

impl Resolve<CreateDockerRegistryAccount, User> for State {
  async fn resolve(
    &self,
    CreateDockerRegistryAccount { account }: CreateDockerRegistryAccount,
    user: User,
  ) -> anyhow::Result<CreateDockerRegistryAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "only admins can create docker registry account accounts"
      ));
    }

    let mut account: DockerRegistryAccount = account.into();

    if account.domain.is_empty() {
      return Err(anyhow!("domain cannot be empty string."));
    }

    if account.username.is_empty() {
      return Err(anyhow!("username cannot be empty string."));
    }

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::CreateDockerRegistryAccount,
      &user,
    );

    account.id = db_client()
      .await
      .registry_accounts
      .insert_one(&account)
      .await
      .context(
        "failed to create docker registry account account on db",
      )?
      .inserted_id
      .as_object_id()
      .context("inserted id is not ObjectId")?
      .to_string();

    update.push_simple_log(
      "create docker registry account",
      format!(
        "Created docker registry account account for {} with username {}",
        account.domain, account.username
      ),
    );

    update.finalize();

    add_update(update)
      .await
      .inspect_err(|e| {
        error!("failed to add update for create docker registry account | {e:#}")
      })
      .ok();

    Ok(account)
  }
}

impl Resolve<UpdateDockerRegistryAccount, User> for State {
  async fn resolve(
    &self,
    UpdateDockerRegistryAccount { id, account }: UpdateDockerRegistryAccount,
    user: User,
  ) -> anyhow::Result<UpdateDockerRegistryAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "only admins can update docker registry accounts"
      ));
    }

    if let Some(domain) = &account.domain {
      if domain.is_empty() {
        return Err(anyhow!(
          "cannot update docker registry account with empty domain"
        ));
      }
    }

    if let Some(username) = &account.username {
      if username.is_empty() {
        return Err(anyhow!(
          "cannot update docker registry account with empty username"
        ));
      }
    }

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::UpdateDockerRegistryAccount,
      &user,
    );

    let account = to_document(&account).context(
      "failed to serialize partial docker registry account account to bson",
    )?;
		
    let db = db_client().await;
    update_one_by_id(
      &db.registry_accounts,
      &id,
      doc! { "$set": account },
      None,
    )
    .await
    .context(
      "failed to update docker registry account account on db",
    )?;

    let Some(account) = find_one_by_id(&db.registry_accounts, &id)
      .await
      .context("failed to query db for registry accounts")?
    else {
      return Err(anyhow!("no account found with given id"));
    };

    update.push_simple_log(
      "update docker registry account",
      format!(
        "Updated docker registry account account for {} with username {}",
        account.domain, account.username
      ),
    );

    update.finalize();

    add_update(update)
      .await
      .inspect_err(|e| {
        error!("failed to add update for update docker registry account | {e:#}")
      })
      .ok();

    Ok(account)
  }
}

impl Resolve<DeleteDockerRegistryAccount, User> for State {
  async fn resolve(
    &self,
    DeleteDockerRegistryAccount { id }: DeleteDockerRegistryAccount,
    user: User,
  ) -> anyhow::Result<DeleteDockerRegistryAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "only admins can delete docker registry accounts"
      ));
    }

    let mut update = make_update(
      ResourceTarget::system(),
      Operation::UpdateDockerRegistryAccount,
      &user,
    );

    let db = db_client().await;
    let Some(account) = find_one_by_id(&db.registry_accounts, &id)
      .await
      .context("failed to query db for git accounts")?
    else {
      return Err(anyhow!("no account found with given id"));
    };
    delete_one_by_id(&db.registry_accounts, &id, None)
      .await
      .context("failed to delete registry account on db")?;

    update.push_simple_log(
      "delete registry account",
      format!(
        "Deleted registry account for {} with username {}",
        account.domain, account.username
      ),
    );

    update.finalize();

    Ok(account)
  }
}
