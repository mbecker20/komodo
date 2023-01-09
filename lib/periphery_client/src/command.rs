use anyhow::Context;
use types::{Command, Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn run_command(&self, server: &Server, command: &Command) -> anyhow::Result<Log> {
        self.post_json(server, &format!("/command"), command)
            .await
            .context("failed to run command on periphery")
    }
}
