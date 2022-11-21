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
    print_startup_log(&config);
    config
}

fn print_startup_log(config: &CoreConfig) {
    println!("starting monitor core on port {}", config.port);
    if config.github_webhook_secret.is_none() {
        println!("\nNOTE: you have not configured a github_webhook_secret. this is optional, but recommended if you use github repo webhooks")
    }
}

pub fn default_config_path() -> String {
    "/config/config.json".to_string()
}
