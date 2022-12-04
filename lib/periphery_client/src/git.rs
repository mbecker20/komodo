use helpers::git::CloneArgs;
use serde_json::json;
use types::{Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn clone_repo(
        &self,
        server: &Server,
        clone_args: impl Into<CloneArgs>,
    ) -> anyhow::Result<Vec<Log>> {
        let clone_args: CloneArgs = clone_args.into();
        self.post_json(server, "/git/clone", &clone_args).await
    }

    pub async fn delete_repo(&self, server: &Server, build_name: &str) -> anyhow::Result<Log> {
        self.post_json(server, "/git/delete", &json!({ "name": build_name }))
            .await
    }
}
