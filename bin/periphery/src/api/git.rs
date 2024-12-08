use anyhow::{anyhow, Context};
use git::GitRes;
use komodo_client::entities::{update::Log, CloneArgs, LatestCommit};
use periphery_client::api::git::{
  CloneRepo, DeleteRepo, GetLatestCommit, PullOrCloneRepo, PullRepo,
  RenameRepo, RepoActionResponse,
};
use resolver_api::Resolve;
use tokio::fs;

use crate::config::periphery_config;

impl Resolve<super::Args> for GetLatestCommit {
  async fn resolve(
    GetLatestCommit { name }: GetLatestCommit,
    _: &super::Args,
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

impl Resolve<super::Args> for CloneRepo {
  #[instrument(
    name = "CloneRepo",
    skip(git_token, environment, replacers)
  )]
  async fn resolve(
    CloneRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: Self,
    _: &super::Args,
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

impl Resolve<super::Args> for PullRepo {
  #[instrument(
    name = "PullRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    PullRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: PullRepo,
    _: &super::Args,
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

impl Resolve<super::Args> for PullOrCloneRepo {
  #[instrument(
    name = "PullOrCloneRepo",
    skip(self, git_token, environment, replacers)
  )]
  async fn resolve(
    PullOrCloneRepo {
      args,
      git_token,
      environment,
      env_file_path,
      skip_secret_interp,
      replacers,
    }: PullOrCloneRepo,
    _: &super::Args,
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

impl Resolve<super::Args> for RenameRepo {
  #[instrument(name = "RenameRepo", skip(self))]
  async fn resolve(
    RenameRepo {
      curr_name,
      new_name,
    }: RenameRepo,
    _: &super::Args,
  ) -> anyhow::Result<Log> {
    let renamed = fs::rename(
      periphery_config().repo_dir.join(&curr_name),
      periphery_config().repo_dir.join(&new_name),
    )
    .await;
    let msg = match renamed {
      Ok(_) => String::from("Renamed Repo directory on Server"),
      Err(_) => format!("No Repo cloned at {curr_name} to rename"),
    };
    Ok(Log::simple("Rename Repo on Server", msg))
  }
}

//

impl Resolve<super::Args> for DeleteRepo {
  #[instrument(name = "DeleteRepo", skip(self))]
  async fn resolve(
    DeleteRepo { name }: DeleteRepo,
    _: &super::Args,
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
