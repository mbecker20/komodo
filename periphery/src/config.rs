use std::sync::Arc;

use axum::Extension;
use clap::Parser;
use dotenv::dotenv;
use helpers::parse_config_file;
use serde::Deserialize;
use types::PeripheryConfig;

use crate::PeripheryConfigExtension;

#[derive(Parser)]
#[command(author = "mbecker20 <becker.maxh@gmail.com>")]
#[command(about = "monitor periphery client")]
pub struct Args {
    /// Run this program as a system daemon
    #[arg(short, long)]
    pub daemon: bool,

    /// Sets destination file of periphery stdout logs
    #[arg(long, default_value = "~/.monitor/periphery.log.out")]
    pub stdout: String,

    /// Sets destination file of periphery stderr logs
    #[arg(long, default_value = "~/.monitor/periphery.log.err")]
    pub stderr: String,

    /// Sets the path of config file to use
    #[arg(short, long)]
    pub config_path: Option<String>,
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
    let config_path = args.config_path.as_ref().unwrap_or(&env.config_path);
    let config =
        parse_config_file::<PeripheryConfig>(config_path).expect("failed to parse config file");
    let _ = std::fs::create_dir(&config.repo_dir);
    print_startup_log(config_path, &args, &config);
    (args, config.port, Extension(Arc::new(config)))
}

fn print_startup_log(config_path: &str, args: &Args, config: &PeripheryConfig) {
    println!("\nconfig path: {config_path}");
    println!("starting montior periphery on port {}", config.port);
    if args.daemon {
        println!("daemon mode enabled\n");
    }
}

fn default_config_path() -> String {
    "/config/periphery.config.toml".to_string()
}
