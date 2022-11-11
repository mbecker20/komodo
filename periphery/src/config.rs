use std::{fs::File, io::Read};

use dotenv::dotenv;
use helpers::parse_config_file;
use serde::Deserialize;
use types::PeripherySecrets;

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_secrets_path")]
    secrets_path: String,
}

pub fn load() -> (u16, PeripherySecrets) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    let secrets = parse_config_file(&env.secrets_path).expect("failed to parse secrets file");
    (env.port, secrets)
}

fn default_port() -> u16 {
    8000
}

fn default_secrets_path() -> String {
    "/secrets/secrets.toml".to_string()
}
