use std::sync::OnceLock;

use clap::Parser;
use merge_config_files::parse_config_file;
use monitor_client::MonitorClient;

pub fn cli_args() -> &'static crate::args::CliArgs {
  static CLI_ARGS: OnceLock<crate::args::CliArgs> = OnceLock::new();
  CLI_ARGS.get_or_init(crate::args::CliArgs::parse)
}

pub fn monitor_client() -> &'static MonitorClient {
  static MONITOR_CLIENT: OnceLock<MonitorClient> = OnceLock::new();
  MONITOR_CLIENT.get_or_init(|| {
    let args = cli_args();
    let crate::args::CredsFile { url, key, secret } =
      match (&args.url, &args.key, &args.secret) {
        (Some(url), Some(key), Some(secret)) => {
          crate::args::CredsFile {
            url: url.clone(),
            key: key.clone(),
            secret: secret.clone(),
          }
        }
        (url, key, secret) => {
          let mut creds: crate::args::CredsFile =
            parse_config_file(cli_args().creds.as_str())
              .expect("failed to parse monitor credentials");

          if let Some(url) = url {
            creds.url.clone_from(url);
          }
          if let Some(key) = key {
            creds.key.clone_from(key);
          }
          if let Some(secret) = secret {
            creds.secret.clone_from(secret);
          }

          creds
        }
      };
    futures::executor::block_on(MonitorClient::new(url, key, secret))
      .expect("failed to initialize monitor client")
  })
}
