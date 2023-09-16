use monitor_types::entities::server::stats::{
    BasicSystemStats, SystemStatsRecord,
};

use crate::state::State;

impl State {
    pub async fn record_server_stats(&self, ts: i64) {
        let status = self.server_status_cache.get_list().await;
        let records = status
            .into_iter()
            .filter(|status| status.stats.is_some())
            .map(|status| {
                let BasicSystemStats {
                    system_load,
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
                    system_load,
                    cpu_perc,
                    cpu_freq_mhz,
                    mem_total_gb,
                    mem_used_gb,
                    disk_total_gb,
                    disk_used_gb,
                }
            })
            .collect::<Vec<_>>();
        let res = self.db.stats.create_many(records).await;
        if let Err(e) = res {
            error!("failed to record server stats | {e:#?}");
        }
    }
}
