use std::sync::Arc;

use axum::Extension;
use types::BuilderSecrets;

mod api;
mod config;

type BuilderSecretsExtension = Extension<Arc<BuilderSecrets>>;

#[tokio::main]
async fn main() {
    let (socket_addr, secrets) = config::load();

    let app = api::router().layer(secrets);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("server crashed");
}
