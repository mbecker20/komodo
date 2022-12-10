use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use diff::Diff;
use helpers::to_monitor_name;
use types::{
    traits::Permissioned, Deployment, Log, Operation, PermissionLevel, Update, UpdateStatus,
    UpdateTarget,
};

use crate::{
    auth::RequestUser,
    helpers::{all_logs_success, any_option_diff_is_some, option_diff_is_some},
    state::State,
};

impl State {
    pub async fn get_deployment_check_permissions(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Deployment> {
        let deployment = self.db.get_deployment(deployment_id).await?;
        let permissions = deployment.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(deployment)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this deployment"
            ))
        }
    }

    pub async fn create_deployment(
        &self,
        name: &str,
        server_id: String,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        self.get_server_check_permissions(&server_id, user, PermissionLevel::Write)
            .await?;
        let deployment = Deployment {
            name: to_monitor_name(name),
            server_id,
            permissions: [(user.id.clone(), PermissionLevel::Write)]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        let start_ts = unix_timestamp_ms() as i64;
        let deployment_id = self
            .db
            .deployments
            .create_one(deployment)
            .await
            .context("failed to add server to db")?;
        let deployment = self.db.get_deployment(&deployment_id).await?;
        let update = Update {
            target: UpdateTarget::Deployment(deployment_id),
            operation: Operation::CreateDeployment,
            start_ts,
            end_ts: Some(unix_timestamp_ms() as i64),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(deployment)
    }

    pub async fn delete_deployment(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Write)
            .await?;
        let start_ts = unix_timestamp_ms() as i64;
        let server = self.db.get_server(&deployment.server_id).await?;
        let log = self
            .periphery
            .container_remove(&server, &deployment.name)
            .await?;
        self.db.deployments.delete_one(deployment_id).await?;
        let update = Update {
            target: UpdateTarget::System,
            operation: Operation::DeleteDeployment,
            start_ts,
            end_ts: Some(unix_timestamp_ms() as i64),
            operator: user.id.clone(),
            logs: vec![
                log,
                Log::simple(format!(
                    "deleted deployment {} on server {}",
                    deployment.name, server.name
                )),
            ],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(deployment)
    }

    pub async fn update_deployment(
        &self,
        mut new_deployment: Deployment,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        let current_deployment = self
            .get_deployment_check_permissions(&new_deployment.id, user, PermissionLevel::Write)
            .await?;
        new_deployment.permissions = current_deployment.permissions.clone();

        self.db
            .deployments
            .update_one(
                &new_deployment.id,
                mungos::Update::Regular(new_deployment.clone()),
            )
            .await
            .context("failed at update one deployment")?;

        let diff = current_deployment.diff(&new_deployment);

        let mut update = Update {
            operation: Operation::UpdateDeployment,
            target: UpdateTarget::Deployment(new_deployment.id.clone()),
            start_ts: unix_timestamp_ms() as i64,
            status: UpdateStatus::InProgress,
            logs: vec![Log::simple(serde_json::to_string_pretty(&diff).unwrap())],
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        if any_option_diff_is_some(&[&diff.repo, &diff.branch, &diff.github_account])
            || option_diff_is_some(&diff.on_clone)
        {
            let server = self.db.get_server(&current_deployment.server_id).await?;
            match self.periphery.clone_repo(&server, &new_deployment).await {
                Ok(clone_logs) => {
                    update.logs.extend(clone_logs);
                }
                Err(e) => update
                    .logs
                    .push(Log::error("cloning repo", format!("{e:#?}"))),
            }
        }

        update.end_ts = Some(unix_timestamp_ms() as i64);
        update.success = all_logs_success(&update.logs);
        update.status = UpdateStatus::Complete;

        self.update_update(update).await?;

        Ok(new_deployment)
    }

    pub async fn deploy(&self, deployment_id: &str, user: &RequestUser) -> anyhow::Result<Update> {
        let mut deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Write)
            .await?;
        if let Some(build_id) = &deployment.build_id {
            let build = self.db.get_build(build_id).await?;
            let image = if let Some(docker_account) = &build.docker_account {
                if deployment.docker_run_args.docker_account.is_none() {
                    deployment.docker_run_args.docker_account = Some(docker_account.to_string())
                }
                format!("{docker_account}/{}", to_monitor_name(&build.name))
            } else {
                to_monitor_name(&build.name)
            };
            let version = if let Some(version) = &deployment.build_version {
                version.to_string()
            } else {
                "latest".to_string()
            };
            deployment.docker_run_args.image = format!("{image}:{version}");
        };
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::DeployDeployment,
            start_ts: unix_timestamp_ms() as i64,
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            // version: deployment.docker_run_args.,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        let deploy_log = self.periphery.deploy(&server, &deployment).await?;

        update.logs.push(deploy_log);
        update.status = UpdateStatus::Complete;
        update.end_ts = Some(unix_timestamp_ms() as i64);

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn reclone_deployment(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Write)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::RecloneDeployment,
            start_ts: unix_timestamp_ms() as i64,
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        update.success = match self.periphery.clone_repo(&server, &deployment).await {
            Ok(clone_logs) => {
                update.logs.extend(clone_logs);
                true
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("clone repo", format!("{e:#?}")));
                false
            }
        };

        update.status = UpdateStatus::Complete;
        update.end_ts = Some(unix_timestamp_ms() as i64);

        self.update_update(update.clone()).await?;

        Ok(update)
    }
}
