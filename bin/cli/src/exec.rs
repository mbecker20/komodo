use std::time::Duration;

use colored::Colorize;
use komodo_client::api::execute::Execution;

use crate::{
  helpers::wait_for_enter,
  state::{cli_args, komodo_client},
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
    Execution::DeleteNetwork(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneNetworks(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeleteImage(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneImages(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeleteVolume(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneVolumes(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneDockerBuilders(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneBuildx(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneSystem(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunSync(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CommitSync(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeployStack(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::DeployStackIfChanged(data) => {
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
      komodo_client().execute(request).await
    }
    Execution::RunBuild(request) => {
      komodo_client().execute(request).await
    }
    Execution::CancelBuild(request) => {
      komodo_client().execute(request).await
    }
    Execution::Deploy(request) => {
      komodo_client().execute(request).await
    }
    Execution::StartDeployment(request) => {
      komodo_client().execute(request).await
    }
    Execution::RestartDeployment(request) => {
      komodo_client().execute(request).await
    }
    Execution::PauseDeployment(request) => {
      komodo_client().execute(request).await
    }
    Execution::UnpauseDeployment(request) => {
      komodo_client().execute(request).await
    }
    Execution::StopDeployment(request) => {
      komodo_client().execute(request).await
    }
    Execution::DestroyDeployment(request) => {
      komodo_client().execute(request).await
    }
    Execution::CloneRepo(request) => {
      komodo_client().execute(request).await
    }
    Execution::PullRepo(request) => {
      komodo_client().execute(request).await
    }
    Execution::BuildRepo(request) => {
      komodo_client().execute(request).await
    }
    Execution::CancelRepoBuild(request) => {
      komodo_client().execute(request).await
    }
    Execution::StartContainer(request) => {
      komodo_client().execute(request).await
    }
    Execution::RestartContainer(request) => {
      komodo_client().execute(request).await
    }
    Execution::PauseContainer(request) => {
      komodo_client().execute(request).await
    }
    Execution::UnpauseContainer(request) => {
      komodo_client().execute(request).await
    }
    Execution::StopContainer(request) => {
      komodo_client().execute(request).await
    }
    Execution::DestroyContainer(request) => {
      komodo_client().execute(request).await
    }
    Execution::StartAllContainers(request) => {
      komodo_client().execute(request).await
    }
    Execution::RestartAllContainers(request) => {
      komodo_client().execute(request).await
    }
    Execution::PauseAllContainers(request) => {
      komodo_client().execute(request).await
    }
    Execution::UnpauseAllContainers(request) => {
      komodo_client().execute(request).await
    }
    Execution::StopAllContainers(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneContainers(request) => {
      komodo_client().execute(request).await
    }
    Execution::DeleteNetwork(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneNetworks(request) => {
      komodo_client().execute(request).await
    }
    Execution::DeleteImage(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneImages(request) => {
      komodo_client().execute(request).await
    }
    Execution::DeleteVolume(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneVolumes(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneDockerBuilders(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneBuildx(request) => {
      komodo_client().execute(request).await
    }
    Execution::PruneSystem(request) => {
      komodo_client().execute(request).await
    }
    Execution::RunSync(request) => {
      komodo_client().execute(request).await
    }
    Execution::CommitSync(request) => {
      komodo_client().write(request).await
    }
    Execution::DeployStack(request) => {
      komodo_client().execute(request).await
    }
    Execution::DeployStackIfChanged(request) => {
      komodo_client().execute(request).await
    }
    Execution::StartStack(request) => {
      komodo_client().execute(request).await
    }
    Execution::RestartStack(request) => {
      komodo_client().execute(request).await
    }
    Execution::PauseStack(request) => {
      komodo_client().execute(request).await
    }
    Execution::UnpauseStack(request) => {
      komodo_client().execute(request).await
    }
    Execution::StopStack(request) => {
      komodo_client().execute(request).await
    }
    Execution::DestroyStack(request) => {
      komodo_client().execute(request).await
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
