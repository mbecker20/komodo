use anyhow::anyhow;
use types::{DockerToken, GithubToken, PeripheryConfig};

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

pub fn get_github_token(
    github_account: &Option<String>,
    config: &PeripheryConfig,
) -> anyhow::Result<Option<GithubToken>> {
    match github_account {
        Some(account) => match config.github_accounts.get(account) {
            Some(token) => Ok(Some(token.to_owned())),
            None => Err(anyhow!(
                "did not find token in config for github account {account} "
            )),
        },
        None => Ok(None),
    }
}

pub fn get_docker_token(
    docker_account: &Option<String>,
    config: &PeripheryConfig,
) -> anyhow::Result<Option<DockerToken>> {
    match docker_account {
        Some(account) => match config.docker_accounts.get(account) {
            Some(token) => Ok(Some(token.to_owned())),
            None => Err(anyhow!(
                "did not find token in config for docker account {account} "
            )),
        },
        None => Ok(None),
    }
}
