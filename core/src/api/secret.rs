use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::Path,
    routing::{delete, post},
    Extension, Json, Router,
};
use db::DbExtension;
use helpers::{generate_secret, handle_anyhow_error};
use mungos::{doc, to_bson, Deserialize, Document, Update};
use types::ApiSecret;

use crate::auth::RequestUserExtension;

const SECRET_LENGTH: usize = 40;
const BCRYPT_COST: u32 = 10;

#[derive(Deserialize)]
struct CreateSecretBody {
    name: String,
    expires: Option<i64>,
}

#[derive(Deserialize)]
struct DeleteSecretPath {
    name: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/create",
            post(|db, user, secret| async {
                create(db, user, secret).await.map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/delete/:name",
            delete(|db, user, secret_id| async {
                delete_one(db, user, secret_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

impl Into<ApiSecret> for CreateSecretBody {
    fn into(self) -> ApiSecret {
        ApiSecret {
            name: self.name,
            expires: self.expires,
            created_at: unix_timestamp_ms() as i64,
            ..Default::default()
        }
    }
}

async fn create(
    Extension(db): DbExtension,
    Extension(req_user): RequestUserExtension,
    Json(secret): Json<CreateSecretBody>,
) -> anyhow::Result<String> {
    let user = db.get_user(&req_user.id).await?;
    for s in &user.secrets {
        if s.name == secret.name {
            return Err(anyhow!("secret with name {} already exists", secret.name));
        }
    }
    let mut secret: ApiSecret = secret.into();
    let secret_str = generate_secret(SECRET_LENGTH);
    secret.hash =
        bcrypt::hash(&secret_str, BCRYPT_COST).context("failed at hashing secret string")?;
    db.users
        .update_one::<Document>(
            &req_user.id,
            Update::Custom(doc! {
                "$push": {
                    "secrets": to_bson(&secret).context("failed at converting secret to bson")?
                }
            }),
        )
        .await
        .context("failed at mongo update query")?;
    Ok(secret_str)
}

async fn delete_one(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Path(DeleteSecretPath { name }): Path<DeleteSecretPath>,
) -> anyhow::Result<()> {
    db.users
        .update_one::<Document>(
            &user.id,
            Update::Custom(doc! {
                "$pull": {
                    "secrets": {
                        "name": name
                    }
                }
            }),
        )
        .await
        .context("failed at mongo update query")?;
    Ok(())
}
