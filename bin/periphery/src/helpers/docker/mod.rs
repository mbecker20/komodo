use anyhow::anyhow;
use monitor_client::entities::update::Log;
use run_command::async_run_command;

use super::run_monitor_command;

pub mod build;
pub mod client;
pub mod container;
pub mod network;

pub async fn prune_images() -> Log {
  let command = String::from("docker image prune -a -f");
  run_monitor_command("prune images", command).await
}

pub fn get_docker_username_pw(
  docker_account: &Option<String>,
  docker_token: &Option<String>,
) -> anyhow::Result<Option<(String, String)>> {
  match docker_account {
    Some(docker_account) => match docker_token {
      Some(docker_token) => Ok(Some((docker_account.to_owned(), docker_token.to_owned()))),
      None => Err(anyhow!(
        "docker token for account {docker_account} has not been configured on this client"
      )),
    },
    None => Ok(None),
  }
}

pub async fn docker_login(
  docker_account: &Option<String>,
  docker_token: &Option<String>,
) -> anyhow::Result<bool> {
  let docker_account_u_pw =
    get_docker_username_pw(docker_account, docker_token)?;
  if let Some((username, password)) = &docker_account_u_pw {
    let login = format!("docker login -u {username} -p {password}");
    async_run_command(&login).await;
    Ok(true)
  } else {
    Ok(false)
  }
}

pub fn parse_extra_args(extra_args: &[String]) -> String {
  let args = extra_args.join(" ");
  if !args.is_empty() {
    format!(" {args}")
  } else {
    args
  }
}

pub async fn prune_system() -> Log {
  let command = String::from("docker system prune -a -f");
  run_monitor_command("prune system", command).await
}
