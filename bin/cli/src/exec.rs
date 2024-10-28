use std::time::Duration;

use colored::Colorize;
use komodo_client::{
  api::execute::{BatchExecutionResult, Execution},
  entities::update::Update,
};

use crate::{
  helpers::wait_for_enter,
  state::{cli_args, komodo_client},
};

pub enum ExecutionResult {
  Single(Update),
  Batch(BatchExecutionResult),
}

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
    Execution::RunAction(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::BatchRunAction(data) => {
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
    Execution::RunAction(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::BatchRunAction(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Batch(u)),
    Execution::RunProcedure(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::RunBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::CancelBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::Deploy(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StartDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::RestartDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PauseDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::UnpauseDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StopDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DestroyDeployment(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::CloneRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PullRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::BuildRepo(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::CancelRepoBuild(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StartContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::RestartContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PauseContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::UnpauseContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StopContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DestroyContainer(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StartAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::RestartAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PauseAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::UnpauseAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StopAllContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneContainers(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DeleteNetwork(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneNetworks(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DeleteImage(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneImages(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DeleteVolume(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneVolumes(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneDockerBuilders(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneBuildx(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PruneSystem(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::RunSync(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::CommitSync(request) => komodo_client()
      .write(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DeployStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DeployStackIfChanged(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StartStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::RestartStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::PauseStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::UnpauseStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::StopStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
    Execution::DestroyStack(request) => komodo_client()
      .execute(request)
      .await
      .map(|u| ExecutionResult::Single(u)),
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
    Ok(ExecutionResult::Single(update)) => {
      println!("\n{}: {update:#?}", "SUCCESS".green())
    }
    Ok(ExecutionResult::Batch(update)) => {
      println!("\n{}: {update:#?}", "SUCCESS".green())
    }
    Err(e) => println!("{}\n\n{e:#?}", "ERROR".red()),
  }

  Ok(())
}
