use std::sync::Arc;

use axum::Extension;
use dotenv::dotenv;
use helpers::parse_config_file;
use serde::Deserialize;
use types::PeripheryConfig;

use crate::PeripheryConfigExtension;

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_config_path")]
    config_path: String,
}

pub fn load() -> (u16, PeripheryConfigExtension) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    let config: PeripheryConfig =
        parse_config_file(&env.config_path).expect("failed to parse config file");
    let _ = std::fs::create_dir(&config.repo_dir);
    print_startup_log(&config);
    (config.port, Extension(Arc::new(config)))
}

fn print_startup_log(config: &PeripheryConfig) {
    println!("starting montior periphery on port {}", config.port);
}

fn default_config_path() -> String {
    "/config/config.toml".to_string()
}
