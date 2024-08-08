use anyhow::{anyhow, Context};
use monitor_client::entities::{
  to_monitor_name, update::Log, CloneArgs, LatestCommit,
};
use periphery_client::api::git::{
  CloneRepo, DeleteRepo, GetLatestCommit, PullRepo,
  RepoActionResponse, RepoActionResponseV1_13,
};
use resolver_api::Resolve;

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
        "repo path is not directory. is it cloned?"
      ));
    }
    git::get_commit_hash_info(&repo_path).await
  }
}

impl Resolve<CloneRepo> for State {
  #[instrument(name = "CloneRepo", skip(self))]
  async fn resolve(
    &self,
    CloneRepo { args, git_token }: CloneRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let CloneArgs {
      provider, account, ..
    } = &args;
    let token = match (account, provider, git_token) {
      (None, _, _) => None,
      (Some(_), None, _) => {
        return Err(anyhow!(
          "got incoming git account but no git provider"
        ))
      }
      (Some(_), Some(_), Some(token)) => Some(token),
      (Some(account), Some(provider), None) => Some(
        crate::helpers::git_token(provider, account).map(ToString::to_string)
          .with_context(
            || format!("failed to get git token from periphery config | provider: {provider} | account: {account}")
          )?,
      ),
    };
    git::clone(args, &periphery_config().repo_dir, token)
      .await
      .map(|(logs, commit_hash, commit_message)| {
        RepoActionResponseV1_13 {
          logs,
          commit_hash,
          commit_message,
        }
        .into()
      })
  }
}

//

impl Resolve<PullRepo> for State {
  #[instrument(name = "PullRepo", skip(self))]
  async fn resolve(
    &self,
    PullRepo {
      name,
      branch,
      commit,
      on_pull,
    }: PullRepo,
    _: (),
  ) -> anyhow::Result<RepoActionResponse> {
    let name = to_monitor_name(&name);
    let (logs, commit_hash, commit_message) = git::pull(
      &periphery_config().repo_dir.join(name),
      &branch,
      &commit,
      &on_pull,
    )
    .await;
    Ok(
      RepoActionResponseV1_13 {
        logs,
        commit_hash,
        commit_message,
      }
      .into(),
    )
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
    let name = to_monitor_name(&name);
    let deleted = std::fs::remove_dir_all(
      periphery_config().repo_dir.join(&name),
    );
    let msg = match deleted {
      Ok(_) => format!("deleted repo {name}"),
      Err(_) => format!("no repo at {name} to delete"),
    };
    Ok(Log::simple("delete repo", msg))
  }
}
