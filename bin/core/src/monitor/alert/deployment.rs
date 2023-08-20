use monitor_types::entities::{
    alert::{Alert, AlertData},
    deployment::Deployment, server::stats::SeverityLevel,
};

use crate::{helpers::resource::StateResource, state::State};

impl State {
    pub async fn alert_deployments(&self) {
        let mut alerts = Vec::<Alert>::new();
        for v in self.deployment_status_cache.get_list().await {
            if v.prev.is_none() {
                continue;
            }
            let prev = v.prev.as_ref().unwrap().to_owned();
            if v.curr.state != prev {
                // send alert
                let d = <State as StateResource<Deployment>>::get_resource(self, &v.curr.id).await;
                if let Err(e) = d {
                    error!("failed to get deployment from db | {e:#?}");
                    continue;
                }
                let d = d.unwrap();
                let data = AlertData::ContainerStateChange {
                    id: v.curr.id.clone(),
                    name: d.name,
                    server: d.config.server_id,
                    from: prev,
                    to: v.curr.state,
                };
                let alert = Alert {
                    level: SeverityLevel::Warning,
                    data,
                    ..Default::default()
                };
                alerts.push(alert);
            }
        }
        self.send_alerts(&alerts).await;
    }
}
