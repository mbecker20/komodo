use anyhow::{anyhow, Context};
use diff::Diff;
use helpers::to_monitor_name;
use types::{
    monitor_timestamp, traits::Permissioned, Log, Operation, PermissionLevel, Procedure,
    ProcedureOperation::*, ProcedureStage, Update, UpdateStatus, UpdateTarget,
};

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_procedure_check_permissions(
        &self,
        procedure_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Procedure> {
        let procedure = self.db.get_procedure(procedure_id).await?;
        let permissions = procedure.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(procedure)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this procedure"
            ))
        }
    }

    pub async fn create_procedure(
        &self,
        name: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        let start_ts = monitor_timestamp();
        let procedure = Procedure {
            name: to_monitor_name(name),
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            created_at: start_ts.clone(),
            updated_at: start_ts.clone(),
            ..Default::default()
        };
        let procedure_id = self
            .db
            .procedures
            .create_one(procedure)
            .await
            .context("failed to add procedure to db")?;
        let procedure = self.db.get_procedure(&procedure_id).await?;
        let update = Update {
            target: UpdateTarget::Procedure(procedure_id),
            operation: Operation::CreateProcedure,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(procedure)
    }

    pub async fn create_full_procedure(
        &self,
        mut full_procedure: Procedure,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        let procedure = self.create_procedure(&full_procedure.name, user).await?;
        full_procedure.id = procedure.id;
        let procedure = self.update_procedure(full_procedure, user).await?;
        Ok(procedure)
    }

    pub async fn delete_procedure(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        let procedure = self
            .get_procedure_check_permissions(id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        self.db
            .procedures
            .delete_one(id)
            .await
            .context(format!("failed at deleting procedure at {id} from mongo"))?;
        let update = Update {
            target: UpdateTarget::System,
            operation: Operation::DeleteProcedure,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            logs: vec![Log::simple(
                "delete deployment",
                format!("deleted procedure {}", procedure.name),
            )],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(procedure)
    }

    pub async fn update_procedure(
        &self,
        mut new_procedure: Procedure,
        user: &RequestUser,
    ) -> anyhow::Result<Procedure> {
        let current_procedure = self
            .get_procedure_check_permissions(&new_procedure.id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();

        // none of these should be changed through this method
        new_procedure.name = current_procedure.name.clone();
        new_procedure.permissions = current_procedure.permissions.clone();
        new_procedure.created_at = current_procedure.created_at.clone();
        new_procedure.updated_at = start_ts.clone();

        // check to make sure no stages have been added that user does not have access to

        self.db
            .procedures
            .update_one(
                &new_procedure.id,
                mungos::Update::Regular(new_procedure.clone()),
            )
            .await
            .context("failed at update one deployment")?;

        let diff = current_procedure.diff(&new_procedure);

        let update = Update {
            operation: Operation::UpdateProcedure,
            target: UpdateTarget::Procedure(new_procedure.id.clone()),
            end_ts: Some(start_ts.clone()),
            start_ts,
            status: UpdateStatus::Complete,
            logs: vec![Log::simple(
                "procedure update",
                serde_json::to_string_pretty(&diff).unwrap(),
            )],
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(new_procedure)
    }

    pub async fn run_procedure(&self, id: &str, user: &RequestUser) -> anyhow::Result<Vec<Update>> {
        let procedure = self
            .get_procedure_check_permissions(id, user, PermissionLevel::Execute)
            .await?;
        let mut updates = Vec::new();
        for ProcedureStage {
            operation,
            target_id,
        } in procedure.stages
        {
            match operation {
                None => {}
                // deployment
                StartContainer => {
                    let update = self
                        .start_container(&target_id, user)
                        .await
                        .context(format!(
                            "failed at start container for deployment (id: {target_id})"
                        ))?;
                    updates.push(update);
                }
                StopContainer => {
                    let update = self
                        .stop_container(&target_id, user)
                        .await
                        .context(format!(
                            "failed at stop container for deployment (id: {target_id})"
                        ))?;
                    updates.push(update);
                }
                RemoveContainer => {
                    let update = self
                        .remove_container(&target_id, user)
                        .await
                        .context(format!(
                            "failed at remove container for deployment (id: {target_id})"
                        ))?;
                    updates.push(update);
                }
                DeployContainer => {
                    let update = self
                        .deploy_container(&target_id, user)
                        .await
                        .context(format!(
                            "failed at deploy container for deployment (id: {target_id})"
                        ))?;
                    updates.push(update);
                }
                RecloneDeployment => {
                    let update = self
                        .reclone_deployment(&target_id, user)
                        .await
                        .context(format!("failed at reclone deployment (id: {target_id})"))?;
                    updates.push(update);
                }
                PullDeployment => {
                    // implement this one
                    // let update = self.pull
                }
                // build
                BuildBuild => {
                    let update = self
                        .build(&target_id, user)
                        .await
                        .context(format!("failed at build (id: {target_id})"))?;
                    updates.push(update);
                }
                RecloneBuild => {
                    let update = self
                        .reclone_build(&target_id, user)
                        .await
                        .context(format!("failed at reclone build (id: {target_id})"))?;
                    updates.push(update);
                }
                // server
                PruneImagesServer => {
                    let update = self.prune_images(&target_id, user).await.context(format!(
                        "failed at prune images on server (id: {target_id})"
                    ))?;
                    updates.push(update);
                }
                PruneContainersServer => {
                    let update = self
                        .prune_containers(&target_id, user)
                        .await
                        .context(format!(
                            "failed at prune containers on server (id: {target_id})"
                        ))?;
                    updates.push(update);
                }
                PruneNetworksServer => {
                    let update = self
                        .prune_networks(&target_id, user)
                        .await
                        .context(format!(
                            "failed at prune networks on servers (id: {target_id})"
                        ))?;
                    updates.push(update);
                }
                // procedure
                RunProcedure => {
                    // need to figure out async recursion
                    // need to guard against infinite procedure loops when they are updated
                    // let proc_updates = self
                    //     .run_procedure(&target_id, user)
                    //     .await
                    //     .context(format!("failed to run nested procedure (id: {target_id})"))?;
                    // updates.extend(proc_updates);
                }
            }
        }
        Ok(updates)
    }
}
