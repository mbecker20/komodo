use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn create_api_secret(
        &self,
        secret_name: &str,
        expires: Option<i64>,
    ) -> anyhow::Result<String> {
        self.post(
            "/api/secret/create",
            json!({
                "name": secret_name,
                "expires": expires
            }),
        )
        .await
    }

    pub async fn delete_api_secret(&self, name: &str) -> anyhow::Result<()> {
        self.delete::<(), _>(&format!("/api/secret/delete/{name}"), None)
            .await
    }
}
