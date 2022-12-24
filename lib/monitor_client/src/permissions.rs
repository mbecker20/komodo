use monitor_types::{PermissionLevel, PermissionsTarget, Update};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn update_user_permissions_on_target(
        &self,
        user_id: &str,
        permission: PermissionLevel,
        target_type: PermissionsTarget,
        target_id: &str,
    ) -> anyhow::Result<Update> {
        self.post(
            "/api/permissions/update",
            json!({
                "user_id": user_id,
                "permission": permission,
                "target_type": target_type,
                "target_id": target_id
            }),
        )
        .await
    }

    pub async fn modify_user_enabled(
        &self,
        user_id: &str,
        enabled: bool,
    ) -> anyhow::Result<Update> {
        self.post(
            "/api/permissions/update",
            json!({
                "user_id": user_id,
                "enabled": enabled,
            }),
        )
        .await
    }
}
