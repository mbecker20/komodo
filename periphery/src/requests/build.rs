use async_trait::async_trait;
use monitor_helpers::optional_string;
use monitor_types::entities::update::Log;
use resolver_api::{derive::Request, Resolve};
use serde::{Deserialize, Serialize};

use crate::{helpers::docker, state::State};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct Build {
    pub build: monitor_types::entities::build::Build,
}

#[async_trait]
impl Resolve<Build> for State {
    async fn resolve(&self, Build { build }: Build, _: ()) -> anyhow::Result<Vec<Log>> {
        let secrets = self.secrets.clone();
        let repo_dir = self.config.repo_dir.clone();
        let log = match self.get_docker_token(&optional_string(&build.config.docker_account)) {
            Ok(docker_token) => {
                match docker::build(&build, repo_dir, docker_token, &secrets).await {
                    Ok(logs) => logs,
                    Err(e) => vec![Log::error("build", format!("{e:#?}"))],
                }
            }
            Err(e) => vec![Log::error("build", format!("{e:#?}"))],
        };
        Ok(log)
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneImages {}

#[async_trait]
impl Resolve<PruneImages> for State {
    async fn resolve(&self, _: PruneImages, _: ()) -> anyhow::Result<Log> {
        Ok(docker::prune_images().await)
    }
}
