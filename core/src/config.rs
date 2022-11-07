use std::{
    env,
    fs::File,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use db::{DbClient, DbExtension};
use dotenv::dotenv;
use mungos::{Deserialize, Mungos};
use types::CoreConfig;

pub fn load() -> CoreConfig {
    let config_path = env::var("CONFIG_PATH").unwrap_or("./config.json".to_string());
    let file = File::open(config_path).expect("failed to open config file");
    serde_json::from_reader(file).unwrap()
}
