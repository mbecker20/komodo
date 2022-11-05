use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use dotenv::dotenv;
use mungos::{Deserialize, Mungos};

#[derive(Deserialize, Debug)]
struct Env {
    port: u16,
    mongo_uri: String,
}

pub async fn load() -> (SocketAddr, Mungos) {
    dotenv().ok();

    let env = envy::from_env::<Env>().unwrap();

    let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{}", env.port))
        .expect("failed to parse socket addr");

    let mungos = Mungos::new(&env.mongo_uri, "monitor_core", Duration::from_secs(3), None)
        .await
        .expect("failed to connect to mongo");

    (socket_addr, mungos)
}
