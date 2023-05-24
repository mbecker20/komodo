use std::sync::Arc;

use axum::Extension;
use clap::Parser;
use dotenv::dotenv;
use merge_config_files::parse_config_paths;
use parse_csl::parse_comma_seperated;
use serde::Deserialize;
use types::PeripheryConfig;

use crate::{HomeDirExtension, PeripheryConfigExtension};

#[derive(Parser)]
#[command(author, about, version)]
pub struct Args {
    /// Sets the path of a config file or directory to use. can use multiple times
    #[arg(short, long)]
    pub config_path: Option<Vec<String>>,

    /// Sets the keywords to match directory periphery config file names on. can use multiple times. default "periphery" and "config"
    #[arg(long)]
    pub config_keyword: Option<Vec<String>>,

    /// Merges nested configs, eg. secrets, docker_accounts, github_accounts
    #[arg(long)]
    pub merge_nested_config: bool,

    /// Extends config arrays, eg. allowed_ips, passkeys
    #[arg(long)]
    pub extend_config_arrays: bool,

    /// Sets the periphery home directory to use instead of $HOME
    /// ~ will be replaced with this directory
    #[arg(long)]
    pub home_dir: Option<String>,
}

#[derive(Deserialize)]
struct Env {
    #[serde(default = "default_config_path")]
    config_paths: String,
    #[serde(default)]
    config_keywords: String,
}

pub fn load() -> (u16, PeripheryConfigExtension, HomeDirExtension) {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    let args = Args::parse();
    let home_dir = get_home_dir(&args.home_dir);
    let config_paths = args
        .config_path
        .as_ref()
        .unwrap_or(
            &parse_comma_seperated(&env.config_paths)
                .expect("failed to parse config paths on environment into comma seperated list"),
        )
        .into_iter()
        .map(|p| p.replace("~", &home_dir))
        .collect::<Vec<_>>();
    let env_match_keywords = parse_comma_seperated(&env.config_keywords)
        .expect("failed to parse environemt CONFIG_KEYWORDS into comma seperated list");
    let match_keywords = args
        .config_keyword
        .as_ref()
        .unwrap_or(&env_match_keywords)
        .into_iter()
        .map(|kw| kw.as_str());
    let config = parse_config_paths::<PeripheryConfig>(
        config_paths.clone(),
        match_keywords,
        args.merge_nested_config,
        args.extend_config_arrays,
    )
    .expect("failed at parsing config");
    let _ = std::fs::create_dir(&config.repo_dir);
    print_startup_log(config_paths, &config);
    (
        config.port,
        Extension(Arc::new(config)),
        Extension(Arc::new(home_dir)),
    )
}

fn print_startup_log(config_paths: Vec<String>, config: &PeripheryConfig) {
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
    println!("starting montior periphery on port {}\n", config.port);
}

fn default_config_path() -> String {
    "~/.monitor/periphery.config.toml".to_string()
}

fn get_home_dir(home_dir_arg: &Option<String>) -> String {
    match home_dir_arg {
        Some(home_dir) => home_dir.to_string(),
        None => std::env::var("HOME")
            .expect("did not find $HOME env var, should pass home dir with arg --home-dir"),
    }
}
