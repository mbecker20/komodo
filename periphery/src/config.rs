use std::{fs::File, io::Read, sync::Arc};

use axum::Extension;
use dotenv::dotenv;
use helpers::parse_config_file;
use serde::Deserialize;
use types::PeripherySecrets;

use crate::PeripherySecretsExtension;

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_secrets_path")]
    secrets_path: String,
}

pub fn load() -> (u16, PeripherySecretsExtension) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    let secrets = parse_config_file(&env.secrets_path).expect("failed to parse secrets file");
    (env.port, Extension(Arc::new(secrets)))
}

fn default_port() -> u16 {
    8000
}

fn default_secrets_path() -> String {
    "/secrets/secrets.toml".to_string()
}
