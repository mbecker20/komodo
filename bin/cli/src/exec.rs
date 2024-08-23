use std::time::Duration;

use colored::Colorize;
use monitor_client::api::execute::Execution;

use crate::{
  helpers::wait_for_enter,
  state::{cli_args, monitor_client},
};

pub async fn run(execution: Execution) -> anyhow::Result<()> {
  if matches!(execution, Execution::None(_)) {
    println!("Got 'none' execution. Doing nothing...");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("Finished doing nothing. Exiting...");
    std::process::exit(0);
  }

  println!("\n{}: Execution", "Mode".dimmed());
  match &execution {
    Execution::None(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunProcedure(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CancelBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::Deploy(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DestroyDeployment(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CloneRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PullRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BuildRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CancelRepoBuild(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DestroyContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneNetworks(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneImages(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneVolumes(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneSystem(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunSync(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeployStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RestartStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PauseStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::UnpauseStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DestroyStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::Sleep(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
  }

  if !cli_args().yes {
    wait_for_enter("run execution")?;
  }

  info!("Running Execution...");

  let res = match execution {
    Execution::RunProcedure(request) => {
      monitor_client().execute(request).await
    }
    Execution::RunBuild(request) => {
      monitor_client().execute(request).await
    }
    Execution::CancelBuild(request) => {
      monitor_client().execute(request).await
    }
    Execution::Deploy(request) => {
      monitor_client().execute(request).await
    }
    Execution::StartDeployment(request) => {
      monitor_client().execute(request).await
    }
    Execution::RestartDeployment(request) => {
      monitor_client().execute(request).await
    }
    Execution::PauseDeployment(request) => {
      monitor_client().execute(request).await
    }
    Execution::UnpauseDeployment(request) => {
      monitor_client().execute(request).await
    }
    Execution::StopDeployment(request) => {
      monitor_client().execute(request).await
    }
    Execution::DestroyDeployment(request) => {
      monitor_client().execute(request).await
    }
    Execution::CloneRepo(request) => {
      monitor_client().execute(request).await
    }
    Execution::PullRepo(request) => {
      monitor_client().execute(request).await
    }
    Execution::BuildRepo(request) => {
      monitor_client().execute(request).await
    }
    Execution::CancelRepoBuild(request) => {
      monitor_client().execute(request).await
    }
    Execution::StartContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::RestartContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::PauseContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::UnpauseContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::StopContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::DestroyContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::StartAllContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::RestartAllContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::PauseAllContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::UnpauseAllContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::StopAllContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneNetworks(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneImages(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneVolumes(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneSystem(request) => {
      monitor_client().execute(request).await
    }
    Execution::RunSync(request) => {
      monitor_client().execute(request).await
    }
    Execution::DeployStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::StartStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::RestartStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::PauseStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::UnpauseStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::StopStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::DestroyStack(request) => {
      monitor_client().execute(request).await
    }
    Execution::Sleep(request) => {
      let duration =
        Duration::from_millis(request.duration_ms as u64);
      tokio::time::sleep(duration).await;
      println!("Finished sleeping!");
      std::process::exit(0)
    }
    Execution::None(_) => unreachable!(),
  };

  match res {
    Ok(update) => println!("\n{}: {update:#?}", "SUCCESS".green()),
    Err(e) => println!("{}\n\n{e:#?}", "ERROR".red()),
  }

  Ok(())
}
