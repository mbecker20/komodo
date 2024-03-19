use std::{
  cmp::Ordering, collections::HashMap, path::PathBuf, str::FromStr,
};

use anyhow::Context;
use mongo_indexed::Indexed;
use monitor_client::entities::{
  alert::{Alert, AlertData, AlertDataVariant},
  monitor_timestamp, optional_string,
  server::{stats::SeverityLevel, ServerListItem, ServerStatus},
  update::ResourceTarget,
};
use mungos::{
  bulk_update::{self, BulkUpdate},
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};

use crate::{
  db_client, helpers::cache::server_status_cache, state::State,
};

type SendAlerts = bool;
type OpenAlertMap<T = AlertDataVariant> =
  HashMap<ResourceTarget, HashMap<T, Alert>>;
type OpenDiskAlertMap = OpenAlertMap<PathBuf>;
type OpenTempAlertMap = OpenAlertMap<String>;

impl State {
  pub async fn alert_servers(
    &self,
    ts: i64,
    mut servers: HashMap<String, ServerListItem>,
  ) {
    let server_statuses = server_status_cache().get_list().await;

    let alerts = self.get_open_alerts().await;

    if let Err(e) = alerts {
      error!("{e:#?}");
      return;
    }

    let (alerts, disk_alerts, temp_alerts) = alerts.unwrap();

    let mut alerts_to_open = Vec::<(Alert, SendAlerts)>::new();
    let mut alerts_to_update = Vec::<(Alert, SendAlerts)>::new();
    let mut alert_ids_to_close = Vec::<(String, SendAlerts)>::new();

    for server_status in server_statuses {
      let server = servers.remove(&server_status.id);
      if server.is_none() {
        continue;
      }
      let server = server.unwrap();
      let server_alerts =
        alerts.get(&ResourceTarget::Server(server_status.id.clone()));

      // ===================
      // SERVER HEALTH
      // ===================
      let health_alert = server_alerts.as_ref().and_then(|alerts| {
        alerts.get(&AlertDataVariant::ServerUnreachable)
      });
      match (server_status.status, health_alert) {
        (ServerStatus::NotOk, None) => {
          // open unreachable alert
          let alert = Alert {
            id: Default::default(),
            ts,
            resolved: false,
            resolved_ts: None,
            level: SeverityLevel::Critical,
            target: ResourceTarget::Server(server_status.id.clone()),
            variant: AlertDataVariant::ServerUnreachable,
            data: AlertData::ServerUnreachable {
              id: server_status.id.clone(),
              name: server.name.clone(),
              region: optional_string(&server.info.region),
            },
          };
          alerts_to_open
            .push((alert, server.info.send_unreachable_alerts))
        }
        (
          ServerStatus::Ok | ServerStatus::Disabled,
          Some(health_alert),
        ) => alert_ids_to_close.push((
          health_alert.id.clone(),
          server.info.send_unreachable_alerts,
        )),
        _ => {}
      }

      if server_status.health.is_none() {
        continue;
      }

      let health = server_status.health.as_ref().unwrap();

      // ===================
      // SERVER CPU
      // ===================
      let cpu_alert = server_alerts
        .as_ref()
        .and_then(|alerts| alerts.get(&AlertDataVariant::ServerCpu))
        .cloned();
      match (health.cpu, cpu_alert) {
        (SeverityLevel::Warning | SeverityLevel::Critical, None) => {
          // open alert
          let alert = Alert {
            id: Default::default(),
            ts,
            resolved: false,
            resolved_ts: None,
            level: health.cpu,
            target: ResourceTarget::Server(server_status.id.clone()),
            variant: AlertDataVariant::ServerCpu,
            data: AlertData::ServerCpu {
              id: server_status.id.clone(),
              name: server.name.clone(),
              region: optional_string(&server.info.region),
              percentage: server_status
                .stats
                .as_ref()
                .map(|s| s.basic.cpu_perc as f64)
                .unwrap_or(0.0),
              top_procs: server_status
                .stats
                .as_ref()
                .map(|s| {
                  let mut procs = s.processes.clone();
                  procs.sort_by(|a, b| {
                    if a.cpu_perc < b.cpu_perc {
                      Ordering::Less
                    } else {
                      Ordering::Greater
                    }
                  });
                  procs.into_iter().take(3).collect()
                })
                .unwrap_or_default(),
            },
          };
          alerts_to_open.push((alert, server.info.send_cpu_alerts));
        }
        (
          SeverityLevel::Warning | SeverityLevel::Critical,
          Some(mut alert),
        ) => {
          // modify alert level
          if alert.level != health.cpu {
            alert.level = health.cpu;
            alert.data = AlertData::ServerCpu {
              id: server_status.id.clone(),
              name: server.name.clone(),
              region: optional_string(&server.info.region),
              percentage: server_status
                .stats
                .as_ref()
                .map(|s| s.basic.cpu_perc as f64)
                .unwrap_or(0.0),
              top_procs: server_status
                .stats
                .as_ref()
                .map(|s| {
                  let mut procs = s.processes.clone();
                  procs.sort_by(|a, b| {
                    if a.cpu_perc < b.cpu_perc {
                      Ordering::Less
                    } else {
                      Ordering::Greater
                    }
                  });
                  procs.into_iter().take(3).collect()
                })
                .unwrap_or_default(),
            };
            alerts_to_update
              .push((alert, server.info.send_cpu_alerts));
          }
        }
        (SeverityLevel::Ok, Some(alert)) => alert_ids_to_close
          .push((alert.id.clone(), server.info.send_cpu_alerts)),
        _ => {}
      }

      // ===================
      // SERVER MEM
      // ===================
      let mem_alert = server_alerts
        .as_ref()
        .and_then(|alerts| alerts.get(&AlertDataVariant::ServerMem))
        .cloned();
      match (health.mem, mem_alert) {
        (SeverityLevel::Warning | SeverityLevel::Critical, None) => {
          // open alert
          let alert = Alert {
            id: Default::default(),
            ts,
            resolved: false,
            resolved_ts: None,
            level: health.cpu,
            target: ResourceTarget::Server(server_status.id.clone()),
            variant: AlertDataVariant::ServerMem,
            data: AlertData::ServerMem {
              id: server_status.id.clone(),
              name: server.name.clone(),
              region: optional_string(&server.info.region),
              total_gb: server_status
                .stats
                .as_ref()
                .map(|s| s.basic.mem_total_gb)
                .unwrap_or(0.0),
              used_gb: server_status
                .stats
                .as_ref()
                .map(|s| s.basic.mem_used_gb)
                .unwrap_or(0.0),
              top_procs: server_status
                .stats
                .as_ref()
                .map(|s| {
                  let mut procs = s.processes.clone();
                  procs.sort_by(|a, b| {
                    if a.mem_mb < b.mem_mb {
                      Ordering::Less
                    } else {
                      Ordering::Greater
                    }
                  });
                  procs.into_iter().take(3).collect()
                })
                .unwrap_or_default(),
            },
          };
          alerts_to_open.push((alert, server.info.send_mem_alerts));
        }
        (
          SeverityLevel::Warning | SeverityLevel::Critical,
          Some(mut alert),
        ) => {
          if alert.level != health.mem {
            alert.level = health.mem;
            alert.data = AlertData::ServerMem {
              id: server_status.id.clone(),
              name: server.name.clone(),
              region: optional_string(&server.info.region),
              total_gb: server_status
                .stats
                .as_ref()
                .map(|s| s.basic.mem_total_gb)
                .unwrap_or(0.0),
              used_gb: server_status
                .stats
                .as_ref()
                .map(|s| s.basic.mem_used_gb)
                .unwrap_or(0.0),
              top_procs: server_status
                .stats
                .as_ref()
                .map(|s| {
                  let mut procs = s.processes.clone();
                  procs.sort_by(|a, b| {
                    if a.mem_mb < b.mem_mb {
                      Ordering::Less
                    } else {
                      Ordering::Greater
                    }
                  });
                  procs.into_iter().take(3).collect()
                })
                .unwrap_or_default(),
            };
            alerts_to_update
              .push((alert, server.info.send_mem_alerts));
          }
        }
        (SeverityLevel::Ok, Some(alert)) => alert_ids_to_close
          .push((alert.id.clone(), server.info.send_mem_alerts)),
        _ => {}
      }

      // ===================
      // SERVER DISK
      // ===================

      let server_disk_alerts = disk_alerts
        .get(&ResourceTarget::Server(server_status.id.clone()));

      for (path, health) in &health.disks {
        let disk_alert = server_disk_alerts
          .as_ref()
          .and_then(|alerts| alerts.get(path))
          .cloned();
        match (*health, disk_alert) {
          (
            SeverityLevel::Warning | SeverityLevel::Critical,
            None,
          ) => {
            let disk = server_status.stats.as_ref().and_then(|s| {
              s.disk.disks.iter().find(|disk| disk.mount == *path)
            });
            let alert = Alert {
              id: Default::default(),
              ts,
              resolved: false,
              resolved_ts: None,
              level: *health,
              target: ResourceTarget::Server(
                server_status.id.clone(),
              ),
              variant: AlertDataVariant::ServerDisk,
              data: AlertData::ServerDisk {
                id: server_status.id.clone(),
                name: server.name.clone(),
                region: optional_string(&server.info.region),
                path: path.to_owned(),
                total_gb: disk
                  .map(|d| d.total_gb)
                  .unwrap_or_default(),
                used_gb: disk.map(|d| d.used_gb).unwrap_or_default(),
              },
            };
            alerts_to_open
              .push((alert, server.info.send_disk_alerts));
          }
          (
            SeverityLevel::Warning | SeverityLevel::Critical,
            Some(mut alert),
          ) => {
            if *health != alert.level {
              let disk = server_status.stats.as_ref().and_then(|s| {
                s.disk.disks.iter().find(|disk| disk.mount == *path)
              });
              alert.level = *health;
              alert.data = AlertData::ServerDisk {
                id: server_status.id.clone(),
                name: server.name.clone(),
                region: optional_string(&server.info.region),
                path: path.to_owned(),
                total_gb: disk
                  .map(|d| d.total_gb)
                  .unwrap_or_default(),
                used_gb: disk.map(|d| d.used_gb).unwrap_or_default(),
              };
              alerts_to_update
                .push((alert, server.info.send_disk_alerts));
            }
          }
          (SeverityLevel::Ok, Some(alert)) => alert_ids_to_close
            .push((alert.id.clone(), server.info.send_disk_alerts)),
          _ => {}
        }
      }

      // ===================
      // SERVER TEMP
      // ===================

      let server_temp_alerts = temp_alerts
        .get(&ResourceTarget::Server(server_status.id.clone()));

      for (component, health) in &health.temps {
        let temp_alert = server_temp_alerts
          .as_ref()
          .and_then(|alerts| alerts.get(component))
          .cloned();
        match (*health, temp_alert) {
          (
            SeverityLevel::Warning | SeverityLevel::Critical,
            None,
          ) => {
            let comp = server_status.stats.as_ref().and_then(|s| {
              s.components
                .iter()
                .find(|comp| comp.label == *component)
            });
            let alert = Alert {
              id: Default::default(),
              ts,
              resolved: false,
              resolved_ts: None,
              level: *health,
              target: ResourceTarget::Server(
                server_status.id.clone(),
              ),
              variant: AlertDataVariant::ServerTemp,
              data: AlertData::ServerTemp {
                id: server_status.id.clone(),
                name: server.name.clone(),
                region: optional_string(&server.info.region),
                component: component.to_owned(),
                temp: comp.map(|c| c.temp).unwrap_or_default() as f64,
                max: comp.map(|c| c.max).unwrap_or_default() as f64,
              },
            };
            alerts_to_open
              .push((alert, server.info.send_temp_alerts));
          }
          (
            SeverityLevel::Warning | SeverityLevel::Critical,
            Some(mut alert),
          ) => {
            if *health != alert.level {
              let comp = server_status.stats.as_ref().and_then(|s| {
                s.components
                  .iter()
                  .find(|comp| comp.label == *component)
              });
              alert.level = *health;
              alert.data = AlertData::ServerTemp {
                id: server_status.id.clone(),
                name: server.name.clone(),
                region: optional_string(&server.info.region),
                component: component.to_owned(),
                temp: comp.map(|c| c.temp).unwrap_or_default() as f64,
                max: comp.map(|c| c.max).unwrap_or_default() as f64,
              };
              alerts_to_update
                .push((alert, server.info.send_temp_alerts));
            }
          }
          (SeverityLevel::Ok, Some(alert)) => alert_ids_to_close
            .push((alert.id.clone(), server.info.send_temp_alerts)),
          _ => {}
        }
      }
    }

    tokio::join!(
      self.open_alerts(&alerts_to_open),
      self.update_alerts(&alerts_to_update),
      self.resolve_alerts(&alert_ids_to_close),
    );
  }

  async fn open_alerts(&self, alerts: &[(Alert, SendAlerts)]) {
    if alerts.is_empty() {
      return;
    }

    let open = || async {
      db_client()
        .await
        .alerts
        .insert_many(alerts.iter().map(|(alert, _)| alert), None)
        .await?;
      anyhow::Ok(())
    };

    let alerts = alerts
      .iter()
      .filter(|(_, send)| *send)
      .map(|(alert, _)| alert)
      .cloned()
      .collect::<Vec<_>>();

    let (res, _) = tokio::join!(open(), self.send_alerts(&alerts));

    if let Err(e) = res {
      error!("failed to create alerts on db | {e:#?}");
    }
  }

  async fn update_alerts(&self, alerts: &[(Alert, SendAlerts)]) {
    if alerts.is_empty() {
      return;
    }

    let open = || async {
      let updates = alerts.iter().map(|(alert, _)| {
        let update = BulkUpdate {
          query: doc! { "_id": ObjectId::from_str(&alert.id).context("failed to convert alert id to ObjectId")? },
          update: doc! { "$set": to_bson(alert).context("failed to convert alert to bson")? }
        };
        anyhow::Ok(update)
      })
      .filter_map(|update| match update {
        Ok(update) => Some(update),
        Err(e) => {
          warn!("failed to generate bulk update for alert | {e:#?}");
          None
        }
      }).collect::<Vec<_>>();

      bulk_update::bulk_update(
        &db_client().await.db,
        Alert::default_collection_name(),
        &updates,
        false,
      )
      .await
      .context("failed to bulk update alerts")?;

      anyhow::Ok(())
    };

    let alerts = alerts
      .iter()
      .filter(|(_, send)| *send)
      .map(|(alert, _)| alert)
      .cloned()
      .collect::<Vec<_>>();

    let (res, _) = tokio::join!(open(), self.send_alerts(&alerts));

    if let Err(e) = res {
      error!("failed to create alerts on db | {e:#?}");
    }
  }

  async fn resolve_alerts(&self, alert_ids: &[(String, SendAlerts)]) {
    if alert_ids.is_empty() {
      return;
    }

    let send_alerts_map =
      alert_ids.iter().cloned().collect::<HashMap<_, _>>();

    let close = || async {
      let alert_ids = alert_ids
        .iter()
        .map(|(id, _)| {
          ObjectId::from_str(id)
            .context("failed to convert alert id to ObjectId")
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
      db_client()
        .await
        .alerts
        .update_many(
          doc! { "_id": { "$in": &alert_ids } },
          doc! {
            "$set": {
              "resolved": true,
              "resolved_ts": monitor_timestamp()
            }
          },
          None,
        )
        .await
        .context("failed to resolve alerts on db")?;
      let mut closed = find_collect(
        &db_client().await.alerts,
        doc! { "_id": { "$in": &alert_ids } },
        None,
      )
      .await
      .context("failed to get closed alerts from db")?;

      for closed in &mut closed {
        closed.level = SeverityLevel::Ok;
      }

      let closed = closed
                .into_iter()
                .filter(|closed| {
                    if let ResourceTarget::Server(id) = &closed.target {
                        send_alerts_map.get(id).cloned().unwrap_or(true)
                    } else {
                        error!("got resource target other than server in resolve_server_alerts");
                        true
                    }
                })
                .collect::<Vec<_>>();

      self.send_alerts(&closed).await;

      anyhow::Ok(())
    };

    if let Err(e) = close().await {
      error!("failed to resolve alerts | {e:#?}");
    }
  }

  async fn get_open_alerts(
    &self,
  ) -> anyhow::Result<(
    OpenAlertMap,
    OpenDiskAlertMap,
    OpenTempAlertMap,
  )> {
    let alerts = find_collect(
      &db_client().await.alerts,
      doc! { "resolved": false },
      None,
    )
    .await
    .context("failed to get open alerts from db")?;

    let mut map = OpenAlertMap::new();
    let mut disk_map = OpenDiskAlertMap::new();
    let mut temp_map = OpenTempAlertMap::new();

    for alert in alerts {
      match &alert.data {
        AlertData::ServerDisk { path, .. } => {
          let inner =
            disk_map.entry(alert.target.clone()).or_default();
          inner.insert(path.to_owned(), alert);
        }
        AlertData::ServerTemp { component, .. } => {
          let inner =
            temp_map.entry(alert.target.clone()).or_default();
          inner.insert(component.to_owned(), alert);
        }
        _ => {
          let inner = map.entry(alert.target.clone()).or_default();
          inner.insert(alert.variant, alert);
        }
      }
    }

    Ok((map, disk_map, temp_map))
  }
}
