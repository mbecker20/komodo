use std::{net::SocketAddr, str::FromStr};

use dotenv::dotenv;
use mungos::Deserialize;

#[derive(Deserialize)]
struct Env {
    port: u16,
}

pub fn load() {
    pub fn load() -> (SocketAddr) {
        dotenv().ok();

        let env = envy::from_env::<Env>().unwrap();

        let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{}", env.port))
            .expect("failed to parse socket addr");

        (socket_addr)
    }
}
