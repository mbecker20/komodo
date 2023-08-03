use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::user::User,
    requests::read::{GetUser, GetUsername, GetUsernameResponse},
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetUser, RequestUser> for State {
    async fn resolve(&self, GetUser {}: GetUser, user: RequestUser) -> anyhow::Result<User> {
        let mut user = self
            .db
            .users
            .find_one_by_id(&user.id)
            .await
            .context("failed at mongo query")?
            .context("no user found with id")?;
        for secret in &mut user.secrets {
            secret.hash = String::new();
        }
        Ok(user)
    }
}

#[async_trait]
impl Resolve<GetUsername, RequestUser> for State {
    async fn resolve(
        &self,
        GetUsername { user_id }: GetUsername,
        _: RequestUser,
    ) -> anyhow::Result<GetUsernameResponse> {
        let user = self
            .db
            .users
            .find_one_by_id(&user_id)
            .await
            .context("failed at mongo query")?
            .context("no user found with id")?;

        Ok(GetUsernameResponse {
            username: user.username,
        })
    }
}
