use async_timing_util::unix_timestamp_ms;
use ::run_command::async_run_command;
use types::{Build, Deployment, Log};

pub async fn clone_build_repo(
    Build {
        name,
        repo,
        branch,
        github_account,
        on_clone,
        ..
    }: &Build,
) -> Log {
    todo!()
}

pub async fn clone_deployment_repo(Deployment { .. }: &Deployment) -> Log {
    todo!()
}

async fn clone(
    repo: &str,
    destination: &str,
    branch: Option<String>,
    access_token: Option<String>,
) -> Log {
    let _ = std::fs::remove_dir_all(destination);
    let access_token = match access_token {
        Some(token) => {
            format!("{token}@")
        }
        None => {
            format!("")
        }
    };
    let branch = branch.unwrap_or("main".to_string());
    let repo_url = format!("https://{access_token}github.com/{repo}.git");
    let command = format!("git clone {repo_url} {destination} -b {branch}");
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    let command = if access_token.len() > 0 {
        command.replace(&access_token, "<TOKEN>")
    } else {
        command
    };
    Log {
        stage: "clone repo".to_string(),
        command,
        success: output.success(),
        stdout: output.stdout,
        stderr: output.stderr,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
    }
}
