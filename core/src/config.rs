use dotenv::dotenv;
use helpers::parse_config_file;
use mungos::Deserialize;
use types::CoreConfig;

#[derive(Deserialize, Debug)]
struct Env {
    #[serde(default = "default_config_path")]
    pub config_path: String,
}

pub fn load() -> CoreConfig {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse environment variables");
    let config = parse_config_file(&env.config_path).expect("failed to parse config");
    config
}

pub fn default_config_path() -> String {
    "/config/config.toml".to_string()
}
