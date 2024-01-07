use std::{cmp::Ordering, sync::Arc};

use async_timing_util::wait_until_timelength;
use monitor_client::entities::{
  server::stats::{
    AllSystemStats, BasicSystemStats, CpuUsage, DiskUsage,
    LoadAverage, NetworkUsage, SingleCpuUsage, SingleDiskUsage,
    SystemComponent, SystemInformation, SystemNetwork, SystemProcess,
  },
  Timelength,
};
use sysinfo::System;
use tokio::sync::RwLock;

pub type StatsClient = Arc<RwLock<InnerStatsClient>>;

pub struct InnerStatsClient {
  pub info: SystemInformation,
  pub stats: AllSystemStats,
  system: sysinfo::System,
  disks: sysinfo::Disks,
  components: sysinfo::Components,
  networks: sysinfo::Networks,
}

const BYTES_PER_GB: f64 = 1073741824.0;
const BYTES_PER_MB: f64 = 1048576.0;
const BYTES_PER_KB: f64 = 1024.0;

impl InnerStatsClient {
  pub fn new(polling_rate: Timelength) -> StatsClient {
    let system = sysinfo::System::new_all();
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let components = sysinfo::Components::new_with_refreshed_list();
    let networks = sysinfo::Networks::new_with_refreshed_list();
    let stats = AllSystemStats {
      polling_rate,
      ..Default::default()
    };
    let client = InnerStatsClient {
      info: get_system_information(&system),
      system,
      disks,
      components,
      networks,
      stats,
    };
    let client = Arc::new(RwLock::new(client));
    let clone = client.clone();
    tokio::spawn(async move {
      loop {
        let ts = wait_until_timelength(
          async_timing_util::Timelength::FiveMinutes,
          0,
        )
        .await;
        let mut client = clone.write().await;
        client.refresh_lists();
        client.stats.refresh_list_ts = ts as i64;
      }
    });
    let clone = client.clone();
    tokio::spawn(async move {
      let polling_rate = polling_rate.to_string().parse().unwrap();
      loop {
        let ts = wait_until_timelength(polling_rate, 1).await;
        let mut client = clone.write().await;
        client.refresh();
        client.stats = client.get_all_stats();
        client.stats.refresh_ts = ts as i64;
      }
    });
    client
  }

  fn refresh(&mut self) {
    self.system.refresh_cpu();
    self.system.refresh_memory();
    self.system.refresh_processes();

    self.networks.refresh();
    self.components.refresh();
    self.disks.refresh();
  }

  fn refresh_lists(&mut self) {
    self.networks.refresh_list();
    self.disks.refresh_list();
    self.components.refresh_list();
  }

  fn get_all_stats(&self) -> AllSystemStats {
    AllSystemStats {
      basic: self.get_basic_system_stats(),
      cpu: self.get_cpu_usage(),
      disk: self.get_disk_usage(),
      network: self.get_network_usage(),
      processes: self.get_processes(),
      components: self.get_components(),
      polling_rate: self.stats.polling_rate,
      refresh_ts: self.stats.refresh_ts,
      refresh_list_ts: self.stats.refresh_list_ts,
    }
  }

  fn get_basic_system_stats(&self) -> BasicSystemStats {
    let cpu = self.system.global_cpu_info();
    let disk = self.get_disk_usage();
    let load_average = System::load_average();
    BasicSystemStats {
      load_average: LoadAverage {
        one: load_average.one,
        five: load_average.five,
        fifteen: load_average.fifteen,
      },
      cpu_perc: cpu.cpu_usage(),
      cpu_freq_mhz: cpu.frequency() as f64,
      mem_used_gb: self.system.used_memory() as f64 / BYTES_PER_GB,
      mem_total_gb: self.system.total_memory() as f64 / BYTES_PER_GB,
      disk_used_gb: disk.used_gb,
      disk_total_gb: disk.total_gb,
    }
  }

  fn get_cpu_usage(&self) -> CpuUsage {
    let cpu = self.system.global_cpu_info();
    CpuUsage {
      cpu_perc: cpu.cpu_usage(),
      cpu_freq_mhz: cpu.frequency() as f64,
      cpus: self
        .system
        .cpus()
        .iter()
        .map(|cpu| SingleCpuUsage {
          name: cpu.name().to_string(),
          usage: cpu.cpu_usage(),
        })
        .collect(),
    }
  }

  fn get_disk_usage(&self) -> DiskUsage {
    let mut free_gb = 0.0;
    let mut total_gb = 0.0;
    let disks = self
      .disks
      .list()
      .iter()
      .map(|disk| {
        let disk_total = disk.total_space() as f64 / BYTES_PER_GB;
        let disk_free = disk.available_space() as f64 / BYTES_PER_GB;
        total_gb += disk_total;
        free_gb += disk_free;
        SingleDiskUsage {
          mount: disk.mount_point().to_owned(),
          used_gb: disk_total - disk_free,
          total_gb: disk_total,
        }
      })
      .collect::<Vec<_>>();
    let used_gb = total_gb - free_gb;
    let mut read_bytes = 0;
    let mut write_bytes = 0;
    for process in self.system.processes().values() {
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

  fn get_network_usage(&self) -> NetworkUsage {
    let mut recieved_kb = 0.0;
    let mut transmitted_kb = 0.0;
    let networks = self
      .networks
      .list()
      .iter()
      .map(|(name, n)| {
        let recv = n.received() as f64 / BYTES_PER_KB;
        let trans = n.transmitted() as f64 / BYTES_PER_KB;
        recieved_kb += recv;
        transmitted_kb += trans;
        SystemNetwork {
          name: name.clone(),
          recieved_kb: recv,
          transmitted_kb: trans,
        }
      })
      .filter(|n| n.recieved_kb > 0.0 || n.transmitted_kb > 0.0)
      .collect::<Vec<_>>();
    NetworkUsage {
      networks,
      recieved_kb,
      transmitted_kb,
    }
  }

  fn get_components(&self) -> Vec<SystemComponent> {
    let mut comps: Vec<_> = self
      .components
      .list()
      .iter()
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
          let a_perc = a.temp / *a.critical.as_ref().unwrap();
          let b_perc = b.temp / *b.critical.as_ref().unwrap();
          if a_perc > b_perc {
            Ordering::Less
          } else {
            Ordering::Greater
          }
        } else {
          Ordering::Less
        }
      } else if b.critical.is_some() {
        Ordering::Greater
      } else {
        Ordering::Equal
      }
    });
    comps
  }

  fn get_processes(&self) -> Vec<SystemProcess> {
    let mut procs: Vec<_> = self
      .system
      .processes()
      .iter()
      .map(|(pid, p)| {
        let disk_usage = p.disk_usage();
        SystemProcess {
          pid: pid.as_u32(),
          name: p.name().to_string(),
          exe: p
            .exe()
            .map(|exe| exe.to_str().unwrap_or_default())
            .unwrap_or_default()
            .to_string(),
          cmd: p.cmd().to_vec(),
          start_time: (p.start_time() * 1000) as f64,
          cpu_perc: p.cpu_usage(),
          mem_mb: p.memory() as f64 / BYTES_PER_MB,
          disk_read_kb: disk_usage.read_bytes as f64 / BYTES_PER_KB,
          disk_write_kb: disk_usage.written_bytes as f64
            / BYTES_PER_KB,
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

fn get_system_information(
  sys: &sysinfo::System,
) -> SystemInformation {
  let cpu = sys.global_cpu_info();
  SystemInformation {
    name: System::name(),
    os: System::long_os_version(),
    kernel: System::kernel_version(),
    host_name: System::host_name(),
    core_count: sys.physical_core_count().map(|c| c as u32),
    cpu_brand: cpu.brand().to_string(),
  }
}
