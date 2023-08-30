#[macro_use]
extern crate log;

use monitor_client::MonitorClient;
use termination_signal::tokio::immediate_term_handle;

async fn app() -> anyhow::Result<()> {
    logger::init(log::LevelFilter::Info)?;

    info!("v {}", env!("CARGO_PKG_VERSION"));

    let monitor = MonitorClient::new_from_env().await?;

    let (mut rx, _) = monitor.subscribe_to_updates(1000, 5);

    loop {
        let msg = rx.recv().await;
        if let Err(e) = msg {
            error!("🚨 recv error | {e:#?}");
            break;
        }
        info!("{msg:#?}")
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let term_signal = immediate_term_handle()?;

    let app = tokio::spawn(app());

    tokio::select! {
        res = app => return res?,
        _ = term_signal => {},
    }

    Ok(())
}
