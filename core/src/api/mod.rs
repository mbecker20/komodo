use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Context};
use axum::{middleware, routing::get, Extension, Json, Router};
use db::{DbClient, DbExtension};
use helpers::handle_anyhow_error;
use mungos::ObjectId;
use periphery::PeripheryClient;
use types::{Update, User};

use crate::{
    auth::{auth_request, RequestUserExtension},
    ws::update,
};

mod build;
mod deployment;
mod server;

type PeripheryExtension = Extension<Arc<PeripheryClient>>;

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|user, db| async { get_user(user, db).await.map_err(handle_anyhow_error) }),
        )
        .nest("/build", build::router())
        .nest("/deployment", deployment::router())
        .nest("/server", server::router())
        .layer(Extension(Arc::new(PeripheryClient::new())))
        .layer(middleware::from_fn(auth_request))
}

async fn get_user(
    Extension(user): RequestUserExtension,
    Extension(db): DbExtension,
) -> anyhow::Result<Json<User>> {
    let mut user = db
        .users
        .find_one_by_id(&user.id)
        .await?
        .ok_or(anyhow!("did not find user"))?;
    user.password = None;
    Ok(Json(user))
}

async fn add_update(
    mut update: Update,
    db: &DbClient,
    update_ws: &update::WsSender,
) -> anyhow::Result<()> {
    let update_id = db
        .updates
        .create_one(update.clone())
        .await
        .context("failed to insert update into db. the create build process was completed.")?;
    update.id = Some(ObjectId::from_str(&update_id).context("failed at attaching update id")?);
    let update_msg = serde_json::to_string(&update).unwrap();
    let _ = update_ws.lock().await.send(update);
    Ok(())
}
