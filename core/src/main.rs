use axum::Router;
use docker::DockerClient;

mod api;
mod config;

#[tokio::main]
async fn main() {
    let docker_client = DockerClient::new().unwrap();
    let containers = docker_client.list_containers().await.unwrap();
    println!("{containers:#?}");

    let (socket_addr, mungos) = config::load().await;

    let app = Router::new().nest("/api", api::router());

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("server crashed");
}
