use anyhow::anyhow;
use types::{traits::Permissioned, Group, PermissionLevel};

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_group_check_permissions(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Group> {
        let group = self.db.get_group(deployment_id).await?;
        let permissions = group.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(group)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this deployment"
            ))
        }
    }
}
