use monitor_client::entities::server::stats::{
  sum_disk_usage, SystemStatsRecord, TotalDiskUsage,
};

use crate::{db::db_client, helpers::cache::server_status_cache};

#[instrument(level = "debug")]
pub async fn record_server_stats(ts: i64) {
  let status = server_status_cache().get_list().await;
  let records = status
    .into_iter()
    .filter_map(|status| {
      let stats = status.stats.as_ref()?;

      let TotalDiskUsage {
        used_gb: disk_used_gb,
        total_gb: disk_total_gb,
      } = sum_disk_usage(&stats.disks);

      Some(SystemStatsRecord {
        ts,
        sid: status.id.clone(),
        cpu_perc: stats.cpu_perc,
        mem_total_gb: stats.mem_total_gb,
        mem_used_gb: stats.mem_used_gb,
        disk_total_gb,
        disk_used_gb,
        disks: stats.disks.clone(),
      })
    })
    .collect::<Vec<_>>();
  if !records.is_empty() {
    let res =
      db_client().await.stats.insert_many(records, None).await;
    if let Err(e) = res {
      error!("failed to record server stats | {e:#}");
    }
  }
}
