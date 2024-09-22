use anyhow::Context;
use komodo_client::entities::{EnvironmentVar, SearchCombinator};

use crate::config::periphery_config;

pub fn git_token(
  domain: &str,
  account_username: &str,
) -> anyhow::Result<&'static str> {
  periphery_config()
    .git_providers
    .iter()
    .find(|provider| provider.domain == domain)
    .and_then(|provider| {
      provider.accounts.iter().find(|account| account.username == account_username).map(|account| account.token.as_str())
    })
    .with_context(|| format!("did not find token in config for git account {account_username} | domain {domain}"))
}

pub fn registry_token(
  domain: &str,
  account_username: &str,
) -> anyhow::Result<&'static str> {
  periphery_config()
    .docker_registries
    .iter()
    .find(|registry| registry.domain == domain)
    .and_then(|registry| {
      registry.accounts.iter().find(|account| account.username == account_username).map(|account| account.token.as_str())
    })
    .with_context(|| format!("did not find token in config for docker registry account {account_username} | domain {domain}"))
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

pub fn log_grep(
  terms: &[String],
  combinator: SearchCombinator,
  invert: bool,
) -> String {
  let maybe_invert = invert.then_some(" -v").unwrap_or_default();
  match combinator {
    SearchCombinator::Or => {
      format!("grep{maybe_invert} -E '{}'", terms.join("|"))
    }
    SearchCombinator::And => {
      format!(
        "grep{maybe_invert} -P '^(?=.*{})'",
        terms.join(")(?=.*")
      )
    }
  }
}

pub async fn ensure_certs() {
  let config = periphery_config();
  if !config.ssl_cert.is_file() || !config.ssl_key.is_file() {
    generate_self_signed_ssl_certs().await
  }
}

#[instrument]
async fn generate_self_signed_ssl_certs() {
  info!("generating certs...");

  let config = periphery_config();

  // ensure cert folders exist
  if let Some(parent) = config.ssl_key.parent() {
    let _ = std::fs::create_dir_all(parent);
  }
  if let Some(parent) = config.ssl_cert.parent() {
    let _ = std::fs::create_dir_all(parent);
  }

  let key_path = &config.ssl_key.display();
  let cert_path = &config.ssl_cert.display();

  let command = format!("openssl req -x509 -newkey rsa:4096 -keyout {key_path} -out {cert_path} -sha256 -days 3650 -nodes -subj \"/C=XX/CN=periphery\"");
  let log = run_command::async_run_command(&command).await;

  if log.success() {
    info!("✅ SSL Certs generated");
  } else {
    panic!(
      "🚨 Failed to generate SSL Certs | stdout: {} | stderr: {}",
      log.stdout, log.stderr
    );
  }
}
