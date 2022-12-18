use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{routing::post, Extension, Json, Router};
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, Update};
use types::unix_from_monitor_ts;

use crate::state::StateExtension;

use super::JwtExtension;

#[derive(Deserialize)]
pub struct SecretLoginBody {
    username: String,
    secret: String,
}

pub fn router() -> Router {
    Router::new().route(
        "/login",
        post(|db, jwt, body| async { login(db, jwt, body).await.map_err(handle_anyhow_error) }),
    )
}

pub async fn login(
    Extension(state): StateExtension,
    Extension(jwt): JwtExtension,
    Json(SecretLoginBody { username, secret }): Json<SecretLoginBody>,
) -> anyhow::Result<String> {
    let user = state
        .db
        .users
        .find_one(doc! { "username": &username }, None)
        .await
        .context("failed at mongo query")?
        .ok_or(anyhow!("did not find user with username {username}"))?;
    let ts = unix_timestamp_ms() as i64;
    for s in user.secrets {
        if let Some(expires) = s.expires {
            let expires = unix_from_monitor_ts(&expires)?;
            if expires < ts {
                state
                    .db
                    .users
                    .update_one::<Document>(
                        &user.id,
                        Update::Custom(doc! { "$pull": { "secrets": { "name": s.name } } }),
                    )
                    .await
                    .context("failed to remove expired secret")?;
                continue;
            }
        }
        if bcrypt::verify(&secret, &s.hash).context("failed at verifying hash")? {
            let jwt = jwt
                .generate(user.id)
                .context("failed at generating jwt for user")?;
            return Ok(jwt);
        }
    }
    Err(anyhow!("invalid secret"))
}
