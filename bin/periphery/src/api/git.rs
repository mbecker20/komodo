use monitor_client::entities::{to_monitor_name, update::Log};
use periphery_client::api::git::{CloneRepo, DeleteRepo, PullRepo};
use resolver_api::Resolve;

use crate::{config::periphery_config, helpers::git, State};

#[async_trait::async_trait]
impl Resolve<CloneRepo> for State {
  async fn resolve(
    &self,
    CloneRepo { args }: CloneRepo,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    git::clone(args).await
  }
}

//

#[async_trait::async_trait]
impl Resolve<PullRepo> for State {
  async fn resolve(
    &self,
    PullRepo {
      name,
      branch,
      on_pull,
    }: PullRepo,
    _: (),
  ) -> anyhow::Result<Vec<Log>> {
    let name = to_monitor_name(&name);
    Ok(
      git::pull(
        periphery_config().repo_dir.join(name),
        &branch,
        &on_pull,
      )
      .await,
    )
  }
}

//

#[async_trait::async_trait]
impl Resolve<DeleteRepo> for State {
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
