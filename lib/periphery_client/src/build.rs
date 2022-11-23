use types::{Build, Log, Server, PERIPHERY_BUILDER_BUSY};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn build(&self, server: &Server, build: &Build) -> anyhow::Result<Option<Vec<Log>>> {
        let res = self.post_json::<_, Vec<Log>>(server, "/build", build).await;
        match res {
            Ok(logs) => Ok(Some(logs)),
            Err(e) => {
                if e.to_string().contains(PERIPHERY_BUILDER_BUSY) {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }
}
