use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_helpers::{all_logs_success, monitor_timestamp};
use monitor_types::{
    entities::{
        deployment::Deployment,
        update::{Log, Update, UpdateStatus, UpdateTarget},
        Operation, PermissionLevel,
    },
    permissioned::Permissioned,
    requests::api::{
        CreateDeployment, DeleteDeployment, GetDeployment, ListDeployments, RenameDeployment,
        UpdateDeployment,
    },
};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeployment { id }: GetDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        self.get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListDeployments, RequestUser> for State {
    async fn resolve(
        &self,
        ListDeployments { query }: ListDeployments,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Deployment>> {
        let deployments = self
            .db
            .deployments
            .get_some(query, None)
            .await
            .context("failed to pull deployments from mongo")?;

        let deployments = if user.is_admin {
            deployments
        } else {
            deployments
                .into_iter()
                .filter(|deployment| {
                    deployment.get_user_permissions(&user.id) > PermissionLevel::None
                })
                .collect()
        };

        Ok(deployments)
    }
}

#[async_trait]
impl Resolve<CreateDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        CreateDeployment { name, config }: CreateDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        if let Some(server_id) = &config.server_id {
            self.get_server_check_permissions(server_id, &user, PermissionLevel::Update)
                .await
                .context("cannot create deployment on this server")?;
        }
        if let Some(build_id) = &config.build_id {
            self.get_build_check_permissions(build_id, &user, PermissionLevel::Read)
                .await
                .context("cannot create deployment with this build attached")?;
        }
        let start_ts = monitor_timestamp();
        let deployment = Deployment {
            id: Default::default(),
            name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            config: config.into(),
        };
        let deployment_id = self
            .db
            .deployments
            .create_one(&deployment)
            .await
            .context("failed to add deployment to db")?;
        let deployment = self.get_deployment(&deployment_id).await?;
        let update = Update {
            target: UpdateTarget::Deployment(deployment_id),
            operation: Operation::CreateDeployment,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create deployment",
                    format!(
                        "created deployment\nid: {}\nname: {}",
                        deployment.id, deployment.name
                    ),
                ),
                Log::simple("config", format!("{:#?}", deployment.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(deployment)
    }
}

#[async_trait]
impl Resolve<DeleteDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteDeployment { id }: DeleteDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        if self.action_states.deployment.busy(&id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let start_ts = monitor_timestamp();

        let mut update = Update {
            target: UpdateTarget::Deployment(id.clone()),
            operation: Operation::DeleteDeployment,
            start_ts,
            operator: user.id.clone(),
            success: true,
            status: UpdateStatus::InProgress,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        if !deployment.config.server_id.is_empty() {
            let server = self.get_server(&deployment.config.server_id).await;
            if let Err(e) = server {
                update.logs.push(Log::error(
                    "remove container",
                    format!(
                        "failed to retrieve server at {} from mongo | {e:#?}",
                        deployment.config.server_id
                    ),
                ));
            } else {
                let server = server.unwrap();
                match self
                    .periphery_client(&server)
                    .request(requests::RemoveContainer {
                        name: deployment.name.clone(),
                        signal: deployment.config.termination_signal.into(),
                        time: deployment.config.termination_timeout.into(),
                    })
                    .await
                {
                    Ok(log) => update.logs.push(log),
                    Err(e) => update.logs.push(Log::error(
                        "remove container",
                        format!("failed to remove container on periphery | {e:#?}"),
                    )),
                }
            }
        }

        let res = self
            .db
            .deployments
            .delete_one(&id)
            .await
            .context("failed to delete deployment from mongo");

        let log = match res {
            Ok(_) => Log::simple(
                "delete deployment",
                format!("deleted deployment {}", deployment.name),
            ),
            Err(e) => Log::error(
                "delete deployment",
                format!("failed to delete deployment\n{e:#?}"),
            ),
        };

        update.logs.push(log);

        update.end_ts = Some(monitor_timestamp());
        update.status = UpdateStatus::Complete;
        update.success = all_logs_success(&update.logs);

        self.update_update(update).await?;

        Ok(deployment)
    }
}

#[async_trait]
impl Resolve<UpdateDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateDeployment { id, config }: UpdateDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        todo!()
    }
}

#[async_trait]
impl Resolve<RenameDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        RenameDeployment { id, name }: RenameDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        todo!()
    }
}
