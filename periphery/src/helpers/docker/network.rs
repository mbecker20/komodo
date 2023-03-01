use types::Log;

use crate::helpers::run_monitor_command;

pub async fn create_network(name: &str, driver: Option<String>) -> Log {
    let driver = match driver {
        Some(driver) => format!(" -d {driver}"),
        None => String::new(),
    };
    let command = format!("docker network create{driver} {name}");
    run_monitor_command("create network", command).await
}

pub async fn delete_network(name: &str) -> Log {
    let command = format!("docker network rm {name}");
    run_monitor_command("delete network", command).await
}

pub async fn prune_networks() -> Log {
    let command = format!("docker network prune -f");
    run_monitor_command("prune networks", command).await
}
