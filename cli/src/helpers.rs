use std::{
    fs::{self, File},
    io::{Read, Write},
};

use async_timing_util::Timelength;
use clap::ArgMatches;
use monitor_types::{CoreConfig, MongoConfig, PeripheryConfig};
use rand::{distributions::Alphanumeric, Rng};
use run_command::run_command_pipe_to_terminal;
use serde::Serialize;

const CORE_IMAGE_NAME: &str = "mbecker20/monitor-core";
const PERIPHERY_IMAGE_NAME: &str = "mbecker20/monitor-periphery";

pub fn gen_core_config(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<String>("path")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/core.config.toml")
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

    println!("\n✅ core config has been generated ✅\n");
}

pub fn start_mongo(sub_matches: &ArgMatches) {
    let username = sub_matches.get_one::<String>("username");
    let password = sub_matches.get_one::<String>("password");

    if (username.is_some() && password.is_none()) {
        println!("❌ must provide --password if username is provided ❌");
        return;
    }
    if (username.is_none() && password.is_some()) {
        println!("❌ must provide --username if password is provided ❌");
        return;
    }

    let name = sub_matches
        .get_one::<String>("name")
        .map(|p| p.as_str())
        .unwrap_or("monitor-mongo");

    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("27017")
        .parse::<u16>()
        .expect("invalid port");

    let network = sub_matches
        .get_one::<String>("network")
        .map(|p| p.as_str())
        .unwrap_or("bridge");

    let mount = sub_matches
        .get_one::<String>("mount")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/db");

    let env = if username.is_some() && password.is_some() {
        let (username, password) = (username.unwrap(), password.unwrap());
        format!(" --env MONGO_INITDB_ROOT_USERNAME={username} --env MONGO_INITDB_ROOT_PASSWORD={password}")
    } else {
        String::new()
    };

    println!("\n==============\n mongo config \n==============\n");
    println!("name: {name}");
    println!("port: {port}");
    println!("mount: {mount}");
    println!("network: {network}");

    println!("\npress ENTER to start mongo");

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    let command = format!("docker run -d --name {name} -p {port}:27017 --network {network} -v {mount}:/data/db{env} mongo --quiet");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!("\n✅ monitor mongo has been started up ✅\n")
    } else {
        eprintln!("\n❌ there was some error on startup ❌\n")
    }
}

pub fn start_core(sub_matches: &ArgMatches) {
    let config_path = sub_matches
        .get_one::<String>("config_path")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/core.config.toml")
        .to_string();

    let name = sub_matches
        .get_one::<String>("name")
        .map(|p| p.as_str())
        .unwrap_or("monitor-core");

    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("9000")
        .parse::<u16>()
        .expect("invalid port");

    let network = sub_matches
        .get_one::<String>("network")
        .map(|p| p.as_str())
        .unwrap_or("bridge");

    println!("\n=============\n core config \n=============\n");
    println!("name: {name}");
    println!("config path: {config_path}");
    println!("port: {port}");
    println!("network: {network}");

    println!("\npress ENTER to start monitor core");

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    let command = format!("docker run -d --name {name} -p {port}:9000 --network {network} -v {config_path}:/config/config.toml {CORE_IMAGE_NAME}");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!("\n✅ monitor core has been started up ✅\n")
    } else {
        eprintln!("\n❌ there was some error on startup ❌\n")
    }
}

pub fn gen_periphery_config(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<String>("path")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/periphery.config.toml")
        .to_string();

    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("9000")
        .parse::<u16>()
        .expect("invalid port");

    let config = PeripheryConfig {
        port,
        repo_dir: "/repos".to_string(),
        secrets: Default::default(),
        github_accounts: Default::default(),
        docker_accounts: Default::default(),
    };

    write_to_toml(&path, config);

    println!("\n✅ periphery config has been generated ✅\n");
}

pub fn start_periphery(sub_matches: &ArgMatches) {
    let config_path = sub_matches
        .get_one::<String>("config_path")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/periphery.config.toml")
        .to_string();

    let repo_dir = sub_matches
        .get_one::<String>("repo_dir")
        .map(|p| p.as_str())
        .unwrap_or("$HOME/.monitor/repos")
        .to_string();

    let name = sub_matches
        .get_one::<String>("name")
        .map(|p| p.as_str())
        .unwrap_or("monitor-periphery");

    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("8000")
        .parse::<u16>()
        .expect("invalid port");

    let network = sub_matches
        .get_one::<String>("network")
        .map(|p| p.as_str())
        .unwrap_or("bridge");

    println!("\n==================\n periphery config \n==================\n");
    println!("name: {name}");
    println!("config path: {config_path}");
    println!("repo dir: {repo_dir}");
    println!("port: {port}");
    println!("network: {network}");

    println!("\npress ENTER to start monitor periphery");

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    let command = format!("docker run -d --name {name} -p {port}:8000 --network {network} -v {config_path}:/config/config.toml -v {repo_dir}:/repos -v /var/run/docker.sock:/var/run/docker.sock {PERIPHERY_IMAGE_NAME}");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!("\n✅ monitor periphery has been started up ✅\n")
    } else {
        eprintln!("\n❌ there was some error on startup ❌\n")
    }
}

fn write_to_toml(path: &str, toml: impl Serialize) {
    fs::write(
        path,
        toml::to_string(&toml).expect("failed to parse config into toml"),
    )
    .expect("❌ failed to write toml to file ❌");
}

fn generate_secret(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
