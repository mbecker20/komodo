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
  ///
  /// Note: If each of `url`, `key` and `secret` are passed,
  /// no file is required at this path.
  #[arg(long, default_value_t = default_creds())]
  pub creds: String,

  /// Pass url in args instead of creds file
  #[arg(long)]
  pub url: Option<String>,
  /// Pass api key in args instead of creds file
  #[arg(long)]
  pub key: Option<String>,
  /// Pass api secret in args instead of creds file
  #[arg(long)]
  pub secret: Option<String>,

  /// Always continue on user confirmation prompts.
  #[arg(long, short, default_value_t = false)]
  pub yes: bool,
}

fn default_creds() -> String {
  let home =
    std::env::var("HOME").unwrap_or_else(|_| String::from("/root"));
  format!("{home}/.config/monitor/creds.toml")
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
  /// Runs an execution
  Execute {
    #[command(subcommand)]
    execution: Execution,
  },
  // Room for more
}

#[derive(Debug, Deserialize)]
pub struct CredsFile {
  pub url: String,
  pub key: String,
  pub secret: String,
}
