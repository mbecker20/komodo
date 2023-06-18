use anyhow::{anyhow, Context};
use monitor_types::{
    entities::{server::Server, user::User, PermissionLevel},
    permissioned::Permissioned,
};

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_user(&self, user_id: &str) -> anyhow::Result<User> {
        self.db
            .users
            .find_one_by_id(user_id)
            .await?
            .context(format!("no user exists with id {user_id}"))
    }

    pub async fn get_server_check_permissions(
        &self,
        server_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Server> {
        let server = self
            .db
            .servers
            .find_one_by_id(server_id)
            .await?
            .context(format!("did not find any server with id {server_id}"))?;
        let permissions = server.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(server)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this server"
            ))
        }
    }
}
