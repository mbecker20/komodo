use anyhow::{anyhow, Context};
use axum::{extract::Json, routing::post, Extension, Router};
use db::{DbClient, DbExtension};
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize};
use types::{User, UserCredentials};

use super::jwt::JwtExtension;

const BCRYPT_COST: u32 = 10;

pub fn router() -> Router {
    Router::new()
        .route(
            "/create_user",
            post(|db, jwt, body| async {
                create_user_handler(db, jwt, body)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/login",
            post(|db, jwt, body| async {
                login_handler(db, jwt, body)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

async fn create_user_handler(
    Extension(db): DbExtension,
    Extension(jwt): JwtExtension,
    Json(UserCredentials { username, password }): Json<UserCredentials>,
) -> anyhow::Result<String> {
    let password = bcrypt::hash(password, BCRYPT_COST).context("failed to hash password")?;

    let user = User {
        username,
        password: Some(password),
        ..Default::default()
    };

    let user_id = db
        .users
        .create_one(user)
        .await
        .context("failed to create user")?;

    let jwt = jwt
        .generate(user_id)
        .context("failed to generate jwt for user")?;

    Ok(jwt)
}

async fn login_handler(
    Extension(db): DbExtension,
    Extension(jwt): JwtExtension,
    Json(UserCredentials { username, password }): Json<UserCredentials>,
) -> anyhow::Result<String> {
    let user = db
        .users
        .find_one(doc! { "username": &username }, None)
        .await
        .context("failed at mongo query")?
        .ok_or(anyhow!("did not find user with username {username}"))?;

    let user_pw_hash = user
        .password
        .ok_or(anyhow!("invalid login, user does not have password login"))?;

    let verified = bcrypt::verify(password, &user_pw_hash).context("failed at verify password")?;

    if !verified {
        return Err(anyhow!("invalid credentials"));
    }

    let user_id = user.id.ok_or(anyhow!("user does not have id"))?.to_string();

    let jwt = jwt
        .generate(user_id)
        .context("failed at generating jwt for user")?;

    Ok(jwt)
}
