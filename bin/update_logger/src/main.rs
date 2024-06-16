#[macro_use]
extern crate tracing;

use monitor_client::MonitorClient;

async fn app() -> anyhow::Result<()> {
  logger::init(&Default::default())?;

  info!("v {}", env!("CARGO_PKG_VERSION"));

  let monitor = MonitorClient::new_from_env().await?;

  let (mut rx, _) = monitor.subscribe_to_updates(1000, 5)?;

  loop {
    let msg = rx.recv().await;
    if let Err(e) = msg {
      error!("🚨 recv error | {e:?}");
      break;
    }
    info!("{msg:?}");
  }

  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut term_signal = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;

  let app = tokio::spawn(app());

  tokio::select! {
    res = app => return res?,
    _ = term_signal.recv() => {},
  }

  Ok(())
}
