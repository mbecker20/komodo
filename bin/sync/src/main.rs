#![allow(unused)]

use std::collections::HashMap;

use anyhow::Context;
use clap::Parser;
use monitor_client::{
  entities::{build::Build, deployment::Deployment, server::Server},
  MonitorClient,
};
use serde::Deserialize;

#[derive(Parser)]
#[command(author, about, version)]
pub struct MonitorSyncArgs {
  /// The root path of the sync files
  #[arg(short, long)]
  pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MonitorSyncFile {
  pub servers: HashMap<String, Server>,
  pub deployments: HashMap<String, Deployment>,
  pub builds: HashMap<String, Build>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv::dotenv().ok();

  let args = MonitorSyncArgs::parse();

  let sync_file: MonitorSyncFile =
    merge_config_files::parse_config_file(args.path).unwrap();

  let monitor = MonitorClient::new_from_env()
    .await
    .context("failed to initialize monitor client")?;

  Ok(())
}
