use axum_extra::routing::SpaRouter;
use dotenv::dotenv;
use merge_config_files::parse_config_file;
use mungos::Deserialize;
use types::CoreConfig;

#[derive(Deserialize, Debug)]
struct Env {
    #[serde(default = "default_config_path")]
    pub config_path: String,
    #[serde(default = "default_frontend_path")]
    pub frontend_path: String,
}

pub fn load() -> (CoreConfig, SpaRouter) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse environment variables");
    let config = parse_config_file(env.config_path).expect("failed to parse config");
    let spa_router = SpaRouter::new("/assets", env.frontend_path);
    (config, spa_router)
}

pub fn default_config_path() -> String {
    "/config/config.toml".to_string()
}

fn default_frontend_path() -> String {
    "/frontend".to_string()
}
