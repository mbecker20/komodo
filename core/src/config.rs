use std::{
    fs::File,
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use db::{DbClient, DbExtension};
use dotenv::dotenv;
use mungos::{Deserialize, Mungos};
use types::CoreConfig;

pub async fn load() -> CoreConfig {
    let config = load_config();

    config
}

fn load_config() -> CoreConfig {
    let file = File::open("/secrets/secrets.json");
    todo!()
}
