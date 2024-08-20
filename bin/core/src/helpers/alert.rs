use anyhow::{anyhow, Context};
use derive_variants::ExtractVariant;
use futures::future::join_all;
use monitor_client::entities::{
  alert::{Alert, AlertData, SeverityLevel},
  alerter::*,
  deployment::DeploymentState,
  stack::StackState,
  update::ResourceTargetVariant,
};
use mungos::{find::find_collect, mongodb::bson::doc};
use slack::types::Block;

use crate::{config::core_config, state::db_client};

#[instrument]
pub async fn send_alerts(alerts: &[Alert]) {
  if alerts.is_empty() {
    return;
  }

  let Ok(alerters) = find_collect(
    &db_client().await.alerters,
    doc! { "config.enabled": true },
    None,
  )
  .await
  .inspect_err(|e| {
    error!(
      "ERROR sending alerts | failed to get alerters from db | {e:#}"
    )
  }) else {
    return;
  };

  let handles =
    alerts.iter().map(|alert| send_alert(&alerters, alert));

  join_all(handles).await;
}

#[instrument(level = "debug")]
async fn send_alert(alerters: &[Alerter], alert: &Alert) {
  if alerters.is_empty() {
    return;
  }

  let alert_type = alert.data.extract_variant();

  let handles = alerters.iter().map(|alerter| async {
    // Don't send if not enabled
    if !alerter.config.enabled {
      return Ok(());
    }

    // Don't send if alert type not configured on the alerter
    if !alerter.config.alert_types.is_empty()
      && !alerter.config.alert_types.contains(&alert_type)
    {
      return Ok(());
    }

    // Don't send if resource is in the blacklist
    if alerter.config.except_resources.contains(&alert.target) {
      return Ok(());
    }

    // Don't send if whitelist configured and target is not included
    if !alerter.config.resources.is_empty()
      && !alerter.config.resources.contains(&alert.target)
    {
      return Ok(());
    }

    match &alerter.config.endpoint {
      AlerterEndpoint::Slack(SlackAlerterEndpoint { url }) => {
        send_slack_alert(url, alert).await.with_context(|| {
          format!(
            "failed to send alert to slack alerter {}",
            alerter.name
          )
        })
      }
      AlerterEndpoint::Custom(CustomAlerterEndpoint { url }) => {
        send_custom_alert(url, alert).await.with_context(|| {
          format!(
            "failed to send alert to custom alerter {}",
            alerter.name
          )
        })
      }
    }
  });

  join_all(handles)
    .await
    .into_iter()
    .filter_map(|res| res.err())
    .for_each(|e| error!("{e:#}"));
}

#[instrument(level = "debug")]
async fn send_custom_alert(
  url: &str,
  alert: &Alert,
) -> anyhow::Result<()> {
  let res = reqwest::Client::new()
    .post(url)
    .json(alert)
    .send()
    .await
    .context("failed at post request to alerter")?;
  let status = res.status();
  if !status.is_success() {
    let text = res
      .text()
      .await
      .context("failed to get response text on alerter response")?;
    return Err(anyhow!(
      "post to alerter failed | {status} | {text}"
    ));
  }
  Ok(())
}

#[instrument(level = "debug")]
async fn send_slack_alert(
  url: &str,
  alert: &Alert,
) -> anyhow::Result<()> {
  let level = fmt_level(alert.level);
  let (text, blocks): (_, Option<_>) = match &alert.data {
    AlertData::ServerUnreachable {
      id,
      name,
      region,
      err,
    } => {
      let region = fmt_region(region);
      match alert.level {
        SeverityLevel::Ok => {
          let text =
            format!("{level} | *{name}*{region} is now *reachable*");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} is now *reachable*"
            )),
          ];
          (text, blocks.into())
        }
        SeverityLevel::Critical => {
          let text =
            format!("{level} | *{name}*{region} is *unreachable* ❌");
          let err = err
            .as_ref()
            .map(|e| format!("\nerror: {e:#?}"))
            .unwrap_or_default();
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} is *unreachable* ❌{err}"
            )),
            Block::section(resource_link(
              ResourceTargetVariant::Server,
              id,
            )),
          ];
          (text, blocks.into())
        }
        _ => unreachable!(),
      }
    }
    AlertData::ServerCpu {
      id,
      name,
      region,
      percentage,
    } => {
      let region = fmt_region(region);
      match alert.level {
        SeverityLevel::Ok => {
          let text = format!("{level} | *{name}*{region} cpu usage at *{percentage:.1}%*");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} cpu usage at *{percentage:.1}%*"
            )),
            Block::section(resource_link(
              ResourceTargetVariant::Server,
              id,
            )),
          ];
          (text, blocks.into())
        }
        _ => {
          let text = format!("{level} | *{name}*{region} cpu usage at *{percentage:.1}%* 📈");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} cpu usage at *{percentage:.1}%* 📈"
            )),
            Block::section(resource_link(
              ResourceTargetVariant::Server,
              id,
            )),
          ];
          (text, blocks.into())
        }
      }
    }
    AlertData::ServerMem {
      id,
      name,
      region,
      used_gb,
      total_gb,
    } => {
      let region = fmt_region(region);
      let percentage = 100.0 * used_gb / total_gb;
      match alert.level {
        SeverityLevel::Ok => {
          let text = format!("{level} | *{name}*{region} memory usage at *{percentage:.1}%* 💾");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} memory usage at *{percentage:.1}%* 💾"
            )),
            Block::section(format!(
              "using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
            )),
            Block::section(resource_link(
              ResourceTargetVariant::Server,
              id,
            )),
          ];
          (text, blocks.into())
        }
        _ => {
          let text = format!("{level} | *{name}*{region} memory usage at *{percentage:.1}%* 💾");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} memory usage at *{percentage:.1}%* 💾"
            )),
            Block::section(format!(
              "using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
            )),
            Block::section(resource_link(
              ResourceTargetVariant::Server,
              id,
            )),
          ];
          (text, blocks.into())
        }
      }
    }
    AlertData::ServerDisk {
      id,
      name,
      region,
      path,
      used_gb,
      total_gb,
    } => {
      let region = fmt_region(region);
      let percentage = 100.0 * used_gb / total_gb;
      match alert.level {
        SeverityLevel::Ok => {
          let text = format!("{level} | *{name}*{region} disk usage at *{percentage:.1}%* | mount point: *{path:?}* 💿");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} disk usage at *{percentage:.1}%* 💿"
            )),
            Block::section(format!(
              "mount point: {path:?} | using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
            )),
            Block::section(resource_link(ResourceTargetVariant::Server, id)),
          ];
          (text, blocks.into())
        }
        _ => {
          let text = format!("{level} | *{name}*{region} disk usage at *{percentage:.1}%* | mount point: *{path:?}* 💿");
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} disk usage at *{percentage:.1}%* 💿"
            )),
            Block::section(format!(
              "mount point: {path:?} | using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
            )),
            Block::section(resource_link(ResourceTargetVariant::Server, id)),
          ];
          (text, blocks.into())
        }
      }
    }
    AlertData::ContainerStateChange {
      name,
      server_name,
      from,
      to,
      id,
      ..
    } => {
      let to = fmt_docker_container_state(to);
      let text = format!("📦 Container *{name}* is now {to}");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "server: {server_name}\nprevious: {from}",
        )),
        Block::section(resource_link(
          ResourceTargetVariant::Deployment,
          id,
        )),
      ];
      (text, blocks.into())
    }
    AlertData::StackStateChange {
      name,
      server_name,
      from,
      to,
      id,
      ..
    } => {
      let to = fmt_stack_state(to);
      let text = format!("🥞 Stack *{name}* is now {to}");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "server: {server_name}\nprevious: {from}",
        )),
        Block::section(resource_link(
          ResourceTargetVariant::Stack,
          id,
        )),
      ];
      (text, blocks.into())
    }
    AlertData::AwsBuilderTerminationFailed {
      instance_id,
      message,
    } => {
      let text = format!(
        "{level} | Failed to terminated AWS builder instance "
      );
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "instance id: *{instance_id}*\n{message}"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::ResourceSyncPendingUpdates { id, name } => {
      let text =
        format!("{level} | Pending resource sync updates on {name}");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "sync id: *{id}*\nsync name: *{name}*",
        )),
        Block::section(resource_link(
          ResourceTargetVariant::ResourceSync,
          id,
        )),
      ];
      (text, blocks.into())
    }
    AlertData::BuildFailed { id, name, version } => {
      let text = format!("{level} | Build {name} has failed");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "build id: *{id}*\nbuild name: *{name}*\nversion: v{version}",
        )),
        Block::section(resource_link(ResourceTargetVariant::Build, id))
      ];
      (text, blocks.into())
    }
    AlertData::RepoBuildFailed { id, name } => {
      let text =
        format!("{level} | Repo build for {name} has failed");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "repo id: *{id}*\nrepo name: *{name}*",
        )),
        Block::section(resource_link(
          ResourceTargetVariant::Repo,
          id,
        )),
      ];
      (text, blocks.into())
    }
    AlertData::None {} => Default::default(),
  };
  if !text.is_empty() {
    let slack = slack::Client::new(url);
    slack.send_message(text, blocks).await?;
  }
  Ok(())
}

fn fmt_region(region: &Option<String>) -> String {
  match region {
    Some(region) => format!(" ({region})"),
    None => String::new(),
  }
}

fn fmt_docker_container_state(state: &DeploymentState) -> String {
  match state {
    DeploymentState::Running => String::from("Running ▶️"),
    DeploymentState::Exited => String::from("Exited 🛑"),
    DeploymentState::Restarting => String::from("Restarting 🔄"),
    DeploymentState::NotDeployed => String::from("Not Deployed"),
    _ => state.to_string(),
  }
}

fn fmt_stack_state(state: &StackState) -> String {
  match state {
    StackState::Running => String::from("Running ▶️"),
    StackState::Stopped => String::from("Stopped 🛑"),
    StackState::Restarting => String::from("Restarting 🔄"),
    StackState::Down => String::from("Down ⬇️"),
    _ => state.to_string(),
  }
}

fn fmt_level(level: SeverityLevel) -> &'static str {
  match level {
    SeverityLevel::Critical => "CRITICAL 🚨",
    SeverityLevel::Warning => "WARNING ‼️",
    SeverityLevel::Ok => "OK ✅",
  }
}

fn resource_link(
  resource_type: ResourceTargetVariant,
  id: &str,
) -> String {
  let path = match resource_type {
    ResourceTargetVariant::System => unreachable!(),
    ResourceTargetVariant::Build => format!("/builds/{id}"),
    ResourceTargetVariant::Builder => {
      format!("/builders/{id}")
    }
    ResourceTargetVariant::Deployment => {
      format!("/deployments/{id}")
    }
    ResourceTargetVariant::Stack => {
      format!("/stacks/{id}")
    }
    ResourceTargetVariant::Server => {
      format!("/servers/{id}")
    }
    ResourceTargetVariant::Repo => format!("/repos/{id}"),
    ResourceTargetVariant::Alerter => {
      format!("/alerters/{id}")
    }
    ResourceTargetVariant::Procedure => {
      format!("/procedures/{id}")
    }
    ResourceTargetVariant::ServerTemplate => {
      format!("/server-templates/{id}")
    }
    ResourceTargetVariant::ResourceSync => {
      format!("/resource-syncs/{id}")
    }
  };

  format!("{}{path}", core_config().host)
}
