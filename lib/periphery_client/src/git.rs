use helpers::git::CloneArgs;
use types::{Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn clone(
        &self,
        server: &Server,
        clone_args: impl Into<CloneArgs>,
    ) -> anyhow::Result<Vec<Log>> {
        let clone_args: CloneArgs = clone_args.into();
        self.post_json(server, "/git/clone", &clone_args).await
    }
}
