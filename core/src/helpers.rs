use anyhow::Context;
use monitor_types::entities::user::User;

use crate::state::State;

impl State {
    pub async fn get_user(&self, user_id: &str) -> anyhow::Result<User> {
        self.db
            .users
            .find_one_by_id(user_id)
            .await?
            .context(format!("no user exists with id {user_id}"))
    }
}
