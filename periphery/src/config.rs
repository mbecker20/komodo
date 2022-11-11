use std::{fs::File, io::Read};

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
    let secrets = read_secrets(&env.secrets_path);
    (env.port, secrets)
}

fn default_port() -> u16 {
    8000
}

fn default_secrets_path() -> String {
    "/secrets/secrets.toml".to_string()
}

fn read_secrets(secrets_path: &str) -> PeripherySecrets {
    let mut secrets_file = File::open(&secrets_path).expect("failed to find secrets");
    if secrets_path.ends_with("toml") {
        let mut contents = String::new();
        secrets_file.read_to_string(&mut contents);
        toml::from_str(&contents).expect("failed to parse secrets toml") 
    } else if secrets_path.ends_with("json") {
        serde_json::from_reader(secrets_file).expect("failed to parse secrets json")
    } else {
        panic!("unsupported secrets file type: {}", secrets_path)
    }
}