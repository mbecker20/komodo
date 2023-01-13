use monitor_types::{Update, UpdateTarget};
use serde_json::{json, Value};

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_updates(
        &self,
        target: impl Into<Option<UpdateTarget>>,
        offset: u64,
    ) -> anyhow::Result<Vec<Update>> {
        let mut query = json!({ "offset": offset });
        if let Some(target) = target.into() {
            let mut value =
                serde_json::from_str::<Value>(&serde_json::to_string(&target).unwrap()).unwrap();
            let value = value.as_object_mut().unwrap();
            query
                .as_object_mut()
                .unwrap()
                .insert("type".to_string(), value.remove("type").unwrap());
            if let Some(target_id) = value.remove("id") {
                query
                    .as_object_mut()
                    .unwrap()
                    .insert("id".to_string(), target_id);
            }
        }
        self.get("/api/update/list", query).await
    }
}
