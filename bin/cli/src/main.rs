#[macro_use]
extern crate tracing;

use colored::Colorize;
use komodo_client::api::read::GetVersion;

mod args;
mod exec;
mod helpers;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt().with_target(false).init();

  info!(
    "Komodo CLI version: {}",
    env!("CARGO_PKG_VERSION").blue().bold()
  );

  let version =
    state::komodo_client().read(GetVersion {}).await?.version;
  info!("Komodo Core version: {}", version.blue().bold());

  match &state::cli_args().command {
    args::Command::Execute { execution } => {
      exec::run(execution.to_owned()).await?
    }
  }

  Ok(())
}
