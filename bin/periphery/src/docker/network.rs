use command::run_monitor_command;
use monitor_client::entities::update::Log;

#[instrument]
pub async fn create_network(
  name: &str,
  driver: Option<String>,
) -> Log {
  let driver = match driver {
    Some(driver) => format!(" -d {driver}"),
    None => String::new(),
  };
  let command = format!("docker network create{driver} {name}");
  run_monitor_command("create network", command).await
}

#[instrument]
pub async fn delete_network(name: &str) -> Log {
  let command = format!("docker network rm {name}");
  run_monitor_command("delete network", command).await
}

#[instrument]
pub async fn prune_networks() -> Log {
  let command = String::from("docker network prune -f");
  run_monitor_command("prune networks", command).await
}
