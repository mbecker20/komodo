use anyhow::Context;
use types::{ImageSummary, Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn image_list(&self, server: &Server) -> anyhow::Result<Vec<ImageSummary>> {
        self.get_json(server, "/image/list")
            .await
            .context("failed to get image list from periphery")
    }

    pub async fn image_prune(&self, server: &Server) -> anyhow::Result<Log> {
        self.post_json(server, &format!("/image/prune"), &(), ())
            .await
            .context("failed to prune images on periphery")
    }
}
