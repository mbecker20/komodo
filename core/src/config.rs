use std::{
    env,
    fs::File,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use db::{DbClient, DbExtension};
use dotenv::dotenv;
use helpers::parse_config_file;
use mungos::{Deserialize, Mungos};
use types::CoreConfig;

#[derive(Deserialize, Debug)]
struct Env {
    #[serde(default = "default_config_path")]
    pub config_path: String,
}

pub fn load() -> CoreConfig {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse environment variables");
    parse_config_file(&env.config_path).expect("failed to parse config")
}

pub fn default_config_path() -> String {
    "/config/config.json".to_string()
}
