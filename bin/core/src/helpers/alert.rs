use anyhow::{anyhow, Context};
use derive_variants::ExtractVariant;
use futures::future::join_all;
use monitor_client::entities::{
  alert::{Alert, AlertData},
  alerter::*,
  deployment::DeploymentState,
  server::stats::SeverityLevel,
};
use mungos::{find::find_collect, mongodb::bson::doc};
use slack::types::Block;

use crate::state::db_client;

#[instrument]
pub async fn send_alerts(alerts: &[Alert]) {
  if alerts.is_empty() {
    return;
  }

  let alerters = match find_collect(
    &db_client().await.alerters,
    doc! { "config.enabled": true },
    None,
  )
  .await
  {
    Ok(alerters) => alerters,
    Err(e) => {
      error!(
        "ERROR sending alerts | failed to get alerters from db | {e:#}"
      );
      return;
    }
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

    // Don't send if resource target not configured on the alerter
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
    AlertData::ServerUnreachable { name, region, .. } => {
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
          let blocks = vec![
            Block::header(level),
            Block::section(format!(
              "*{name}*{region} is *unreachable* ❌"
            )),
          ];
          (text, blocks.into())
        }
        _ => unreachable!(),
      }
    }
    AlertData::ServerCpu {
      name,
      region,
      percentage,
      ..
    } => {
      let region = fmt_region(region);
      let text = format!("{level} | *{name}*{region} cpu usage at *{percentage:.1}%* 📈 🚨");
      let blocks = vec![
        Block::header(format!("{level} 🚨")),
        Block::section(format!(
          "*{name}*{region} cpu usage at *{percentage:.1}%* 📈 🚨"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::ServerMem {
      name,
      region,
      used_gb,
      total_gb,
      ..
    } => {
      let region = fmt_region(region);
      let percentage = 100.0 * used_gb / total_gb;
      let text =
                format!("{level} | *{name}*{region} memory usage at *{percentage:.1}%* 💾 🚨");
      let blocks = vec![
        Block::header(level),
        Block::section(format!(
          "*{name}*{region} memory usage at *{percentage:.1}%* 💾 🚨"
        )),
        Block::section(format!(
          "using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::ServerDisk {
      name,
      region,
      path,
      used_gb,
      total_gb,
      ..
    } => {
      let region = fmt_region(region);
      let percentage = 100.0 * used_gb / total_gb;
      let text = format!("{level} | *{name}*{region} disk usage at *{percentage:.1}%* | mount point: *{path:?}* 💿 🚨");
      let blocks = vec![
        Block::header(level),
        Block::section(format!(
          "*{name}*{region} disk usage at *{percentage:.1}%* 💿 🚨"
        )),
        Block::section(format!(
          "mount point: {path:?} | using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::ContainerStateChange {
      name,
      server_name,
      from,
      to,
      ..
    } => {
      let to = fmt_docker_container_state(to);
      let text = format!("📦 container *{name}* is now {to}");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "server: {server_name}\nprevious: {from}"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::AwsBuilderTerminationFailed {
      instance_id,
      message,
    } => {
      let text = format!(
        "{level} | Failed to terminated AWS builder instance"
      );
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "instance id: **{instance_id}**\n{message}"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::ResourceSyncPendingUpdates { id, name } => {
      let text =
        format!("{level} | There are pending resource sync updates");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "sync id: **{id}**\nsync name: **{name}**"
        )),
      ];
      (text, blocks.into())
    }
    AlertData::BuildFailed { id, name, version } => {
      let text = format!("{level} | Build {name} has failed");
      let blocks = vec![
        Block::header(text.clone()),
        Block::section(format!(
          "build id: **{id}**\nbuild name: **{name}**\nversion: v{version}"
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

fn fmt_level(level: SeverityLevel) -> &'static str {
  match level {
    SeverityLevel::Critical => "CRITICAL 🚨",
    SeverityLevel::Warning => "WARNING 🚨",
    SeverityLevel::Ok => "OK ✅",
  }
}
