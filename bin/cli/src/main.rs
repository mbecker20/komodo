#[macro_use]
extern crate tracing;

use colored::Colorize;
use monitor_client::api::read::GetVersion;

mod args;
mod exec;
mod helpers;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt().with_target(false).init();

  let version =
    state::monitor_client().read(GetVersion {}).await?.version;
  info!("monitor version: {}", version.to_string().blue().bold());

  match &state::cli_args().command {
    args::Command::Execute { execution } => {
      exec::run(execution.to_owned()).await?
    }
  }

  Ok(())
}
