use clap::{Parser, Subcommand};
use monitor_client::api::execute::Execution;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
  /// Sync or Exec
  #[command(subcommand)]
  pub command: Command,
  /// The path to a creds file.
  #[arg(long, default_value_t = default_creds())]
  pub creds: String,
  /// Always continue on user confirmation prompts.
  #[arg(long, short, default_value_t = false)]
  pub yes: bool,
}

fn default_creds() -> String {
  let home = std::env::var("HOME")
    .expect("no HOME env var. cannot get default config path.");
  format!("{home}/.config/monitor/creds.toml")
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
  /// Runs syncs on resource files
  Sync {
    /// The path of the resource folder / file
    /// Folder paths will recursively incorporate all the resources it finds under the folder
    #[arg(default_value_t = String::from("./resources"))]
    path: String,

    /// Will delete any resources that aren't included in the resource files.
    #[arg(long, default_value_t = false)]
    delete: bool,
  },

  /// Runs an execution
  Execute {
    #[command(subcommand)]
    execution: Execution,
  },
}

#[derive(Debug, Deserialize)]
pub struct CredsFile {
  pub url: String,
  pub key: String,
  pub secret: String,
}
