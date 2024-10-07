use anyhow::{anyhow, Context};
use komodo_client::{
  api::read::{
    GetDockerRegistryAccount, GetDockerRegistryAccountResponse,
    GetGitProviderAccount, GetGitProviderAccountResponse,
    ListDockerRegistryAccounts, ListDockerRegistryAccountsResponse,
    ListGitProviderAccounts, ListGitProviderAccountsResponse,
  },
  entities::user::User,
};
use mongo_indexed::{doc, Document};
use mungos::{
  by_id::find_one_by_id, find::find_collect,
  mongodb::options::FindOptions,
};
use resolver_api::Resolve;

use crate::state::{db_client, State};

impl Resolve<GetGitProviderAccount, User> for State {
  async fn resolve(
    &self,
    GetGitProviderAccount { id }: GetGitProviderAccount,
    user: User,
  ) -> anyhow::Result<GetGitProviderAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "Only admins can read git provider accounts"
      ));
    }
    find_one_by_id(&db_client().git_accounts, &id)
      .await
      .context("failed to query db for git provider accounts")?
      .context("did not find git provider account with the given id")
  }
}

impl Resolve<ListGitProviderAccounts, User> for State {
  async fn resolve(
    &self,
    ListGitProviderAccounts { domain, username }: ListGitProviderAccounts,
    user: User,
  ) -> anyhow::Result<ListGitProviderAccountsResponse> {
    if !user.admin {
      return Err(anyhow!(
        "Only admins can read git provider accounts"
      ));
    }
    let mut filter = Document::new();
    if let Some(domain) = domain {
      filter.insert("domain", domain);
    }
    if let Some(username) = username {
      filter.insert("username", username);
    }
    find_collect(
      &db_client().git_accounts,
      filter,
      FindOptions::builder()
        .sort(doc! { "domain": 1, "username": 1 })
        .build(),
    )
    .await
    .context("failed to query db for git provider accounts")
  }
}

impl Resolve<GetDockerRegistryAccount, User> for State {
  async fn resolve(
    &self,
    GetDockerRegistryAccount { id }: GetDockerRegistryAccount,
    user: User,
  ) -> anyhow::Result<GetDockerRegistryAccountResponse> {
    if !user.admin {
      return Err(anyhow!(
        "Only admins can read docker registry accounts"
      ));
    }
    find_one_by_id(&db_client().registry_accounts, &id)
      .await
      .context("failed to query db for docker registry accounts")?
      .context(
        "did not find docker registry account with the given id",
      )
  }
}

impl Resolve<ListDockerRegistryAccounts, User> for State {
  async fn resolve(
    &self,
    ListDockerRegistryAccounts { domain, username }: ListDockerRegistryAccounts,
    user: User,
  ) -> anyhow::Result<ListDockerRegistryAccountsResponse> {
    if !user.admin {
      return Err(anyhow!(
        "Only admins can read docker registry accounts"
      ));
    }
    let mut filter = Document::new();
    if let Some(domain) = domain {
      filter.insert("domain", domain);
    }
    if let Some(username) = username {
      filter.insert("username", username);
    }
    find_collect(
      &db_client().registry_accounts,
      filter,
      FindOptions::builder()
        .sort(doc! { "domain": 1, "username": 1 })
        .build(),
    )
    .await
    .context("failed to query db for docker registry accounts")
  }
}
