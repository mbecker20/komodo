use anyhow::Context;

use crate::config::periphery_config;

pub fn get_github_token(
  github_account: &String,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .github_accounts
    .get(github_account)
    .with_context(|| format!("did not find token in config for github account {github_account}"))
}

pub fn get_docker_token(
  docker_account: &String,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .docker_accounts
    .get(docker_account)
    .with_context(|| format!("did not find token in config for docker account {docker_account}"))
}
