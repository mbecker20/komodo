use anyhow::{anyhow, Context};
use axum::{
    extract::Path,
    routing::{delete, post},
    Extension, Json, Router,
};
use helpers::{generate_secret, handle_anyhow_error};
use mungos::{
    mongodb::bson::{doc, to_bson, Document},
    Update,
};
use serde::{Deserialize, Serialize};
use types::{monitor_timestamp, ApiSecret};
use typeshare::typeshare;

use crate::{auth::RequestUserExtension, state::StateExtension};

const SECRET_LENGTH: usize = 40;
const BCRYPT_COST: u32 = 10;

#[typeshare]
#[derive(Serialize, Deserialize)]
struct CreateSecretBody {
    name: String,
    expires: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DeleteSecretPath {
    name: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/create",
            post(|state, user, secret| async {
                create(state, user, secret)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/delete/:name",
            delete(|state, user, secret_id| async {
                delete_one(state, user, secret_id)
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
            created_at: monitor_timestamp(),
            ..Default::default()
        }
    }
}

async fn create(
    Extension(state): StateExtension,
    Extension(req_user): RequestUserExtension,
    Json(secret): Json<CreateSecretBody>,
) -> anyhow::Result<String> {
    let user = state.db.get_user(&req_user.id).await?;
    for s in &user.secrets {
        if s.name == secret.name {
            return Err(anyhow!("secret with name {} already exists", secret.name));
        }
    }
    let mut secret: ApiSecret = secret.into();
    let secret_str = generate_secret(SECRET_LENGTH);
    secret.hash =
        bcrypt::hash(&secret_str, BCRYPT_COST).context("failed at hashing secret string")?;
    state
        .db
        .users
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
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
    Path(DeleteSecretPath { name }): Path<DeleteSecretPath>,
) -> anyhow::Result<()> {
    state
        .db
        .users
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
