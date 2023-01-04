use async_timing_util::{unix_timestamp_ms, wait_until_timelength, Timelength, ONE_HOUR_MS};
use futures_util::future::join_all;
use mungos::doc;
use slack::types::Block;
use types::{Server, SystemStats, SystemStatsQuery, SystemStatsRecord};

use crate::state::State;

#[derive(Default)]
pub struct AlertStatus {
    cpu_alert: bool,
    mem_alert: bool,
    disk_alert: bool,
    component_alert: bool,
}

impl State {
    pub async fn collect_server_stats(&self) {
        loop {
            let ts = wait_until_timelength(
                self.config.monitoring_interval.to_string().parse().unwrap(),
                0,
            )
            .await as i64;
            let servers = self.get_enabled_servers_with_stats().await;
            if let Err(e) = servers {
                eprintln!("failed to get server list from db: {e:?}");
                continue;
            }
            for (server, res) in servers.unwrap() {
                if let Err(e) = res {
                    println!("server unreachable: {e:?}");
                    if let Some(slack) = &self.slack {
                        let (header, info) = generate_unreachable_message(&server);
                        let res = slack.send_message_with_header(&header, info.clone()).await;
                        if let Err(e) = res {
                            eprintln!("failed to send message to slack: {e} | header: {header} | info: {info:?}")
                        }
                    }
                    continue;
                }
                let stats = res.unwrap();
                self.check_server_stats(&server, &stats).await;
                let res = self
                    .db
                    .stats
                    .create_one(SystemStatsRecord::from_stats(server.id, ts, stats))
                    .await;
                if let Err(e) = res {
                    eprintln!("failed to insert stats into mongo | {e}");
                }
            }
        }
    }

    async fn get_enabled_servers_with_stats(
        &self,
    ) -> anyhow::Result<Vec<(Server, anyhow::Result<SystemStats>)>> {
        let servers = self
            .db
            .servers
            .get_some(doc! { "enabled": true }, None)
            .await?;

        let futures = servers.into_iter().map(|server| async move {
            let stats = self
                .periphery
                .get_system_stats(
                    &server,
                    &SystemStatsQuery {
                        networks: true,
                        components: true,
                        processes: false,
                    },
                )
                .await;
            (server, stats)
        });

        Ok(join_all(futures).await)
    }

    async fn check_server_stats(&self, server: &Server, stats: &SystemStats) {
        self.check_cpu(server, stats).await;
        self.check_mem(server, stats).await;
        self.check_disk(server, stats).await;
        self.check_components(server, stats).await;
    }

    async fn check_cpu(&self, server: &Server, stats: &SystemStats) {
        let lock = self.server_alert_status.lock().await;
        if self.slack.is_none() || lock.get(&server.id).map(|s| s.cpu_alert).unwrap_or(false) {
            return;
        }
        drop(lock);
        if stats.cpu_perc > server.cpu_alert {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *CPU usage* 📈 🚨",
                    server.name
                )),
                Block::section(format!("cpu: *{:.1}%*", stats.cpu_perc)),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *CPU usage* 📈 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high cpu usage on {} | usage: {:.1}% | {e:?}",
                    server.name, stats.cpu_perc
                )
            } else {
                let mut lock = self.server_alert_status.lock().await;
                let entry = lock.entry(server.id.clone()).or_default();
                entry.cpu_alert = true;
            }
        }
    }

    async fn check_mem(&self, server: &Server, stats: &SystemStats) {
        let lock = self.server_alert_status.lock().await;
        if self.slack.is_none() || lock.get(&server.id).map(|s| s.mem_alert).unwrap_or(false) {
            return;
        }
        drop(lock);
        if (stats.mem_used_gb / stats.mem_total_gb) * 100.0 > server.mem_alert {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *memory usage* 💾 🚨",
                    server.name
                )),
                Block::section(format!(
                    "memory: used *{:.2} GB* of *{:.2} GB* (*{:.1}%*)",
                    stats.mem_used_gb,
                    stats.mem_total_gb,
                    (stats.mem_used_gb / stats.mem_total_gb) * 100.0
                )),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *memory usage* 💾 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high mem usage on {} | usage: {:.2}GB of {:.2}GB | {e:?}",
                    server.name, stats.mem_used_gb, stats.mem_total_gb,
                )
            } else {
                let mut lock = self.server_alert_status.lock().await;
                let entry = lock.entry(server.id.clone()).or_default();
                entry.mem_alert = true;
            }
        }
    }

    async fn check_disk(&self, server: &Server, stats: &SystemStats) {
        let lock = self.server_alert_status.lock().await;
        if self.slack.is_none() || lock.get(&server.id).map(|s| s.disk_alert).unwrap_or(false) {
            return;
        }
        drop(lock);
        if (stats.disk.used_gb / stats.disk.total_gb) * 100.0 > server.disk_alert {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *disk usage* 💿 🚨",
                    server.name
                )),
                Block::section(format!(
                    "disk: used *{:.2} GB* of *{:.2} GB* (*{:.1}%*)",
                    stats.disk.used_gb,
                    stats.disk.total_gb,
                    (stats.disk.used_gb / stats.disk.total_gb) * 100.0
                )),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *disk usage* 💿 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high disk usage on {} | usage: {:.2}GB of {:.2}GB | {e:?}",
                    server.name, stats.disk.used_gb, stats.disk.total_gb,
                )
            } else {
                let mut lock = self.server_alert_status.lock().await;
                let entry = lock.entry(server.id.clone()).or_default();
                entry.disk_alert = true;
            }
        }
    }

    async fn check_components(&self, server: &Server, stats: &SystemStats) {
        let lock = self.server_alert_status.lock().await;
        if self.slack.is_none()
            || lock
                .get(&server.id)
                .map(|s| s.component_alert)
                .unwrap_or(false)
        {
            return;
        }
        drop(lock);
        let info = stats
            .components
            .iter()
            .map(|c| {
                if let Some(critical) = c.critical {
                    if c.temp / critical > 0.75 {
                        format!(
                            "{}: *{:.1}°* (*{:.1}%* to critical) 🌡️",
                            c.label,
                            c.temp,
                            (c.temp / critical) * 100.0
                        )
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if info.len() > 0 {
            let region = if let Some(region) = &server.region {
                format!(" ({region})")
            } else {
                String::new()
            };
            let mut blocks = vec![
                Block::header("WARNING 🚨"),
                Block::section(format!(
                    "*{}*{region} has high *tempurature* 🌡️ 🚨",
                    server.name
                )),
                Block::section(info.join("\n")),
            ];

            if let Some(to_notify) = generate_to_notify(server) {
                blocks.push(Block::section(to_notify))
            }

            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(
                    format!(
                        "WARNING 🚨 | *{}*{region} has high *tempurature* 🌡️ 🚨",
                        server.name
                    ),
                    blocks,
                )
                .await;
            if let Err(e) = res {
                eprintln!(
                    "failed to send message to slack | high tempurature on {} | {} | {e:?}",
                    server.name,
                    info.join(" | "),
                )
            } else {
                let mut lock = self.server_alert_status.lock().await;
                let entry = lock.entry(server.id.clone()).or_default();
                entry.component_alert = true;
            }
        }
    }

    pub async fn daily_update(&self) {
        let offset = self.config.daily_offset_hours as u128 * ONE_HOUR_MS;
        loop {
            wait_until_timelength(Timelength::OneDay, offset).await;
            let servers = self.get_enabled_servers_with_stats().await;
            if let Err(e) = &servers {
                eprintln!(
                    "{} | failed to get servers with stats for daily update | {e:#?}",
                    unix_timestamp_ms()
                );
                continue;
            }
            let mut blocks = vec![Block::header("INFO | daily update"), Block::divider()];
            for (server, stats) in servers.unwrap() {
                let region = if let Some(region) = &server.region {
                    format!(" | {region}")
                } else {
                    String::new()
                };
                if let Ok(stats) = stats {
                    let cpu_warning = if stats.cpu_perc > server.cpu_alert {
                        " 🚨"
                    } else {
                        ""
                    };
                    let mem_warning =
                        if (stats.mem_used_gb / stats.mem_total_gb) * 100.0 > server.mem_alert {
                            " 🚨"
                        } else {
                            ""
                        };
                    let disk_warning =
                        if (stats.disk.used_gb / stats.disk.total_gb) * 100.0 > server.disk_alert {
                            " 🚨"
                        } else {
                            ""
                        };
                    let status = if !cpu_warning.is_empty()
                        || !mem_warning.is_empty()
                        || !disk_warning.is_empty()
                    {
                        "*WARNING* 🚨"
                    } else {
                        "*OK* ✅"
                    };
                    let name_line = format!("*{}*{region} | {status}", server.name);
                    let cpu_line = format!("CPU: *{:.1}%*{cpu_warning}", stats.cpu_perc);
                    let mem_line = format!(
                        "MEM: *{:.1}%* ({:.2} GB of {:.2} GB){mem_warning}",
                        (stats.mem_used_gb / stats.mem_total_gb) * 100.0,
                        stats.mem_used_gb,
                        stats.mem_total_gb,
                    );
                    let disk_line = format!(
                        "DISK: *{:.1}%* ({:.2} GB of {:.2} GB){disk_warning}",
                        (stats.disk.used_gb / stats.disk.total_gb) * 100.0,
                        stats.disk.used_gb,
                        stats.disk.total_gb,
                    );
                    blocks.push(Block::section(format!(
                        "{name_line}\n{cpu_line}\n{mem_line}\n{disk_line}",
                    )));
                } else {
                    blocks.push(Block::section(format!(
                        "*{}*{region} | *UNREACHABLE* ❌",
                        server.name
                    )));
                }
                blocks.push(Block::divider())
            }
            let res = self
                .slack
                .as_ref()
                .unwrap()
                .send_message(format!("INFO | daily update"), blocks)
                .await;
            if let Err(e) = res {
                eprintln!(
                    "{} | failed to send daily update message | {e:?}",
                    unix_timestamp_ms()
                );
            }
            {
                self.server_alert_status.lock().await.clear();
            }
        }
    }
}

fn generate_unreachable_message(server: &Server) -> (String, Option<String>) {
    let region = match &server.region {
        Some(region) => format!(" ({region})"),
        None => String::new(),
    };
    let header = format!("WARNING 🚨 | {}{region} is unreachable ❌", server.name);
    let to_notify = server
        .to_notify
        .iter()
        .map(|u| format!("<@{u}>"))
        .collect::<Vec<_>>()
        .join(" ");
    let info = if to_notify.len() > 0 {
        Some(to_notify)
    } else {
        None
    };
    (header, info)
}

fn generate_to_notify(server: &Server) -> Option<String> {
    if server.to_notify.len() > 0 {
        Some(
            server
                .to_notify
                .iter()
                .map(|u| format!("<@{u}>"))
                .collect::<Vec<String>>()
                .join(" "),
        )
    } else {
        None
    }
}
