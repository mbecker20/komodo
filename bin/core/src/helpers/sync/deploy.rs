use std::time::Duration;

use formatting::{bold, colored, muted, Color};
use futures::future::join_all;
use monitor_client::{
  api::execute::{Deploy, DeployStack},
  entities::{
    update::{Log, ResourceTarget},
    user::sync_user,
  },
};
use resolver_api::Resolve;

use crate::{
  api::execute::ExecuteRequest,
  helpers::update::init_execution_update, state::State,
};

/// (Deployment or Stack, Dependencies).
/// They all come as names, not ids.
pub type ToDeploy = Vec<(ResourceTarget, Vec<ResourceTarget>)>;

pub async fn deploy(
  mut to_deploy: ToDeploy,
  logs: &mut Vec<Log>,
  has_error: &mut bool,
) {
  let mut log = format!(
    "{}: running executions to sync deployment / stack state",
    muted("INFO")
  );
  let mut round = 1;
  let user = sync_user();

  while !to_deploy.is_empty() {
    // Collect all waiting deployments without waiting dependencies.
    let good_to_deploy = to_deploy
      .iter()
      .filter(|(_, after)| {
        to_deploy.iter().all(|(name, _)| !after.contains(name))
      })
      .map(|(name, _)| name.clone())
      .collect::<Vec<_>>();

    // Deploy the ones ready for deployment
    let res =
      join_all(good_to_deploy.iter().map(|target| async move {
        let res = async {
          match &target {
            ResourceTarget::Deployment(name) => {
              let req = ExecuteRequest::Deploy(Deploy {
                deployment: name.to_string(),
                stop_signal: None,
                stop_time: None,
              });

              let update = init_execution_update(&req, user).await?;
              let ExecuteRequest::Deploy(req) = req else {
                unreachable!()
              };
              State.resolve(req, (user.to_owned(), update)).await
            }
            ResourceTarget::Stack(name) => {
              let req = ExecuteRequest::DeployStack(DeployStack {
                stack: name.to_string(),
                stop_time: None,
              });

              let update = init_execution_update(&req, user).await?;
              let ExecuteRequest::DeployStack(req) = req else {
                unreachable!()
              };
              State.resolve(req, (user.to_owned(), update)).await
            }
            _ => unreachable!(),
          }
        }
        .await;
        (target, res)
      }))
      .await;

    // Log results of deploy
    for (target, res) in res {
      let (resource, name) = target.extract_variant_id();
      if let Err(e) = res {
        *has_error = true;
        log.push_str(&format!(
          "\n{}: failed to deploy {resource} '{}' in round {} | {e:#}",
          colored("ERROR", Color::Red),
          bold(name),
          bold(round)
        ));
      } else {
        log.push_str(&format!(
          "\n{}: deployed {resource} '{}' in round {}",
          muted("INFO"),
          bold(name),
          bold(round)
        ));
      }
    }

    // Early exit if any deploy has errors
    if *has_error {
      log.push_str(&format!(
        "\n{}: exited in round {} {}",
        muted("INFO"),
        bold(round),
        colored("with errors", Color::Red)
      ));
      logs.push(Log::error("Sync Deployment / Stack State", log));
      return;
    }

    // Remove the deployed ones from 'to_deploy'
    to_deploy.retain(|(name, _)| !good_to_deploy.contains(name));

    // If there must be another round, these are dependent on the first round.
    // Sleep for 1s to allow for first round to startup
    if !to_deploy.is_empty() {
      // Increment the round
      round += 1;
      tokio::time::sleep(Duration::from_secs(1)).await;
    }
  }

  log.push_str(&format!(
    "\n{}: finished after {} round{}",
    muted("INFO"),
    bold(round),
    (round > 1).then_some("s").unwrap_or_default()
  ));

  logs.push(Log::simple("Sync Deployment State", log));
}
