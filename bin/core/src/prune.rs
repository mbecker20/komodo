use async_timing_util::{
  unix_timestamp_ms, wait_until_timelength, Timelength, ONE_DAY_MS,
};
use mungos::mongodb::bson::doc;

use crate::{config::core_config, db::db_client};

pub fn spawn_prune_loop() {
  tokio::spawn(async move {
    loop {
      wait_until_timelength(Timelength::OneDay, 5000).await;
      let (stats_res, alerts_res) =
        tokio::join!(prune_stats(), prune_alerts());
      if let Err(e) = stats_res {
        error!("error in pruning stats | {e:#?}");
      }
      if let Err(e) = alerts_res {
        error!("error in pruning alerts | {e:#?}");
      }
    }
  });
}

async fn prune_stats() -> anyhow::Result<()> {
  if core_config().keep_stats_for_days == 0 {
    return Ok(());
  }
  let delete_before_ts = (unix_timestamp_ms()
    - core_config().keep_stats_for_days as u128 * ONE_DAY_MS)
    as i64;
  let res = db_client()
    .await
    .stats
    .delete_many(
      doc! {
          "ts": { "$lt": delete_before_ts }
      },
      None,
    )
    .await?;
  info!("deleted {} stats from db", res.deleted_count);
  Ok(())
}

async fn prune_alerts() -> anyhow::Result<()> {
  if core_config().keep_alerts_for_days == 0 {
    return Ok(());
  }
  let delete_before_ts = (unix_timestamp_ms()
    - core_config().keep_alerts_for_days as u128 * ONE_DAY_MS)
    as i64;
  let res = db_client()
    .await
    .alerts
    .delete_many(
      doc! {
          "ts": { "$lt": delete_before_ts }
      },
      None,
    )
    .await?;
  info!("deleted {} alerts from db", res.deleted_count);
  Ok(())
}
