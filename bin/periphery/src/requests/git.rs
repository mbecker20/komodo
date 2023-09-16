use monitor_types::{
    entities::{update::Log, CloneArgs, SystemCommand},
    to_monitor_name,
};
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::{helpers::git, state::State};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct CloneRepo {
    pub args: CloneArgs,
}

#[async_trait::async_trait]
impl Resolve<CloneRepo> for State {
    async fn resolve(
        &self,
        CloneRepo { args }: CloneRepo,
        _: (),
    ) -> anyhow::Result<Vec<Log>> {
        let access_token =
            self.get_github_token(&args.github_account)?;
        git::clone(args, self.config.repo_dir.clone(), access_token)
            .await
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct PullRepo {
    pub name: String,
    pub branch: Option<String>,
    pub on_pull: Option<SystemCommand>,
}

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
        Ok(git::pull(
            self.config.repo_dir.join(name),
            &branch,
            &on_pull,
        )
        .await)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteRepo {
    pub name: String,
}

#[async_trait::async_trait]
impl Resolve<DeleteRepo> for State {
    async fn resolve(
        &self,
        DeleteRepo { name }: DeleteRepo,
        _: (),
    ) -> anyhow::Result<Log> {
        let name = to_monitor_name(&name);
        let deleted =
            std::fs::remove_dir_all(self.config.repo_dir.join(&name));
        let msg = match deleted {
            Ok(_) => format!("deleted repo {name}"),
            Err(_) => format!("no repo at {name} to delete"),
        };
        Ok(Log::simple("delete repo", msg))
    }
}
