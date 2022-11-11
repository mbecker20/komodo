use std::sync::Arc;

use ::helpers::parse_config_file;
use axum::Extension;
use dotenv::dotenv;
use mungos::Deserialize;

use crate::BuilderSecretsExtension;

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_config_path")]
    secrets_path: String,
}

pub fn load() -> (u16, BuilderSecretsExtension) {
    dotenv().ok();

    let env = envy::from_env::<Env>().unwrap();

    let secrets = parse_config_file(&env.secrets_path).expect("failed to parse config");

    (env.port, Extension(Arc::new(secrets)))
}

fn default_port() -> u16 {
    9001
}

fn default_config_path() -> String {
    "/secrets/secrets.json".to_string()
}
