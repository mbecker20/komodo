use std::{
    fs::{self, File},
    io::Write,
};

use async_timing_util::Timelength;
use clap::ArgMatches;
use monitor_types::{CoreConfig, MongoConfig, PeripheryConfig};
use rand::{distributions::Alphanumeric, Rng};
use serde::Serialize;

pub fn gen_core_config(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<String>("path")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/config.toml")
        .to_string();
    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("9000")
        .parse::<u16>()
        .expect("invalid port");
    let mongo_uri = sub_matches
        .get_one::<String>("mongo_uri")
        .map(|p| p.as_str())
        .unwrap_or("mongodb://mongo")
        .to_string();
    let mongo_db_name = sub_matches
        .get_one::<String>("mongo_db_name")
        .map(|p| p.as_str())
        .unwrap_or("monitor")
        .to_string();
    let jwt_valid_for = sub_matches
        .get_one::<String>("jwt_valid_for")
        .map(|p| p.as_str())
        .unwrap_or("1-wk")
        .parse()
        .expect("invalid jwt_valid_for");
    let slack_url = sub_matches
        .get_one::<String>("slack_url")
        .map(|p| p.to_owned());

    let config = CoreConfig {
        port,
        jwt_valid_for,
        slack_url,
        github_oauth: Default::default(),
        mongo: MongoConfig {
            uri: mongo_uri,
            db_name: mongo_db_name,
            app_name: "monitor".to_string(),
        },
        jwt_secret: generate_secret(40),
        github_webhook_secret: generate_secret(30),
    };

    write_to_toml(&path, config);

    println!("\ncore config has been generated ✅");
}

pub fn start_mongo(sub_matches: &ArgMatches) {}

pub fn start_core(sub_matches: &ArgMatches) {}

pub fn gen_periphery_config(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<String>("path")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/config.toml")
        .to_string();
    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("9000")
        .parse::<u16>()
        .expect("invalid port");
    let repo_dir = sub_matches
        .get_one::<String>("repo_dir")
        .map(|p| p.as_str())
        .unwrap_or("/repos")
        .to_string();

    let config = PeripheryConfig {
        port,
        repo_dir,
        secrets: Default::default(),
        github_accounts: Default::default(),
        docker_accounts: Default::default(),
    };

    write_to_toml(&path, config);

    println!("\nperiphery config has been generated ✅");
}

pub fn start_periphery(sub_matches: &ArgMatches) {}

fn write_to_toml(path: &str, toml: impl Serialize) {
    fs::write(
        path,
        toml::to_string(&toml).expect("failed to parse config into toml"),
    )
    .expect("failed to write toml to file");
}

fn generate_secret(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
