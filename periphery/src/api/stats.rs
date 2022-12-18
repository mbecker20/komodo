use std::sync::{Arc, RwLock};

use axum::{routing::get, Extension, Json, Router};
use sysinfo::{CpuExt, DiskExt, NetworkExt, ProcessExt, ProcessRefreshKind, SystemExt};
use types::{DiskUsage, SingleDiskUsage, SystemNetwork, SystemStats};

pub fn router() -> Router {
    Router::new()
        .route(
            "/system",
            get(|Extension(sys): StatsExtension| async move {
                let stats = sys.write().unwrap().get_stats();
                Json(stats)
            }),
        )
        .layer(StatsClient::extension())
}

type StatsExtension = Extension<Arc<RwLock<StatsClient>>>;

struct StatsClient {
    sys: sysinfo::System,
}

const BYTES_PER_GB: f64 = 1073741824.0;
const BYTES_PER_KB: f64 = 1024.0;

impl StatsClient {
    pub fn extension() -> StatsExtension {
        let client = StatsClient {
            sys: sysinfo::System::new_all(),
        };
        Extension(Arc::new(RwLock::new(client)))
    }

    pub fn get_stats(&mut self) -> SystemStats {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        SystemStats {
            cpu_perc: self.sys.global_cpu_info().cpu_usage(),
            mem_used_gb: self.sys.used_memory() as f64 / BYTES_PER_GB,
            mem_total_gb: self.sys.total_memory() as f64 / BYTES_PER_GB,
            disk: self.get_disk_usage(),
            networks: self.get_networks(),
        }
    }

    fn get_networks(&mut self) -> Vec<SystemNetwork> {
        self.sys.refresh_networks();
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

    fn get_disk_usage(&mut self) -> DiskUsage {
        self.sys.refresh_disks();
        let mut free_gb = 0.0;
        let mut total_gb = 0.0;
        let mut disks = Vec::new();
        for disk in self.sys.disks() {
            let disk_total = disk.total_space() as f64 / BYTES_PER_GB;
            let disk_free = disk.available_space() as f64 / BYTES_PER_GB;
            let mount = disk.mount_point().to_owned();
            let mount_str = mount.to_str().unwrap();
            if mount_str == "/" || mount_str.contains("external") {
                total_gb += disk_total;
                free_gb += disk_free;
            }
            disks.push(SingleDiskUsage {
                mount,
                used_gb: disk_total - disk_free,
                total_gb: disk_total,
            });
        }
        let used_gb = total_gb - free_gb;
        self.sys
            .refresh_processes_specifics(ProcessRefreshKind::new().with_disk_usage());
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
}
