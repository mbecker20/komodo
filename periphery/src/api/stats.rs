use std::sync::{Arc, RwLock};

use async_timing_util::wait_until_timelength;
use axum::{routing::get, Extension, Json, Router};
use sysinfo::{
    ComponentExt, CpuExt, DiskExt, NetworkExt, ProcessExt, ProcessRefreshKind, SystemExt, PidExt,
};
use types::{
    DiskUsage, SingleDiskUsage, SystemComponent, SystemNetwork, SystemProcess, SystemStats,
    Timelength,
};

pub fn router(stats_polling_rate: Timelength) -> Router {
    Router::new()
        .route(
            "/system",
            get(|Extension(sys): StatsExtension| async move {
                let stats = sys.read().unwrap().get_stats();
                Json(stats)
            }),
        )
        .layer(StatsClient::extension(stats_polling_rate))
}

type StatsExtension = Extension<Arc<RwLock<StatsClient>>>;

struct StatsClient {
    sys: sysinfo::System,
    polling_rate: Timelength,
    refresh_ts: u128,
    refresh_list_ts: u128,
}

const BYTES_PER_GB: f64 = 1073741824.0;
const BYTES_PER_KB: f64 = 1024.0;

impl StatsClient {
    pub fn extension(polling_rate: Timelength) -> StatsExtension {
        let client = StatsClient {
            sys: sysinfo::System::new_all(),
            polling_rate,
            refresh_ts: 0,
            refresh_list_ts: 0,
        };
        let client = Arc::new(RwLock::new(client));
        let clone = client.clone();
        tokio::spawn(async move {
            let polling_rate = polling_rate.to_string().parse().unwrap();
            loop {
                let ts = wait_until_timelength(polling_rate, 0).await;
                let mut client = clone.write().unwrap();
                client.refresh();
                client.refresh_ts = ts;
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

    pub fn get_stats(&self) -> SystemStats {
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
            .map(|(pid, p)| SystemProcess {
                pid: pid.as_u32(),
                name: p.name().to_string(),

            })
            .collect()
    }
}
