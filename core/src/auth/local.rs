use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use async_trait::async_trait;
use monitor_types::{
    entities::user::User,
    requests::auth::{
        CreateLocalUser, CreateLocalUserResponse, LoginLocalUser, LoginLocalUserResponse,
    },
};
use mungos::mongodb::bson::doc;
use resolver_api::Resolve;

use crate::state::State;

const BCRYPT_COST: u32 = 10;

#[async_trait]
impl Resolve<CreateLocalUser> for State {
    async fn resolve(
        &self,
        CreateLocalUser { username, password }: CreateLocalUser,
    ) -> anyhow::Result<CreateLocalUserResponse> {
        if !self.config.local_auth {
            return Err(anyhow!("local auth is not enabled"));
        }

        let password = bcrypt::hash(password, BCRYPT_COST).context("failed to hash password")?;

        let no_users_exist = self.db.users.find_one(None, None).await?.is_none();

        let ts = unix_timestamp_ms() as i64;

        let user = User {
            username,
            password: Some(password),
            enabled: no_users_exist,
            admin: no_users_exist,
            create_server_permissions: no_users_exist,
            create_build_permissions: no_users_exist,
            created_at: ts,
            updated_at: ts,
            ..Default::default()
        };

        let user_id = self
            .db
            .users
            .create_one(user)
            .await
            .context("failed to create user")?;

        let jwt = self
            .jwt
            .generate(user_id)
            .context("failed to generate jwt for user")?;

        Ok(CreateLocalUserResponse { jwt })
    }
}

#[async_trait]
impl Resolve<LoginLocalUser> for State {
    async fn resolve(
        &self,
        LoginLocalUser { username, password }: LoginLocalUser,
    ) -> anyhow::Result<LoginLocalUserResponse> {
        if !self.config.local_auth {
            return Err(anyhow!("local auth is not enabled"));
        }

        let user = self
            .db
            .users
            .find_one(doc! { "username": &username }, None)
            .await
            .context("failed at mongo query")?
            .ok_or(anyhow!("did not find user with username {username}"))?;

        let user_pw_hash = user
            .password
            .ok_or(anyhow!("invalid login, user does not have password login"))?;

        let verified =
            bcrypt::verify(password, &user_pw_hash).context("failed at verify password")?;

        if !verified {
            return Err(anyhow!("invalid credentials"));
        }

        let jwt = self
            .jwt
            .generate(user.id)
            .context("failed at generating jwt for user")?;

        Ok(LoginLocalUserResponse { jwt })
    }
}
