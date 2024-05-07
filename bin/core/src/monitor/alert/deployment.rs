use std::collections::HashMap;

use monitor_client::entities::{
  alert::{Alert, AlertData, AlertDataVariant},
  deployment::Deployment,
  server::stats::SeverityLevel,
  update::ResourceTarget,
};

use crate::{
  helpers::alert::send_alerts, monitor::deployment_status_cache,
  resource, state::db_client,
};

#[instrument(level = "debug")]
pub async fn alert_deployments(
  ts: i64,
  server_names: HashMap<String, String>,
) {
  let mut alerts = Vec::<Alert>::new();
  for v in deployment_status_cache().get_list().await {
    if v.prev.is_none() {
      continue;
    }
    let prev = v.prev.as_ref().unwrap().to_owned();
    if v.curr.state != prev {
      // send alert
      let d = resource::get::<Deployment>(&v.curr.id).await;
      if let Err(e) = d {
        error!("failed to get deployment from db | {e:#?}");
        continue;
      }
      let d = d.unwrap();
      let target: ResourceTarget = (&d).into();
      let data = AlertData::ContainerStateChange {
        id: v.curr.id.clone(),
        name: d.name,
        server_name: server_names
          .get(&d.config.server_id)
          .cloned()
          .unwrap_or(String::from("unknown")),
        server_id: d.config.server_id,
        from: prev,
        to: v.curr.state,
      };
      let alert = Alert {
        id: Default::default(),
        level: SeverityLevel::Warning,
        variant: AlertDataVariant::ContainerStateChange,
        resolved: true,
        resolved_ts: ts.into(),
        target,
        data,
        ts,
      };
      if d.config.send_alerts {
        alerts.push(alert);
      }
    }
  }
  if alerts.is_empty() {
    return;
  }
  send_alerts(&alerts).await;
  let res = db_client().await.alerts.insert_many(alerts, None).await;
  if let Err(e) = res {
    error!("failed to record deployment status alerts to db | {e:#}");
  }
}
