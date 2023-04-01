use std::sync::Arc;

use axum::Extension;
use clap::Parser;
use dotenv::dotenv;
use helpers::parse_comma_seperated_list;
use merge_config_files::parse_config_files;
use serde::Deserialize;
use types::PeripheryConfig;

use crate::{HomeDirExtension, PeripheryConfigExtension};

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

    /// Sets the path of a config file to use. can use multiple times
    #[arg(short, long)]
    pub config_path: Option<Vec<String>>,

    #[arg(short, long)]
    pub merge_nested_config: bool,

    #[arg(short, long)]
    pub home_dir: Option<String>,

    #[arg(short, long)]
    version: bool,
}

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_config_path")]
    config_paths: String,
}

pub fn load() -> (Args, u16, PeripheryConfigExtension, HomeDirExtension) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    let args = Args::parse();
    if args.version {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0)
    }
    let home_dir = get_home_dir(&args.home_dir);
    let config_paths = args
        .config_path
        .as_ref()
        .unwrap_or(
            &parse_comma_seperated_list(env.config_paths)
                .expect("failed to parse config paths on environment into comma seperated list"),
        )
        .into_iter()
        .map(|p| p.replace("~", &home_dir))
        .collect::<Vec<_>>();
    let config = parse_config_files::<PeripheryConfig>(
        config_paths.clone(),
        args.merge_nested_config,
        args.merge_nested_config,
    )
    .expect("failed at parsing config");
    let _ = std::fs::create_dir(&config.repo_dir);
    print_startup_log(config_paths, &args, &config);
    (
        args,
        config.port,
        Extension(Arc::new(config)),
        Extension(Arc::new(home_dir)),
    )
}

fn print_startup_log(config_paths: Vec<String>, args: &Args, config: &PeripheryConfig) {
    println!("\nconfig paths: {config_paths:?}");
    let mut config_for_print = config.clone();
    config_for_print.github_accounts = config_for_print
        .github_accounts
        .into_iter()
        .map(|(a, _)| (a, "<SECRET>".to_string()))
        .collect();
    config_for_print.docker_accounts = config_for_print
        .docker_accounts
        .into_iter()
        .map(|(a, _)| (a, "<SECRET>".to_string()))
        .collect();
    config_for_print.secrets = config_for_print
        .secrets
        .into_iter()
        .map(|(s, _)| (s, "<SECRET>".to_string()))
        .collect();
    config_for_print.passkeys = config_for_print
        .passkeys
        .into_iter()
        .map(|_| "<SECRET>".to_string())
        .collect();
    println!("{config_for_print:#?}");
    if args.daemon {
        println!("daemon mode enabled");
    }
    println!("starting montior periphery on port {}\n", config.port);
}

fn default_config_path() -> String {
    "~/.monitor/periphery.config.toml".to_string()
}

fn get_home_dir(home_dir_arg: &Option<String>) -> String {
    match home_dir_arg {
        Some(home_dir) => home_dir.to_string(),
        None => std::env::var("$HOME")
            .expect("did not find $HOME env var, should pass home dir with arg --home-dir"),
    }
}
