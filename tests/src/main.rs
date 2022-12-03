#![allow(unused)]

mod config;

#[tokio::main]
async fn main() {
    let monitor = config::load().await;
    let servers = monitor
        .list_servers()
        .await
        .expect("failed at list servers");
    let server = servers.get(0).unwrap();
    let server_id = server.id.unwrap().to_string();
    let stats = monitor.get_server_stats(&server_id).await.unwrap();
    println!("{stats:#?}")
}
