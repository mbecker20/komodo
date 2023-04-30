use anyhow::{anyhow, Context};
use diff::Diff;
use helpers::{all_logs_success, to_monitor_name};
use mungos::doc;
use types::{
    monitor_timestamp,
    traits::{Busy, Permissioned},
    Deployment, DeploymentWithContainerState, DockerContainerState, Log, Operation,
    PermissionLevel, ServerStatus, ServerWithStatus, TerminationSignal, Update, UpdateStatus,
    UpdateTarget,
};

use crate::{
    auth::RequestUser,
    helpers::{any_option_diff_is_some, empty_or_only_spaces, get_image_name, option_diff_is_some},
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

    pub async fn deployment_busy(&self, id: &str) -> bool {
        match self.deployment_action_states.lock().await.get(id) {
            Some(a) => a.busy(),
            None => false,
        }
    }

    pub async fn create_deployment(
        &self,
        name: &str,
        server_id: String,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        self.get_server_check_permissions(&server_id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        let deployment = Deployment {
            name: to_monitor_name(name),
            server_id,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            created_at: start_ts.clone(),
            updated_at: start_ts.clone(),
            ..Default::default()
        };
        let deployment_id = self
            .db
            .deployments
            .create_one(deployment)
            .await
            .context("failed to add deployment to db")?;
        let deployment = self.db.get_deployment(&deployment_id).await?;
        let update = Update {
            target: UpdateTarget::Deployment(deployment_id),
            operation: Operation::CreateDeployment,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(deployment)
    }

    pub async fn create_full_deployment(
        &self,
        mut deployment: Deployment,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        deployment.id = self
            .create_deployment(&deployment.name, deployment.server_id.clone(), user)
            .await?
            .id;
        let deployment = self.update_deployment(deployment, user).await?;
        Ok(deployment)
    }

    pub async fn copy_deployment(
        &self,
        target_id: &str,
        new_name: String,
        new_server_id: String,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        let mut deployment = self
            .get_deployment_check_permissions(target_id, user, PermissionLevel::Update)
            .await?;
        deployment.name = new_name;
        deployment.server_id = new_server_id;
        let deployment = self.create_full_deployment(deployment, user).await?;
        Ok(deployment)
    }

    pub async fn delete_deployment(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Deployment> {
        if self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        let server = self.db.get_server(&deployment.server_id).await?;
        let log = match self
            .periphery
            .container_remove(&server, &deployment.name, stop_signal, stop_time)
            .await
        {
            Ok(log) => log,
            Err(e) => Log::error("destroy container", format!("{e:#?}")),
        };
        self.db
            .deployments
            .delete_one(deployment_id)
            .await
            .context(format!(
                "failed at deleting deployment at {deployment_id} from mongo"
            ))?;
        let update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::DeleteDeployment,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            logs: vec![
                log,
                Log::simple(
                    "delete deployment",
                    format!(
                        "deleted deployment {} on server {}",
                        deployment.name, server.name
                    ),
                ),
            ],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(deployment)
    }

    pub async fn update_deployment(
        &self,
        new_deployment: Deployment,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        if self.deployment_busy(&new_deployment.id).await {
            return Err(anyhow!("deployment busy"));
        }
        let id = new_deployment.id.clone();
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(id.clone()).or_default();
            entry.updating = true;
        }
        let res = self.update_deployment_inner(new_deployment, user).await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(id).or_default();
            entry.updating = false;
        }
        res
    }

    async fn update_deployment_inner(
        &self,
        mut new_deployment: Deployment,
        user: &RequestUser,
    ) -> anyhow::Result<Deployment> {
        let current_deployment = self
            .get_deployment_check_permissions(&new_deployment.id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();

        // none of these should be changed through this method
        new_deployment.name = current_deployment.name.clone();
        new_deployment.permissions = current_deployment.permissions.clone();
        new_deployment.server_id = current_deployment.server_id.clone();
        new_deployment.created_at = current_deployment.created_at.clone();
        new_deployment.updated_at = start_ts.clone();

        // filter out any volumes, ports, env vars, extra args which are or contain empty strings
        // these could only happen by accident
        new_deployment.docker_run_args.volumes = new_deployment
            .docker_run_args
            .volumes
            .into_iter()
            .filter(|v| !empty_or_only_spaces(&v.local) && !empty_or_only_spaces(&v.container))
            .collect();
        new_deployment.docker_run_args.ports = new_deployment
            .docker_run_args
            .ports
            .into_iter()
            .filter(|p| !empty_or_only_spaces(&p.local) && !empty_or_only_spaces(&p.container))
            .collect();
        new_deployment.docker_run_args.environment = new_deployment
            .docker_run_args
            .environment
            .into_iter()
            .filter(|e| !empty_or_only_spaces(&e.variable) && !empty_or_only_spaces(&e.value))
            .collect();
        new_deployment.docker_run_args.extra_args = new_deployment
            .docker_run_args
            .extra_args
            .into_iter()
            .filter(|a| a.len() != 0)
            .collect();

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
            start_ts,
            status: UpdateStatus::InProgress,
            logs: vec![Log::simple(
                "deployment update",
                serde_json::to_string_pretty(&diff).unwrap(),
            )],
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

        update.end_ts = Some(monitor_timestamp());
        update.success = all_logs_success(&update.logs);
        update.status = UpdateStatus::Complete;

        self.update_update(update).await?;

        Ok(new_deployment)
    }

    pub async fn rename_deployment(
        &self,
        deployment_id: &str,
        new_name: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        if self.deployment_busy(&deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.renaming = true;
        }
        let res = self
            .rename_deployment_inner(deployment_id, new_name, user)
            .await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.renaming = false;
        }
        res
    }

    async fn rename_deployment_inner(
        &self,
        deployment_id: &str,
        new_name: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Update)
            .await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::RenameDeployment,
            start_ts,
            status: UpdateStatus::InProgress,
            operator: user.id.to_string(),
            success: true,
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;
        let server_with_status = self.get_server(&deployment.server_id, user).await;
        if server_with_status.is_err() {
            update.logs.push(Log::error(
                "get server",
                format!(
                    "failed to get server info: {:?}",
                    server_with_status.as_ref().err().unwrap()
                ),
            ));
            update.status = UpdateStatus::Complete;
            update.end_ts = monitor_timestamp().into();
            update.success = false;
            self.update_update(update).await?;
            return Err(server_with_status.err().unwrap());
        }
        let ServerWithStatus { server, status } = server_with_status.unwrap();
        if status != ServerStatus::Ok {
            update.logs.push(Log::error(
                "check server status",
                String::from("cannot rename deployment when periphery is disabled or unreachable"),
            ));
            update.status = UpdateStatus::Complete;
            update.end_ts = monitor_timestamp().into();
            update.success = false;
            self.update_update(update).await?;
            return Err(anyhow!(
                "cannot rename deployment when periphery is disabled or unreachable"
            ));
        }
        let deployment_state = self
            .get_deployment_with_container_state(user, deployment_id)
            .await;
        if deployment_state.is_err() {
            update.logs.push(Log::error(
                "check deployment status",
                format!(
                    "could not get current state of deployment: {:?}",
                    deployment_state.as_ref().err().unwrap()
                ),
            ));
            update.status = UpdateStatus::Complete;
            update.end_ts = monitor_timestamp().into();
            update.success = false;
            self.update_update(update).await?;
            return Err(deployment_state.err().unwrap());
        }
        let DeploymentWithContainerState {
            deployment, state, ..
        } = deployment_state.unwrap();
        if state != DockerContainerState::NotDeployed {
            let log = self
                .periphery
                .container_rename(&server, &deployment.name, new_name)
                .await;
            if log.is_err() {
                update.logs.push(Log::error(
                    "rename container",
                    format!("{:?}", log.as_ref().err().unwrap()),
                ));
                update.status = UpdateStatus::Complete;
                update.end_ts = monitor_timestamp().into();
                update.success = false;
                self.update_update(update).await?;
                return Err(log.err().unwrap());
            }
            let log = log.unwrap();
            if !log.success {
                update.logs.push(log);
                update.status = UpdateStatus::Complete;
                update.end_ts = monitor_timestamp().into();
                update.success = false;
                self.update_update(update).await?;
                return Err(anyhow!("rename container on periphery not successful"));
            }
            update.logs.push(log);
        }
        let res = self
            .db
            .deployments
            .update_one(
                deployment_id,
                mungos::Update::<()>::Set(
                    doc! { "name": to_monitor_name(new_name), "updated_at": monitor_timestamp() },
                ),
            )
            .await
            .context("failed to update deployment name on mongo");
        if let Err(e) = res {
            update
                .logs
                .push(Log::error("mongo update", format!("{e:?}")));
        } else {
            update.logs.push(Log::simple(
                "mongo update",
                String::from("updated name on mongo"),
            ))
        }

        if deployment.repo.is_some() {
            let res = self.reclone_deployment(deployment_id, user, false).await;
            if let Err(e) = res {
                update
                    .logs
                    .push(Log::error("reclone repo", format!("{e:?}")));
            } else {
                update.logs.push(Log::simple(
                    "reclone repo",
                    "deployment repo cloned with new name".to_string(),
                ));
            }
        }

        update.end_ts = monitor_timestamp().into();
        update.status = UpdateStatus::Complete;
        update.success = all_logs_success(&update.logs);

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn reclone_deployment(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        check_deployment_busy: bool,
    ) -> anyhow::Result<Update> {
        if check_deployment_busy && self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.recloning = true;
        }
        let res = self.reclone_deployment_inner(deployment_id, user).await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.recloning = false;
        }
        res
    }

    async fn reclone_deployment_inner(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Execute)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::RecloneDeployment,
            start_ts: monitor_timestamp(),
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
        update.end_ts = Some(monitor_timestamp());

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn deploy_container(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Update> {
        if self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.deploying = true;
        }
        let res = self
            .deploy_container_inner(deployment_id, user, stop_signal, stop_time)
            .await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.deploying = false;
        }
        res
    }

    async fn deploy_container_inner(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let mut deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Execute)
            .await?;
        let version = if let Some(build_id) = &deployment.build_id {
            let build = self.db.get_build(build_id).await?;
            let image = get_image_name(&build);
            if deployment.docker_run_args.docker_account.is_none() {
                if let Some(docker_account) = &build.docker_account {
                    deployment.docker_run_args.docker_account = Some(docker_account.to_string())
                };
            }
            let version = if let Some(version) = &deployment.build_version {
                version.clone()
            } else {
                build.version.clone()
            };
            deployment.docker_run_args.image = format!("{image}:{}", version.to_string());
            Some(version)
        } else {
            None
        };
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::DeployContainer,
            start_ts,
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            version,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        let deploy_log = match self
            .periphery
            .deploy(&server, &deployment, stop_signal, stop_time)
            .await
        {
            Ok(log) => log,
            Err(e) => Log::error("deploy container", format!("{e:#?}")),
        };

        update.success = deploy_log.success;
        update.logs.push(deploy_log);
        update.status = UpdateStatus::Complete;
        update.end_ts = Some(monitor_timestamp());

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn start_container(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        if self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.starting = true;
        }
        let res = self.start_container_inner(deployment_id, user).await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.starting = false;
        }
        res
    }

    async fn start_container_inner(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Execute)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::StartContainer,
            start_ts,
            status: UpdateStatus::InProgress,
            success: true,
            operator: user.id.clone(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        let log = self
            .periphery
            .container_start(&server, &deployment.name)
            .await;

        update.success = match log {
            Ok(log) => {
                let success = log.success;
                update.logs.push(log);
                success
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("start container", format!("{e:#?}")));
                false
            }
        };

        update.end_ts = Some(monitor_timestamp());
        update.status = UpdateStatus::Complete;

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn stop_container(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Update> {
        if self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.stopping = true;
        }
        let res = self
            .stop_container_inner(deployment_id, user, stop_signal, stop_time)
            .await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.stopping = false;
        }
        res
    }

    async fn stop_container_inner(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Execute)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::StopContainer,
            start_ts,
            status: UpdateStatus::InProgress,
            success: true,
            operator: user.id.clone(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        let log = self
            .periphery
            .container_stop(&server, &deployment.name, stop_signal, stop_time)
            .await;

        update.success = match log {
            Ok(log) => {
                let success = log.success;
                update.logs.push(log);
                success
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("stop container", format!("{e:#?}")));
                false
            }
        };

        update.end_ts = Some(monitor_timestamp());
        update.status = UpdateStatus::Complete;

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn remove_container(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Update> {
        if self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.removing = true;
        }
        let res = self
            .remove_container_inner(deployment_id, user, stop_signal, stop_time)
            .await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.removing = false;
        }
        res
    }

    async fn remove_container_inner(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        stop_signal: Option<TerminationSignal>,
        stop_time: Option<i32>,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Execute)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::RemoveContainer,
            start_ts,
            status: UpdateStatus::InProgress,
            success: true,
            operator: user.id.clone(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        let log = self
            .periphery
            .container_remove(&server, &deployment.name, stop_signal, stop_time)
            .await;

        update.success = match log {
            Ok(log) => {
                let success = log.success;
                update.logs.push(log);
                success
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("remove container", format!("{e:#?}")));
                false
            }
        };

        update.end_ts = Some(monitor_timestamp());
        update.status = UpdateStatus::Complete;

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    pub async fn pull_deployment_repo(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        if self.deployment_busy(deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.pulling = true;
        }
        let res = self.pull_deployment_repo_inner(deployment_id, user).await;
        {
            let mut lock = self.deployment_action_states.lock().await;
            let entry = lock.entry(deployment_id.to_string()).or_default();
            entry.pulling = false;
        }
        res
    }

    async fn pull_deployment_repo_inner(
        &self,
        deployment_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let deployment = self
            .get_deployment_check_permissions(deployment_id, user, PermissionLevel::Execute)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let mut update = Update {
            target: UpdateTarget::Deployment(deployment_id.to_string()),
            operation: Operation::PullDeployment,
            start_ts,
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        let logs = self
            .periphery
            .pull_repo(
                &server,
                &deployment.name,
                &deployment.branch,
                &deployment.on_pull,
            )
            .await?;

        update.success = all_logs_success(&logs);
        update.logs.extend(logs);
        update.end_ts = Some(monitor_timestamp());
        update.status = UpdateStatus::Complete;

        self.update_update(update.clone()).await?;

        Ok(update)
    }
}
