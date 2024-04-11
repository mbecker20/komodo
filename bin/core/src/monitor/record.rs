use monitor_client::entities::server::stats::{
  BasicSystemStats, SystemStatsRecord,
};

use crate::{db::db_client, helpers::cache::server_status_cache};

#[instrument(level = "debug")]
pub async fn record_server_stats(ts: i64) {
  let status = server_status_cache().get_list().await;
  let records = status
    .into_iter()
    .filter(|status| status.stats.is_some())
    .map(|status| {
      let BasicSystemStats {
        load_average,
        cpu_perc,
        cpu_freq_mhz,
        mem_total_gb,
        mem_used_gb,
        disk_total_gb,
        disk_used_gb,
        ..
      } = status.stats.as_ref().unwrap().basic;
      SystemStatsRecord {
        ts,
        sid: status.id.clone(),
        load_average,
        cpu_perc,
        cpu_freq_mhz,
        mem_total_gb,
        mem_used_gb,
        disk_total_gb,
        disk_used_gb,
      }
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
