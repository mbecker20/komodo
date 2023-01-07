use std::sync::{Arc, RwLock};

use async_timing_util::wait_until_timelength;
use axum::{
    extract::{ws::Message, Query, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use sysinfo::{ComponentExt, CpuExt, DiskExt, NetworkExt, PidExt, ProcessExt, SystemExt};
use tokio::{
    select,
    sync::broadcast::{self, Receiver},
};
use tokio_util::sync::CancellationToken;
use types::{
    DiskUsage, SingleDiskUsage, SystemComponent, SystemNetwork, SystemProcess, SystemStats,
    SystemStatsQuery, Timelength,
};

pub fn router(stats_polling_rate: Timelength) -> Router {
    Router::new()
        .route(
            "/",
            get(
                |Extension(sys): StatsExtension, Query(query): Query<SystemStatsQuery>| async move {
                    let stats = sys.read().unwrap().get_cached_stats(query);
                    Json(stats)
                },
            ),
        )
        .route(
            "/ws",
            get(
                |Extension(sys): StatsExtension,
                 Query(query): Query<SystemStatsQuery>,
                 ws: WebSocketUpgrade| async move {
                    sys.read().unwrap().ws_subscribe(ws, Arc::new(query))
                },
            ),
        )
        .layer(StatsClient::extension(stats_polling_rate))
}

type StatsExtension = Extension<Arc<RwLock<StatsClient>>>;

struct StatsClient {
    sys: sysinfo::System,
    cache: SystemStats,
    polling_rate: Timelength,
    refresh_ts: u128,
    refresh_list_ts: u128,
    receiver: Receiver<SystemStats>,
}

const BYTES_PER_GB: f64 = 1073741824.0;
const BYTES_PER_MB: f64 = 1048576.0;
const BYTES_PER_KB: f64 = 1024.0;

impl StatsClient {
    fn extension(polling_rate: Timelength) -> StatsExtension {
        let (sender, receiver) = broadcast::channel::<SystemStats>(10);
        let client = StatsClient {
            sys: sysinfo::System::new_all(),
            cache: SystemStats::default(),
            polling_rate,
            refresh_ts: 0,
            refresh_list_ts: 0,
            receiver,
        };
        let client = Arc::new(RwLock::new(client));
        let clone = client.clone();
        tokio::spawn(async move {
            let polling_rate = polling_rate.to_string().parse().unwrap();
            loop {
                let ts = wait_until_timelength(polling_rate, 0).await;
                {
                    let mut client = clone.write().unwrap();
                    client.refresh();
                    client.refresh_ts = ts;
                    client.cache = client.get_stats();
                }
                sender
                    .send(clone.read().unwrap().cache.clone())
                    .expect("failed to broadcast new stats to reciever");
            }
        });
        let clone = client.clone();
        tokio::spawn(async move {
            loop {
                let ts = wait_until_timelength(async_timing_util::Timelength::FiveMinutes, 0).await;
                let mut client = clone.write().unwrap();
                client.refresh_lists();
                client.refresh_list_ts = ts;
            }
        });
        Extension(client)
    }

    fn ws_subscribe(
        &self,
        ws: WebSocketUpgrade,
        query: Arc<SystemStatsQuery>,
    ) -> impl IntoResponse {
        // println!("client subscribe");
        let mut reciever = self.get_receiver();
        ws.on_upgrade(|socket| async move {
            let (mut ws_sender, mut ws_reciever) = socket.split();
            let cancel = CancellationToken::new();
            let cancel_clone = cancel.clone();
            tokio::spawn(async move {
                loop {
                    let mut stats = select! {
                        _ = cancel_clone.cancelled() => break,
                        stats = reciever.recv() => { stats.expect("failed to recv stats msg") }
                    };
                    if !query.disks {
                        stats.disk.disks = vec![]
                    }
                    if !query.components {
                        stats.components = vec![]
                    }
                    if !query.networks {
                        stats.networks = vec![]
                    }
                    if !query.processes {
                        stats.processes = vec![]
                    }
                    let _ = ws_sender
                        .send(Message::Text(serde_json::to_string(&stats).unwrap()))
                        .await;
                }
            });
            while let Some(msg) = ws_reciever.next().await {
                match msg {
                    Ok(msg) => match msg {
                        Message::Close(_) => {
                            // println!("client CLOSE");
                            cancel.cancel();
                            return;
                        }
                        _ => {}
                    },
                    Err(_) => {
                        // println!("client CLOSE");
                        cancel.cancel();
                        return;
                    }
                }
            }
        })
    }

    fn get_receiver(&self) -> Receiver<SystemStats> {
        self.receiver.resubscribe()
    }

    fn refresh(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_networks();
        self.sys.refresh_disks();
        self.sys.refresh_components();
        self.sys.refresh_processes();
    }

    fn refresh_lists(&mut self) {
        self.sys.refresh_networks_list();
        self.sys.refresh_disks_list();
        self.sys.refresh_components_list();
    }

    fn get_cached_stats(&self, query: SystemStatsQuery) -> SystemStats {
        let mut stats = self.cache.clone();
        if !query.disks {
            stats.disk.disks = vec![]
        }
        if !query.networks {
            stats.networks = vec![];
        }
        if !query.components {
            stats.components = vec![];
        }
        if !query.processes {
            stats.processes = vec![];
        }
        stats
    }

    fn get_stats(&self) -> SystemStats {
        SystemStats {
            cpu_perc: self.sys.global_cpu_info().cpu_usage(),
            mem_used_gb: self.sys.used_memory() as f64 / BYTES_PER_GB,
            mem_total_gb: self.sys.total_memory() as f64 / BYTES_PER_GB,
            disk: self.get_disk_usage(),
            networks: self.get_networks(),
            components: self.get_components(),
            processes: self.get_processes(),
            polling_rate: self.polling_rate,
            refresh_ts: self.refresh_ts,
            refresh_list_ts: self.refresh_list_ts,
        }
    }

    fn get_networks(&self) -> Vec<SystemNetwork> {
        self.sys
            .networks()
            .into_iter()
            .map(|(name, n)| SystemNetwork {
                name: name.clone(),
                recieved_kb: n.received() as f64 / BYTES_PER_KB,
                transmitted_kb: n.transmitted() as f64 / BYTES_PER_KB,
            })
            .filter(|n| n.recieved_kb > 0.0 || n.transmitted_kb > 0.0)
            .collect()
    }

    fn get_disk_usage(&self) -> DiskUsage {
        let mut free_gb = 0.0;
        let mut total_gb = 0.0;
        let mut disks = Vec::new();
        for disk in self.sys.disks() {
            let mount = disk.mount_point().to_owned();
            let mount_str = mount.to_str().unwrap();
            if mount_str == "/" || mount_str.contains("external") {
                let disk_total = disk.total_space() as f64 / BYTES_PER_GB;
                let disk_free = disk.available_space() as f64 / BYTES_PER_GB;
                total_gb += disk_total;
                free_gb += disk_free;
                disks.push(SingleDiskUsage {
                    mount,
                    used_gb: disk_total - disk_free,
                    total_gb: disk_total,
                });
            }
        }
        let used_gb = total_gb - free_gb;
        let mut read_kb = 0.0;
        let mut write_kb = 0.0;
        for (_, process) in self.sys.processes() {
            let disk_usage = process.disk_usage();
            read_kb += disk_usage.read_bytes as f64 / BYTES_PER_KB;
            write_kb += disk_usage.written_bytes as f64 / BYTES_PER_KB;
        }
        DiskUsage {
            used_gb,
            total_gb,
            read_kb,
            write_kb,
            disks,
        }
    }

    fn get_components(&self) -> Vec<SystemComponent> {
        self.sys
            .components()
            .into_iter()
            .map(|c| SystemComponent {
                label: c.label().to_string(),
                temp: c.temperature(),
                max: c.max(),
                critical: c.critical(),
            })
            .collect()
    }

    fn get_processes(&self) -> Vec<SystemProcess> {
        self.sys
            .processes()
            .into_iter()
            .map(|(pid, p)| {
                let disk_usage = p.disk_usage();
                SystemProcess {
                    pid: pid.as_u32(),
                    name: p.name().to_string(),
                    exe: p.exe().to_str().unwrap_or("").to_string(),
                    cmd: p.cmd().to_vec(),
                    cpu_perc: p.cpu_usage(),
                    mem_mb: p.memory() as f64 / BYTES_PER_MB,
                    disk_read_kb: disk_usage.read_bytes as f64 / BYTES_PER_KB,
                    disk_write_kb: disk_usage.written_bytes as f64 / BYTES_PER_KB,
                }
            })
            .collect()
    }
}
