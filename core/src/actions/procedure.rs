use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use diff::Diff;
use helpers::to_monitor_name;
use types::{
    traits::Permissioned, Log, Operation, PermissionLevel, Procedure, ProcedureOperation::*,
    ProcedureStage, Update, UpdateStatus, UpdateTarget,
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
        let start_ts = unix_timestamp_ms() as i64;
        let procedure = Procedure {
            name: to_monitor_name(name),
            permissions: [(user.id.clone(), PermissionLevel::Write)]
                .into_iter()
                .collect(),
            created_at: start_ts,
            updated_at: start_ts,
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
            end_ts: Some(unix_timestamp_ms() as i64),
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
            .get_procedure_check_permissions(id, user, PermissionLevel::Write)
            .await?;
        let start_ts = unix_timestamp_ms() as i64;
        self.db
            .procedures
            .delete_one(id)
            .await
            .context(format!("failed at deleting procedure at {id} from mongo"))?;
        let update = Update {
            target: UpdateTarget::System,
            operation: Operation::DeleteProcedure,
            start_ts,
            end_ts: Some(unix_timestamp_ms() as i64),
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
            .get_procedure_check_permissions(&new_procedure.id, user, PermissionLevel::Write)
            .await?;
        let start_ts = unix_timestamp_ms() as i64;

        // none of these should be changed through this method
        new_procedure.name = current_procedure.name.clone();
        new_procedure.permissions = current_procedure.permissions.clone();
        new_procedure.created_at = current_procedure.created_at;
        new_procedure.updated_at = start_ts;

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
            end_ts: Some(start_ts),
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
            .get_procedure_check_permissions(id, user, PermissionLevel::Write)
            .await?;
        let mut updates = Vec::new();
        for ProcedureStage {
            operation,
            target_id,
        } in procedure.stages
        {
            match operation {
                StartContainer => {}
                StopContainer => {}
                RemoveContainer => {}
                DeployContainer => {}
                RecloneDeployment => {}

                BuildBuild => {}
                RecloneBuild => {}

                PruneImagesServer => {}
                _ => {}
            }
        }
        Ok(updates)
    }
}
