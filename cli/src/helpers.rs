use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    net::IpAddr,
    path::PathBuf,
    str::FromStr,
};

use async_timing_util::Timelength;
use clap::ArgMatches;
use colored::Colorize;
use rand::{distributions::Alphanumeric, Rng};
use run_command::run_command_pipe_to_terminal;
use serde::Serialize;

use crate::types::{CoreConfig, MongoConfig, PeripheryConfig, RestartMode};

const CORE_IMAGE_NAME: &str = "mbecker2020/monitor_core";
const PERIPHERY_IMAGE_NAME: &str = "mbecker2020/monitor_periphery";
const PERIPHERY_CRATE: &str = "monitor_periphery";

pub fn gen_core_config(sub_matches: &ArgMatches) {
    let host = sub_matches
        .get_one::<String>("host")
        .map(|p| p.as_str())
        .unwrap_or("http://localhost:9000")
        .to_string();

    let path = sub_matches
        .get_one::<String>("path")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/core.config.toml")
        .to_string();

    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("9000")
        .parse::<u16>()
        .expect("invalid port");

    let mongo_uri = sub_matches
        .get_one::<String>("mongo-uri")
        .map(|p| p.as_str())
        .unwrap_or("mongodb://monitor-mongo")
        .to_string();

    let mongo_db_name = sub_matches
        .get_one::<String>("mongo-db-name")
        .map(|p| p.as_str())
        .unwrap_or("monitor")
        .to_string();

    let jwt_valid_for = sub_matches
        .get_one::<String>("jwt-valid-for")
        .map(|p| p.as_str())
        .unwrap_or("1-wk")
        .parse()
        .expect("invalid jwt-valid-for");

    let slack_url = sub_matches
        .get_one::<String>("slack-url")
        .map(|p| p.to_owned());

    let config = CoreConfig {
        host,
        port,
        jwt_valid_for,
        monitoring_interval: Timelength::OneMinute,
        daily_offset_hours: 0,
        keep_stats_for_days: 120,
        slack_url,
        local_auth: true,
        github_oauth: Default::default(),
        google_oauth: Default::default(),
        mongo: MongoConfig {
            uri: mongo_uri,
            db_name: mongo_db_name,
            app_name: "monitor".to_string(),
        },
        jwt_secret: generate_secret(40),
        github_webhook_secret: generate_secret(30),
    };

    write_to_toml(&path, &config);

    println!(
        "\n✅ {} has been generated at {path} ✅\n",
        "core config".bold()
    );
}

pub fn start_mongo(sub_matches: &ArgMatches) {
    let username = sub_matches.get_one::<String>("username");
    let password = sub_matches.get_one::<String>("password");

    if (username.is_some() && password.is_none()) {
        println!(
            "\n❌ must provide {} if username is provided ❌\n",
            "--password".bold()
        );
        return;
    }
    if (username.is_none() && password.is_some()) {
        println!(
            "\n❌ must provide {} if password is provided ❌\n",
            "--username".bold()
        );
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

    let restart = sub_matches
        .get_one::<String>("restart")
        .map(|p| p.as_str())
        .unwrap_or("unless-stopped")
        .parse::<RestartMode>()
        .expect("invalid restart mode");

    let env = if let (Some(username), Some(password)) = (username, password) {
        format!(" --env MONGO_INITDB_ROOT_USERNAME={username} --env MONGO_INITDB_ROOT_PASSWORD={password}")
    } else {
        String::new()
    };

    println!(
        "\n====================\n    {}    \n====================\n",
        "mongo config".bold()
    );
    if let Some(username) = username {
        println!("{}: {username}", "mongo username".dimmed());
    }
    println!("{}: {name}", "container name".dimmed());
    println!("{}: {port}", "port".dimmed());
    println!("{}: {mount}", "mount".dimmed());
    println!("{}: {network}", "network".dimmed());
    println!("{}: {restart}", "restart".dimmed());

    println!(
        "\npress {} to start {}. {}",
        "ENTER".green().bold(),
        "MongoDB".bold(),
        "(ctrl-c to cancel)".dimmed()
    );

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    let command = format!("docker run -d --name {name} -p {port}:27017 --network {network} -v {mount}:/data/db{env} --restart {restart} mongo --quiet");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!("\n✅ {} has been started up ✅\n", "monitor mongo".bold())
    } else {
        eprintln!("\n❌ there was some {} on startup ❌\n", "error".red())
    }
}

pub fn start_core(sub_matches: &ArgMatches) {
    let config_path = sub_matches
        .get_one::<String>("config-path")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/core.config.toml")
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

    let restart = sub_matches
        .get_one::<String>("restart")
        .map(|p| p.as_str())
        .unwrap_or("unless-stopped")
        .parse::<RestartMode>()
        .expect("invalid restart mode");

    println!(
        "\n===================\n    {}    \n===================\n",
        "core config".bold()
    );
    println!("{}: {name}", "container name".dimmed());
    println!("{}: {config_path}", "config path".dimmed());
    println!("{}: {port}", "port".dimmed());
    println!("{}: {network}", "network".dimmed());
    println!("{}: {restart}", "restart".dimmed());

    println!(
        "\npress {} to start {}. {}",
        "ENTER".green().bold(),
        "monitor core".bold(),
        "(ctrl-c to cancel)".dimmed()
    );

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    let command = format!("docker run -d --name {name} -p {port}:9000 --network {network} -v {config_path}:/config/config.toml --restart {restart} --add-host host.docker.internal:host-gateway {CORE_IMAGE_NAME}");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!("\n✅ {} has been started up ✅\n", "monitor core".bold())
    } else {
        eprintln!("\n❌ there was some {} on startup ❌\n", "error".red())
    }
}

pub fn gen_periphery_config(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<String>("path")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/periphery.config.toml")
        .to_string();

    let port = sub_matches
        .get_one::<String>("port")
        .map(|p| p.as_str())
        .unwrap_or("8000")
        .parse::<u16>()
        .expect("invalid port");

    let stats_polling_rate = sub_matches
        .get_one::<String>("stats-polling-rate")
        .map(|p| p.as_str())
        .unwrap_or("5-sec")
        .parse::<Timelength>()
        .expect("invalid timelength");

    let allowed_ips = sub_matches
        .get_one::<String>("allowed-ips")
        .map(|p| p.as_str())
        .unwrap_or("")
        .split(",")
        .filter(|ip| ip.len() > 0)
        .map(|ip| {
            ip.parse()
                .expect("given allowed ip address is not valid ip")
        })
        .collect::<Vec<IpAddr>>();

    let repo_dir = sub_matches
        .get_one::<String>("repo-dir")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/repos")
        .to_string()
        .replace("~", env::var("HOME").unwrap().as_str());

    let config = PeripheryConfig {
        port,
        stats_polling_rate,
        allowed_ips,
        repo_dir: "/repos".to_string(),
        secrets: Default::default(),
        github_accounts: Default::default(),
        docker_accounts: Default::default(),
    };

    write_to_toml(&path, &config);

    println!(
        "\n✅ {} generated at {path} ✅\n",
        "periphery config".bold()
    );
}

pub fn start_periphery_daemon(sub_matches: &ArgMatches) {
    let config_path = sub_matches
        .get_one::<String>("config-path")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/periphery.config.toml")
        .to_string();

    let stdout = sub_matches
        .get_one::<String>("stdout")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/periphery.log.out")
        .to_string();

    let stderr = sub_matches
        .get_one::<String>("stderr")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/periphery.log.err")
        .to_string();

    println!(
        "\n========================\n    {}    \n========================\n",
        "periphery config".bold()
    );
    println!("{}: {config_path}", "config path".dimmed());
    println!("{}: {stdout}", "stdout".dimmed());
    println!("{}: {stderr}", "stderr".dimmed());

    println!(
        "\npress {} to start {}. {}",
        "ENTER".green().bold(),
        "monitor periphery".bold(),
        "(ctrl-c to cancel)".dimmed()
    );

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    println!("\ninstalling periphery binary...\n");

    let install_output = run_command_pipe_to_terminal(&format!("cargo install {PERIPHERY_CRATE}"));

    if install_output.success() {
        println!("\ninstallation finished, starting monitor periphery daemon\n")
    } else {
        eprintln!(
            "\n❌ there was some {} during periphery installation ❌\n",
            "error".red()
        );
        return;
    }

    let command = format!("if pgrep periphery; then pkill periphery; fi && periphery --daemon --config-path {config_path} --stdout {stdout} --stderr {stderr}");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!(
            "\n✅ {} has been started up ✅\n",
            "monitor periphery".bold()
        )
    } else {
        eprintln!("\n❌ there was some {} on startup ❌\n", "error".red())
    }
}

pub fn start_periphery_container(sub_matches: &ArgMatches) {
    let config_path = sub_matches
        .get_one::<String>("config-path")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/periphery.config.toml")
        .to_string();

    let repo_dir = sub_matches
        .get_one::<String>("repo-dir")
        .map(|p| p.as_str())
        .unwrap_or("~/.monitor/repos")
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

    let restart = sub_matches
        .get_one::<String>("restart")
        .map(|p| p.as_str())
        .unwrap_or("unless-stopped")
        .parse::<RestartMode>()
        .expect("invalid restart mode");

    println!(
        "\n========================\n    {}    \n========================\n",
        "periphery config".bold()
    );
    println!("{}: {name}", "container name".dimmed());
    println!("{}: {config_path}", "config path".dimmed());
    println!("{}: {repo_dir}", "repo folder".dimmed());
    println!("{}: {port}", "port".dimmed());
    println!("{}: {network}", "network".dimmed());
    println!("{}: {restart}", "restart".dimmed());

    println!(
        "\npress {} to start {}. {}",
        "ENTER".green().bold(),
        "monitor periphery".bold(),
        "(ctrl-c to cancel)".dimmed()
    );

    let buffer = &mut [0u8];
    let res = std::io::stdin().read_exact(buffer);

    if res.is_err() {
        println!("pressed another button, exiting");
    }

    let command = format!("docker run -d --name {name} -p {port}:8000 --network {network} -v {config_path}:/config/config.toml -v {repo_dir}:/repos -v /var/run/docker.sock:/var/run/docker.sock --restart {restart} {PERIPHERY_IMAGE_NAME}");

    let output = run_command_pipe_to_terminal(&command);

    if output.success() {
        println!(
            "\n✅ {} has been started up ✅\n",
            "monitor periphery".bold()
        )
    } else {
        eprintln!("\n❌ there was some {} on startup ❌\n", "error".red())
    }
}

pub fn gen_periphery_service_file(sub_matches: &ArgMatches) {
    let mut file = File::create("/etc/systemd/system/periphery.service")
        .expect("failed to create config file. user must be root for this operation.");
    file.write_all(generate_periphery_unit_file().as_bytes())
        .expect("failed to write config file. user must be root for this operation");

    let output = run_command_pipe_to_terminal("systemctl daemon-reload");

    if output.success() {
        println!("\n✅ successfully added service to systemd ✅\n")
    } else {
        eprintln!(
            "\n❌ there was some {} adding service to systemd ❌\n",
            "error".red()
        )
    }
}

fn write_to_toml(path: &str, toml: impl Serialize) {
    let path = PathBuf::from_str(&path.replace("~", &std::env::var("HOME").unwrap()))
        .expect("not a valid path");
    let _ = fs::create_dir_all(pop_path(&path));
    fs::write(
        path,
        toml::to_string(&toml).expect("failed to parse config into toml"),
    )
    .expect("❌ failed to write toml to file ❌");
}

fn pop_path(path: &PathBuf) -> PathBuf {
    let mut clone = path.clone();
    clone.pop();
    clone
}

fn generate_secret(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn generate_periphery_unit_file() -> String {
    let home = env::var("HOME").expect("failed to find $HOME env var");
    let user = env::var("USER").expect("failed to find $USER env var");
    format!("[Unit]
Description=agent to connect with monitor core
After=network.target

[Service]
Type=simple
User={user}
WorkingDirectory={home}
ExecStart=/bin/bash --login -c 'source {home}/.bashrc; $HOME/.cargo/bin/periphery --config-path ~/.monitor/periphery.config.toml --home-dir $HOME'
TimeoutStartSec=0

[Install]
WantedBy=default.target")
}
