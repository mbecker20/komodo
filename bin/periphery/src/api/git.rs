use anyhow::anyhow;
use monitor_client::entities::{
  to_monitor_name, update::Log, CloneArgs, LatestCommit,
};
use periphery_client::api::git::{
  CloneRepo, DeleteRepo, GetLatestCommit, PullRepo,
};
use resolver_api::Resolve;

use crate::{
  config::periphery_config, helpers::get_github_token, State,
};

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
    CloneRepo { args, github_token }: CloneRepo,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let CloneArgs { github_account, .. } = &args;
    let github_token =
      match (github_token, get_github_token(github_account)) {
        (Some(token), _) => Some(token),
        (None, Ok(token)) => token,
        (None, Err(e)) => return Err(e),
      };
    git::clone(args, &periphery_config().repo_dir, github_token).await
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
  ) -> anyhow::Result<Vec<Log>> {
    let name = to_monitor_name(&name);
    Ok(
      git::pull(
        &periphery_config().repo_dir.join(name),
        &branch,
        &commit,
        &on_pull,
      )
      .await,
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
