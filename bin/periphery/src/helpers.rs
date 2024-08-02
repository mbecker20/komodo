use anyhow::Context;
use command::run_monitor_command;
use formatting::format_serror;
use monitor_client::entities::{update::Log, EnvironmentVar};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::fs;

use crate::config::periphery_config;

pub fn get_git_token(
  domain: &str,
  account_username: &str,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .git_providers
    .iter()
    .find(|_provider| _provider.domain == domain)
    .and_then(|provider| provider.accounts
        .iter()
          .find(|account| account.username == account_username).map(|account| &account.token))
    .with_context(|| format!("did not find token in config for git account {account_username} | domain {domain}"))
}

pub fn get_docker_token(
  domain: &str,
  account_username: &str,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .docker_registries
    .iter().find(|registry| registry.domain == domain)
    .and_then(|registry| registry.accounts.iter().find(|account| account.username == account_username).map(|account| &account.token))
    .with_context(|| format!("did not find token in config for docker account {account_username} | domain {domain}"))
}

pub fn parse_extra_args(extra_args: &[String]) -> String {
  let args = extra_args.join(" ");
  if !args.is_empty() {
    format!(" {args}")
  } else {
    args
  }
}

pub fn parse_labels(labels: &[EnvironmentVar]) -> String {
  labels
    .iter()
    .map(|p| format!(" --label {}=\"{}\"", p.variable, p.value))
    .collect::<Vec<_>>()
    .join("")
}

pub async fn run_stack_command(
  file_contents: &str,
  stage: &str,
  command: String,
) -> Vec<Log> {
  let dir = periphery_config().repo_dir.join(random_string(10));
  if let Err(e) = fs::create_dir_all(&dir)
    .await
    .with_context(|| format!("directory: {dir:?}"))
    .context("failed to create directory for compose file")
  {
    return vec![Log::error(
      "create compose directory",
      format_serror(&e.into()),
    )];
  }
  let file_path = dir.join("compose.yaml");
  if let Err(e) = fs::write(&file_path, file_contents)
    .await
    .with_context(|| format!("file: {file_path:?}"))
    .context("failed to write compose file")
  {
    return vec![Log::error(
      "write compose file",
      format_serror(&e.into()),
    )];
  }
  let mut logs = vec![
    run_monitor_command(
      stage,
      format!("cd {} && {command}", dir.display()),
    )
    .await,
  ];

  if let Err(e) = fs::remove_dir_all(&dir)
    .await
    .with_context(|| format!("directory: {dir:?}"))
    .context("failed to remove compose directory")
  {
    error!("{e:#}");
    logs
      .push(Log::error("post run cleanup", format_serror(&e.into())));
  }

  logs
}

pub fn random_string(length: usize) -> String {
  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect()
}
