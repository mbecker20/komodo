use anyhow::Context;
use serde_json::json;
use types::{CloneArgs, Command, Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn clone_repo(
        &self,
        server: &Server,
        clone_args: impl Into<CloneArgs>,
    ) -> anyhow::Result<Vec<Log>> {
        let clone_args: CloneArgs = clone_args.into();
        self.post_json(server, "/git/clone", &clone_args)
            .await
            .context("failed to clone repo on periphery")
    }

    pub async fn pull_repo(
        &self,
        server: &Server,
        name: &str,
        branch: &Option<String>,
        on_pull: &Option<Command>,
    ) -> anyhow::Result<Vec<Log>> {
        self.post_json(
            server,
            "/git/pull",
            &json!({ "name": name, "branch": branch, "on_pull": on_pull }),
        )
        .await
        .context("failed to pull repo on periphery")
    }

    pub async fn delete_repo(&self, server: &Server, build_name: &str) -> anyhow::Result<Log> {
        self.post_json(server, "/git/delete", &json!({ "name": build_name }))
            .await
            .context("failed to delete repo on periphery")
    }
}
