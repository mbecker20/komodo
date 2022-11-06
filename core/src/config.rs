use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use db::{DbClient, DbExtension};
use dotenv::dotenv;
use mungos::{Deserialize, Mungos};

#[derive(Deserialize, Debug)]
struct Env {
    port: u16,
    mongo_uri: String,
    #[serde(default = "default_mongo_app_name")]
    mongo_app_name: String,
    #[serde(default = "default_mongo_db_name")]
    mongo_db_name: String,
}

pub async fn load() -> (SocketAddr, DbExtension) {
    dotenv().ok();

    let env = envy::from_env::<Env>().unwrap();

    let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{}", env.port))
        .expect("failed to parse socket addr");

    let db_client = DbClient::new(&env.mongo_uri, &env.mongo_app_name, &env.mongo_db_name)
        .await
        .expect("failed to initialize db client");

    (socket_addr, db_client)
}

fn load_secrets() {}

fn default_mongo_app_name() -> String {
    "monitor_core".to_string()
}

fn default_mongo_db_name() -> String {
    "monitor".to_string()
}
