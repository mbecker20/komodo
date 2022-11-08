use std::fs::File;

use dotenv::dotenv;
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
    let secrets_file = File::open(&env.secrets_path).expect("failed to find secrets");
    let secrets: PeripherySecrets =
        serde_json::from_reader(secrets_file).expect("failed to parse secrets file");
    (env.port, secrets)
}

fn default_port() -> u16 {
    8000
}

fn default_secrets_path() -> String {
    "/secrets/secrets.json".to_string()
}
