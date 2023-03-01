use std::{
    cmp::Ordering,
    sync::{Arc, RwLock},
};

use async_timing_util::wait_until_timelength;
use axum::{extract::Query, routing::get, Extension, Json, Router};
use sysinfo::{ComponentExt, CpuExt, DiskExt, NetworkExt, PidExt, ProcessExt, SystemExt};
use types::{
    DiskUsage, SingleCpuUsage, SingleDiskUsage, SystemComponent, SystemInformation, SystemNetwork,
    SystemProcess, SystemStats, SystemStatsQuery, Timelength,
};

pub fn router() -> Router {
    Router::new().route(
        "/",
        get(
            |sys: StatsExtension, Query(query): Query<SystemStatsQuery>| async move {
                let stats = sys.read().unwrap().get_cached_stats(query);
                Json(stats)
            },
        ),
    )
    // .route(
    //     "/ws",
    //     get(
    //         |sys: StatsExtension,
    //          Query(query): Query<SystemStatsQuery>,
    //          ws: WebSocketUpgrade| async move {
    //             sys.read().unwrap().ws_subscribe(ws, Arc::new(query))
    //         },
    //     ),
    // )
}

pub type StatsExtension = Extension<Arc<RwLock<StatsClient>>>;

pub struct StatsClient {
    pub info: SystemInformation,
    sys: sysinfo::System,
    cache: SystemStats,
    polling_rate: Timelength,
    refresh_ts: u128,
    refresh_list_ts: u128,
    // receiver: Receiver<SystemStats>,
}

const BYTES_PER_GB: f64 = 1073741824.0;
const BYTES_PER_MB: f64 = 1048576.0;
const BYTES_PER_KB: f64 = 1024.0;

impl StatsClient {
    pub fn extension(polling_rate: Timelength) -> StatsExtension {
        // let (sender, receiver) = broadcast::channel::<SystemStats>(10);
        let sys = sysinfo::System::new_all();
        let client = StatsClient {
            info: get_system_information(&sys),
            sys,
            cache: SystemStats::default(),
            polling_rate,
            refresh_ts: 0,
            refresh_list_ts: 0,
            // receiver,
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
                // sender
                //     .send(clone.read().unwrap().cache.clone())
                //     .expect("failed to broadcast new stats to reciever");
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

    // fn ws_subscribe(
    //     &self,
    //     ws: WebSocketUpgrade,
    //     query: Arc<SystemStatsQuery>,
    // ) -> impl IntoResponse {
    //     // println!("client subscribe");
    //     let mut reciever = self.get_receiver();
    //     ws.on_upgrade(|socket| async move {
    //         let (mut ws_sender, mut ws_reciever) = socket.split();
    //         let cancel = CancellationToken::new();
    //         let cancel_clone = cancel.clone();
    //         tokio::spawn(async move {
    //             loop {
    //                 let mut stats = select! {
    //                     _ = cancel_clone.cancelled() => break,
    //                     stats = reciever.recv() => { stats.expect("failed to recv stats msg") }
    //                 };
    //                 if query.cpus {
    //                     stats.cpus = vec![]
    //                 }
    //                 if !query.disks {
    //                     stats.disk.disks = vec![]
    //                 }
    //                 if !query.components {
    //                     stats.components = vec![]
    //                 }
    //                 if !query.networks {
    //                     stats.networks = vec![]
    //                 }
    //                 if !query.processes {
    //                     stats.processes = vec![]
    //                 }
    //                 let _ = ws_sender
    //                     .send(Message::Text(serde_json::to_string(&stats).unwrap()))
    //                     .await;
    //             }
    //         });
    //         while let Some(msg) = ws_reciever.next().await {
    //             match msg {
    //                 Ok(msg) => match msg {
    //                     Message::Close(_) => {
    //                         // println!("client CLOSE");
    //                         cancel.cancel();
    //                         return;
    //                     }
    //                     _ => {}
    //                 },
    //                 Err(_) => {
    //                     // println!("client CLOSE");
    //                     cancel.cancel();
    //                     return;
    //                 }
    //             }
    //         }
    //     })
    // }

    // fn get_receiver(&self) -> Receiver<SystemStats> {
    //     self.receiver.resubscribe()
    // }

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
        SystemStats {
            system_load: self.cache.system_load,
            cpu_perc: self.cache.cpu_perc,
            cpu_freq_mhz: self.cache.cpu_freq_mhz,
            mem_used_gb: self.cache.mem_used_gb,
            mem_total_gb: self.cache.mem_total_gb,
            disk: DiskUsage {
                used_gb: self.cache.disk.used_gb,
                total_gb: self.cache.disk.total_gb,
                read_kb: self.cache.disk.read_kb,
                write_kb: self.cache.disk.write_kb,
                disks: if query.disks {
                    self.cache.disk.disks.clone()
                } else {
                    vec![]
                },
            },
            cpus: if query.cpus {
                self.cache.cpus.clone()
            } else {
                vec![]
            },
            networks: if query.networks {
                self.cache.networks.clone()
            } else {
                vec![]
            },
            components: if query.components {
                self.cache.components.clone()
            } else {
                vec![]
            },
            processes: if query.processes {
                self.cache.processes.clone()
            } else {
                vec![]
            },
            polling_rate: self.cache.polling_rate,
            refresh_ts: self.cache.refresh_ts,
            refresh_list_ts: self.cache.refresh_list_ts,
        }
    }

    fn get_stats(&self) -> SystemStats {
        let cpu = self.sys.global_cpu_info();
        SystemStats {
            system_load: self.sys.load_average().one,
            cpu_perc: self.sys.global_cpu_info().cpu_usage(),
            cpu_freq_mhz: cpu.frequency() as f64,
            mem_used_gb: self.sys.used_memory() as f64 / BYTES_PER_GB,
            mem_total_gb: self.sys.total_memory() as f64 / BYTES_PER_GB,
            disk: self.get_disk_usage(),
            cpus: self.get_cpus(),
            networks: self.get_networks(),
            components: self.get_components(),
            processes: self.get_processes(),
            polling_rate: self.polling_rate,
            refresh_ts: self.refresh_ts,
            refresh_list_ts: self.refresh_list_ts,
        }
    }

    fn get_cpus(&self) -> Vec<SingleCpuUsage> {
        self.sys
            .cpus()
            .into_iter()
            .map(|cpu| SingleCpuUsage {
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage(),
            })
            .collect()
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
            let disk_total = disk.total_space() as f64 / BYTES_PER_GB;
            let disk_free = disk.available_space() as f64 / BYTES_PER_GB;
            total_gb += disk_total;
            free_gb += disk_free;
            disks.push(SingleDiskUsage {
                mount: disk.mount_point().to_owned(),
                used_gb: disk_total - disk_free,
                total_gb: disk_total,
            });
        }
        let used_gb = total_gb - free_gb;
        let mut read_bytes = 0;
        let mut write_bytes = 0;
        for (_, process) in self.sys.processes() {
            let disk_usage = process.disk_usage();
            read_bytes += disk_usage.read_bytes;
            write_bytes += disk_usage.written_bytes;
        }
        DiskUsage {
            used_gb,
            total_gb,
            read_kb: read_bytes as f64 / BYTES_PER_KB,
            write_kb: write_bytes as f64 / BYTES_PER_KB,
            disks,
        }
    }

    fn get_components(&self) -> Vec<SystemComponent> {
        let mut comps: Vec<_> = self
            .sys
            .components()
            .into_iter()
            .map(|c| SystemComponent {
                label: c.label().to_string(),
                temp: c.temperature(),
                max: c.max(),
                critical: c.critical(),
            })
            .collect();
        comps.sort_by(|a, b| {
            if a.critical.is_some() {
                if b.critical.is_some() {
                    let a_perc = a.temp as f32 / *a.critical.as_ref().unwrap() as f32;
                    let b_perc = b.temp as f32 / *b.critical.as_ref().unwrap() as f32;
                    if a_perc > b_perc {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Less
                }
            } else {
                if b.critical.is_some() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        });
        comps
    }

    fn get_processes(&self) -> Vec<SystemProcess> {
        let mut procs: Vec<_> = self
            .sys
            .processes()
            .into_iter()
            .map(|(pid, p)| {
                let disk_usage = p.disk_usage();
                SystemProcess {
                    pid: pid.as_u32(),
                    name: p.name().to_string(),
                    exe: p.exe().to_str().unwrap_or("").to_string(),
                    cmd: p.cmd().to_vec(),
                    start_time: (p.start_time() * 1000) as f64,
                    cpu_perc: p.cpu_usage(),
                    mem_mb: p.memory() as f64 / BYTES_PER_MB,
                    disk_read_kb: disk_usage.read_bytes as f64 / BYTES_PER_KB,
                    disk_write_kb: disk_usage.written_bytes as f64 / BYTES_PER_KB,
                }
            })
            .collect();
        procs.sort_by(|a, b| {
            if a.cpu_perc > b.cpu_perc {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        procs
    }
}

fn get_system_information(sys: &sysinfo::System) -> SystemInformation {
    let cpu = sys.global_cpu_info();
    SystemInformation {
        name: sys.name(),
        os: sys.long_os_version(),
        kernel: sys.kernel_version(),
        core_count: sys.physical_core_count().map(|c| c as u32),
        host_name: sys.host_name(),
        cpu_brand: cpu.brand().to_string(),
    }
}
