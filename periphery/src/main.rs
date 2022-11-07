use axum::Router;
use docker::DockerClient;
use helpers::get_socket_addr;

mod config;
mod helpers;

#[tokio::main]
async fn main() {
    let (port, secrets) = config::load();

    let app = Router::new()
		.layer(DockerClient::extension());

    axum::Server::bind(&get_socket_addr(port))
        .serve(app.into_make_service())
        .await
        .expect("monitor periphery axum server crashed");
}
