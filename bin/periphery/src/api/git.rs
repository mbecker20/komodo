use anyhow::{anyhow, Context};
use git::GitRes;
use komodo_client::entities::{update::Log, CloneArgs, LatestCommit};
use periphery_client::api::git::{
  CloneRepo, DeleteRepo, GetLatestCommit, PullOrCloneRepo, PullRepo,
  RenameRepo, RepoActionResponse,
};
use resolver_api::Resolve;
use tokio::fs;

use crate::{config::periphery_config, State};

impl Resolve<GetLatestCommit, ()> for State {
  async fn resolve(
    &self,
    GetLatestCommit { name }: GetLatestCommit,
    _: (),
  ) -> anyhow::Result<LatestCommit> {
    let repo_path = periphery_config().repo_dir.join(name);
    if !repo_path.is_dir() {
      return Err(anyhow!(
        "Repo path is not directory. is it cloned?"
      ));
    }
    git::get_commit_hash_info(&repo_path).await
  }
}

impl Resolve<CloneRepo> for State {
  #[instrument(
    name = "CloneRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    &self,
    CloneRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: CloneRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, git_token) {
      (None, _) => None,
      (Some(_), Some(token)) => Some(token),
      (Some(account),  None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("Failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::clone(
      args,
      &periphery_config().repo_dir,
      token,
      &environment,
      &env_file_path,
      (!skip_secret_interp).then_some(&periphery_config().secrets),
      &replacers,
    )
    .await
    .map(
      |GitRes {
         logs,
         hash,
         message,
         env_file_path,
       }| {
        RepoActionResponse {
          logs,
          commit_hash: hash,
          commit_message: message,
          env_file_path,
        }
      },
    )
  }
}

//

impl Resolve<PullRepo> for State {
  #[instrument(
    name = "PullRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    &self,
    PullRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: PullRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, git_token) {
      (None, _) => None,
      (Some(_), Some(token)) => Some(token),
      (Some(account),  None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("Failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::pull(
      args,
      &periphery_config().repo_dir,
      token,
      &environment,
      &env_file_path,
      (!skip_secret_interp).then_some(&periphery_config().secrets),
      &replacers,
    )
    .await
    .map(
      |GitRes {
         logs,
         hash,
         message,
         env_file_path,
       }| {
        RepoActionResponse {
          logs,
          commit_hash: hash,
          commit_message: message,
          env_file_path,
        }
      },
    )
  }
}

//

impl Resolve<PullOrCloneRepo> for State {
  #[instrument(
    name = "PullOrCloneRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    &self,
    PullOrCloneRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: PullOrCloneRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, git_token) {
      (None, _) => None,
      (Some(_), Some(token)) => Some(token),
      (Some(account),  None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("Failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::pull_or_clone(
      args,
      &periphery_config().repo_dir,
      token,
      &environment,
      &env_file_path,
      (!skip_secret_interp).then_some(&periphery_config().secrets),
      &replacers,
    )
    .await
    .map(
      |GitRes {
         logs,
         hash,
         message,
         env_file_path,
       }| {
        RepoActionResponse {
          logs,
          commit_hash: hash,
          commit_message: message,
          env_file_path,
        }
      },
    )
  }
}

//

impl Resolve<RenameRepo> for State {
  #[instrument(name = "RenameRepo", skip(self))]
  async fn resolve(
    &self,
    RenameRepo {
      curr_name,
      new_name,
    }: RenameRepo,
    _: (),
  ) -> anyhow::Result<Log> {
    let renamed = fs::rename(
      periphery_config().repo_dir.join(&curr_name),
      periphery_config().repo_dir.join(&new_name),
    )
    .await;
    let msg = match renamed {
      Ok(_) => format!("Rename Repo from {curr_name} to {new_name}"),
      Err(_) => format!("No Repo cloned at {curr_name} to Rename"),
    };
    Ok(Log::simple("Rename Repo", msg))
  }
}

//

impl Resolve<DeleteRepo> for State {
  #[instrument(name = "DeleteRepo", skip(self))]
  async fn resolve(
    &self,
    DeleteRepo { name }: DeleteRepo,
    _: (),
  ) -> anyhow::Result<Log> {
    // If using custom clone path, it will be passed by core instead of name.
    // So the join will resolve to just the absolute path.
    let deleted =
      fs::remove_dir_all(periphery_config().repo_dir.join(&name))
        .await;
    let msg = match deleted {
      Ok(_) => format!("Deleted Repo {name}"),
      Err(_) => format!("No Repo at {name} to delete"),
    };
    Ok(Log::simple("Delete Repo on Host", msg))
  }
}
