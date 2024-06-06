use monitor_client::entities::{monitor_timestamp, update::Log};
use run_command::{async_run_command, CommandOutput};

pub async fn run_monitor_command(
  stage: &str,
  command: String,
) -> Log {
  let start_ts = monitor_timestamp();
  let output = async_run_command(&command).await;
  output_into_log(stage, command, start_ts, output)
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
    end_ts: monitor_timestamp(),
  }
}
