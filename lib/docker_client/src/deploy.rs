use run_command::{async_run_command, CommandOutput};
use types::{Deployment, Log};

use crate::DockerClient;

impl DockerClient {
    pub async fn deploy(&self, deployment: &Deployment) -> (bool, Log) {
        let docker_run = docker_run_command(deployment);
        let output = async_run_command(&docker_run).await;
        output_into_log("docker run", output)
    }

    pub async fn docker_start_command(&self, container_name: &str) -> (bool, Log) {
        let command = format!("start stop {container_name}");
        let output = async_run_command(&command).await;
        output_into_log("docker stop", output)
    }

    pub async fn docker_stop_command(&self, container_name: &str) -> (bool, Log) {
        let command = format!("docker stop {container_name}");
        let output = async_run_command(&command).await;
        output_into_log("docker stop", output)
    }

    pub async fn docker_stop_and_remove(&self, container_name: &str) -> (bool, Log) {
        let command =
            format!("docker stop {container_name} && docker container rm {container_name}");
        let output = async_run_command(&command).await;
        output_into_log("docker stop and remove", output)
    }
}

fn docker_run_command(deployment: &Deployment) -> String {
    todo!()
}

fn output_into_log(stage: &str, output: CommandOutput) -> (bool, Log) {
    let success = output.success();
    let log = Log {
        stage: stage.to_string(),
        stdout: output.stdout,
        stderr: output.stderr,
    };
    (success, log)
}
