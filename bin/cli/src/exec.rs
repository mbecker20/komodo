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
    Execution::Deploy(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StartContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::StopAllContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RemoveContainer(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::CloneRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PullRepo(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneNetworks(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneImages(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::PruneContainers(data) => {
      println!("{}: {data:?}", "Data".dimmed())
    }
    Execution::RunSync(data) => {
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
    Execution::Deploy(request) => {
      monitor_client().execute(request).await
    }
    Execution::StartContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::StopContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::StopAllContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::RemoveContainer(request) => {
      monitor_client().execute(request).await
    }
    Execution::CloneRepo(request) => {
      monitor_client().execute(request).await
    }
    Execution::PullRepo(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneNetworks(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneImages(request) => {
      monitor_client().execute(request).await
    }
    Execution::PruneContainers(request) => {
      monitor_client().execute(request).await
    }
    Execution::RunSync(request) => {
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
