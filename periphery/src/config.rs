use std::sync::Arc;

use axum::Extension;
use clap::Parser;
use dotenv::dotenv;
use helpers::parse_config_file;
use serde::Deserialize;
use types::PeripheryConfig;

use crate::PeripheryConfigExtension;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub daemon: bool,
    #[arg(long, default_value = "~/.monitor/periphery.log.out")]
    pub stdout: String,
    #[arg(long, default_value = "~/.monitor/periphery.log.err")]
    pub stderr: String,
}

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_config_path")]
    config_path: String,
}

pub fn load() -> (Args, u16, PeripheryConfigExtension) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    let args = Args::parse();
    let config: PeripheryConfig =
        parse_config_file(&env.config_path).expect("failed to parse config file");
    let _ = std::fs::create_dir(&config.repo_dir);
    print_startup_log(&args, &config);
    (args, config.port, Extension(Arc::new(config)))
}

fn print_startup_log(args: &Args, config: &PeripheryConfig) {
    println!("starting montior periphery on port {}", config.port);
    if args.daemon {
        println!("daemonize mode enabled");
    }
}

fn default_config_path() -> String {
    "/config/config.toml".to_string()
}
