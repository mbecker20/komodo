use std::{net::SocketAddr, str::FromStr, sync::Arc};

use axum::Extension;
use dotenv::dotenv;
use ::helpers::parse_config_file;
use mungos::Deserialize;

use crate::BuilderSecretsExtension;

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_config_path")]
    secrets_path: String,
}

pub fn load() -> (SocketAddr, BuilderSecretsExtension) {
    dotenv().ok();

    let env = envy::from_env::<Env>().unwrap();

    let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{}", env.port))
        .expect("failed to parse socket addr");

    let secrets = parse_config_file(&env.secrets_path).expect("failed to parse config");

    (socket_addr, Extension(Arc::new(secrets)))
}

fn default_port() -> u16 {
    9001
}

fn default_config_path() -> String {
    "/secrets/secrets.json".to_string()
}
