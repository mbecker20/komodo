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
            cpu: self.sys.global_cpu_info().cpu_usage(),
            mem_used: self.sys.used_memory() as f64 / BYTES_PER_GB,
            mem_total: self.sys.total_memory() as f64 / BYTES_PER_GB,
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
                recieved: n.received() as f64 / BYTES_PER_KB,
                transmitted: n.transmitted() as f64 / BYTES_PER_KB,
            })
            .filter(|n| n.recieved > 0.0 || n.transmitted > 0.0)
            .collect()
    }

    fn get_disk_usage(&mut self) -> DiskUsage {
        self.sys.refresh_disks();
        let mut free = 0.0;
        let mut total = 0.0;
        let mut disks = Vec::new();
        for disk in self.sys.disks() {
            let disk_total = disk.total_space() as f64 / BYTES_PER_GB;
            let disk_free = disk.available_space() as f64 / BYTES_PER_GB;
            disks.push(SingleDiskUsage {
                mount: disk.mount_point().to_owned(),
                used: disk_total - disk_free,
                total: disk_total,
            });
            total += disk_total;
            free += disk_free;
        }
        let used = total - free;
        self.sys
            .refresh_processes_specifics(ProcessRefreshKind::new().with_disk_usage());
        let mut read = 0.0;
        let mut write = 0.0;
        for (_, process) in self.sys.processes() {
            let disk_usage = process.disk_usage();
            read += disk_usage.read_bytes as f64 / BYTES_PER_KB;
            write += disk_usage.written_bytes as f64 / BYTES_PER_KB;
        }
        DiskUsage {
            used,
            total,
            read,
            write,
            disks,
        }
    }
}
