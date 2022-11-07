mod config;

#[tokio::main]
async fn main() {
    let (socket_addr) = config::load();
}
