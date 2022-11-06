use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::{extract::Json, Extension};
use db::DbClient;
use mungos::{doc, Deserialize};
use types::{User, UserCredentials};

const BCRYPT_COST: u32 = 10;

async fn create_user_handler(
    Extension(db): Extension<Arc<DbClient>>,
    Json(UserCredentials { username, password }): Json<UserCredentials>,
) -> anyhow::Result<String> {
    let password = bcrypt::hash(password, BCRYPT_COST).context("failed to hash password")?;

    let user = User {
        username,
        password: Some(password),
        ..Default::default()
    };

    let id = db
        .users
        .create_one(user)
        .await
        .context("failed to create user")?;

    Ok(id)
}

async fn login_handler(
    Extension(db): Extension<Arc<DbClient>>,
    Json(UserCredentials { username, password }): Json<UserCredentials>,
) -> anyhow::Result<String> {
    // returns a jwt if login successful
    let user = db
        .users
        .find_one(doc! { "username": &username }, None)
        .await
        .context("failed at mongo query")?
        .ok_or(anyhow!("did not find user with username {username}"))?;

    let user_pw_hash = user.password.ok_or(anyhow!("invalid login, user does not have password login"))?;

    let verified = bcrypt::verify(password, &user_pw_hash).context("failed at verify password")?;

    if !verified {
        return Err(anyhow!("invalid credentials"))
    }

    // get a jwt for the user and send it back

    todo!()
}
