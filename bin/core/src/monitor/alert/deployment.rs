use std::collections::HashMap;

use monitor_types::entities::{
  alert::{Alert, AlertData, AlertDataVariant},
  deployment::Deployment,
  server::stats::SeverityLevel,
  update::ResourceTarget,
};

use crate::{helpers::resource::StateResource, state::State};

impl State {
  pub async fn alert_deployments(
    &self,
    ts: i64,
    server_names: HashMap<String, String>,
  ) {
    let mut alerts = Vec::<Alert>::new();
    for v in self.deployment_status_cache.get_list().await {
      if v.prev.is_none() {
        continue;
      }
      let prev = v.prev.as_ref().unwrap().to_owned();
      if v.curr.state != prev {
        // send alert
        let d = <State as StateResource<Deployment>>::get_resource(
          self, &v.curr.id,
        )
        .await;
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
    self.send_alerts(&alerts).await;
    let res = self.db.alerts.insert_many(alerts, None).await;
    if let Err(e) = res {
      error!(
        "failed to record deployment status alerts to db | {e:#?}"
      );
    }
  }
}
