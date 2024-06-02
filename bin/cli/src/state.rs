use std::sync::OnceLock;

use clap::Parser;
use monitor_client::MonitorClient;

pub fn cli_args() -> &'static crate::args::CliArgs {
  static CLI_ARGS: OnceLock<crate::args::CliArgs> = OnceLock::new();
  CLI_ARGS.get_or_init(crate::args::CliArgs::parse)
}

pub fn monitor_client() -> &'static MonitorClient {
  static MONITOR_CLIENT: OnceLock<MonitorClient> = OnceLock::new();
  MONITOR_CLIENT.get_or_init(|| {
    let crate::args::CredsFile { url, key, secret } =
      crate::helpers::parse_toml_file(&cli_args().creds)
        .expect("failed to parse monitor credentials");
    futures::executor::block_on(MonitorClient::new(url, key, secret))
      .expect("failed to initialize monitor client")
  })
}
