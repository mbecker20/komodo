use std::sync::Arc;

use axum::Extension;
use helpers::get_socket_addr;
use types::BuilderSecrets;

mod api;
mod config;

type BuilderSecretsExtension = Extension<Arc<BuilderSecrets>>;

#[tokio::main]
async fn main() {
    let (port, secrets) = config::load();

    let app = api::router().layer(secrets);

    println!("starting montior builder on port {port}");

    axum::Server::bind(&get_socket_addr(port))
        .serve(app.into_make_service())
        .await
        .expect("server crashed");
}
