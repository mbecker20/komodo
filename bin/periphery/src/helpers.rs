use anyhow::anyhow;

use crate::config::periphery_config;

pub fn get_github_token(
  github_account: &Option<String>,
) -> anyhow::Result<Option<String>> {
  match github_account {
    Some(account) => {
      match periphery_config().github_accounts.get(account) {
        Some(token) => Ok(Some(token.to_owned())),
        None => Err(anyhow!(
          "did not find token in config for github account {account}"
        )),
      }
    }
    None => Ok(None),
  }
}

pub fn get_docker_token(
  docker_account: &Option<String>,
) -> anyhow::Result<Option<String>> {
  match docker_account {
    Some(account) => {
      match periphery_config().docker_accounts.get(account) {
        Some(token) => Ok(Some(token.to_owned())),
        None => Err(anyhow!(
          "did not find token in config for docker account {account}"
        )),
      }
    }
    None => Ok(None),
  }
}
