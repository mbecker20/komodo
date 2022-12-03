use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, Update};

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
    Extension(db): DbExtension,
    Extension(jwt): JwtExtension,
    Json(SecretLoginBody { username, secret }): Json<SecretLoginBody>,
) -> anyhow::Result<String> {
    let user = db
        .users
        .find_one(doc! { "username": &username }, None)
        .await
        .context("failed at mongo query")?
        .ok_or(anyhow!("did not find user with username {username}"))?;
    let user_id = user.id.unwrap().to_string();
    let ts = unix_timestamp_ms() as i64;
    for s in user.secrets {
        if let Some(expires) = s.expires {
            if expires < ts {
                db.users
                    .update_one::<Document>(
                        &user_id,
                        Update::Custom(doc! { "$pull": { "secrets": { "name": s.name } } }),
                    )
                    .await
                    .context("failed to remove expired secret")?;
                continue;
            }
        }
        if bcrypt::verify(&secret, &s.hash).context("failed at verifying hash")? {
            let jwt = jwt
                .generate(user_id)
                .context("failed at generating jwt for user")?;
            return Ok(jwt);
        }
    }
    Err(anyhow!("invalid secret"))
}
