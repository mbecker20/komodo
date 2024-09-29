use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::http::HeaderMap;
use komodo_client::{
  api::auth::{
    CreateLocalUser, CreateLocalUserResponse, LoginLocalUser,
    LoginLocalUserResponse,
  },
  entities::user::{User, UserConfig},
};
use mongo_indexed::Document;
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::{
  config::core_config,
  state::State,
  state::{db_client, jwt_client},
};

const BCRYPT_COST: u32 = 10;

impl Resolve<CreateLocalUser, HeaderMap> for State {
  #[instrument(name = "CreateLocalUser", skip(self))]
  async fn resolve(
    &self,
    CreateLocalUser { username, password }: CreateLocalUser,
    _: HeaderMap,
  ) -> anyhow::Result<CreateLocalUserResponse> {
    let core_config = core_config();

    if !core_config.local_auth {
      return Err(anyhow!("Local auth is not enabled"));
    }

    if username.is_empty() {
      return Err(anyhow!("Username cannot be empty string"));
    }

    if ObjectId::from_str(&username).is_ok() {
      return Err(anyhow!("Username cannot be valid ObjectId"));
    }

    if password.is_empty() {
      return Err(anyhow!("Password cannot be empty string"));
    }

    let password = bcrypt::hash(password, BCRYPT_COST)
      .context("failed to hash password")?;

    let no_users_exist = db_client()
      .users
      .find_one(Document::new())
      .await?
      .is_none();

    if !no_users_exist && core_config.disable_user_registration {
      return Err(anyhow!("User registration is disabled"));
    }

    let ts = unix_timestamp_ms() as i64;

    let user = User {
      id: Default::default(),
      username,
      enabled: no_users_exist || core_config.enable_new_users,
      admin: no_users_exist,
      super_admin: no_users_exist,
      create_server_permissions: no_users_exist,
      create_build_permissions: no_users_exist,
      updated_at: ts,
      last_update_view: 0,
      recents: Default::default(),
      all: Default::default(),
      config: UserConfig::Local { password },
    };

    let user_id = db_client()
      .users
      .insert_one(user)
      .await
      .context("failed to create user")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();

    let jwt = jwt_client()
      .generate(user_id)
      .context("failed to generate jwt for user")?;

    Ok(CreateLocalUserResponse { jwt })
  }
}

impl Resolve<LoginLocalUser, HeaderMap> for State {
  #[instrument(name = "LoginLocalUser", level = "debug", skip(self))]
  async fn resolve(
    &self,
    LoginLocalUser { username, password }: LoginLocalUser,
    _: HeaderMap,
  ) -> anyhow::Result<LoginLocalUserResponse> {
    if !core_config().local_auth {
      return Err(anyhow!("local auth is not enabled"));
    }

    let user = db_client()
      .users
      .find_one(doc! { "username": &username })
      .await
      .context("failed at db query for users")?
      .with_context(|| {
        format!("did not find user with username {username}")
      })?;

    let UserConfig::Local {
      password: user_pw_hash,
    } = user.config
    else {
      return Err(anyhow!(
        "non-local auth users can not log in with a password"
      ));
    };

    let verified = bcrypt::verify(password, &user_pw_hash)
      .context("failed at verify password")?;

    if !verified {
      return Err(anyhow!("invalid credentials"));
    }

    let jwt = jwt_client()
      .generate(user.id)
      .context("failed at generating jwt for user")?;

    Ok(LoginLocalUserResponse { jwt })
  }
}
