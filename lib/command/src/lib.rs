use std::path::Path;

use komodo_client::entities::{komodo_timestamp, update::Log};
use run_command::{async_run_command, CommandOutput};

/// Parses commands out of multiline string
/// and chains them together with '&&'
///
/// Supports full line and end of line comments. See [parse_multiline_command].
pub async fn run_komodo_command(
  stage: &str,
  path: impl Into<Option<&Path>>,
  command: impl AsRef<str>,
) -> Log {
  let command = parse_multiline_command(command);
  let command = if let Some(path) = path.into() {
    format!("cd {} && {command}", path.display(),)
  } else {
    command
  };
  let start_ts = komodo_timestamp();
  let output = async_run_command(&command).await;
  output_into_log(stage, command, start_ts, output)
}

/// Parses commands out of multiline string
/// and chains them together with '&&'
///
/// Supports full line and end of line comments.
///
/// ## Example:
/// ```sh
/// # comments supported
/// sh ./shell1.sh # end of line supported
/// sh ./shell2.sh
/// # print done
/// echo done
/// ```
/// becomes
/// ```sh
/// sh ./shell1.sh && sh ./shell2.sh && echo done
/// ```
pub fn parse_multiline_command(command: impl AsRef<str>) -> String {
  command
    .as_ref()
    .split('\n')
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .filter_map(|line| line.split(" #").next())
    .map(str::trim)
    .collect::<Vec<_>>()
    .join(" && ")
}

pub fn output_into_log(
  stage: &str,
  command: String,
  start_ts: i64,
  output: CommandOutput,
) -> Log {
  let success = output.success();
  Log {
    stage: stage.to_string(),
    stdout: output.stdout,
    stderr: output.stderr,
    command,
    success,
    start_ts,
    end_ts: komodo_timestamp(),
  }
}
