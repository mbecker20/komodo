use anyhow::{anyhow, Context};
use diff::Diff;
use helpers::all_logs_success;
use types::{
    monitor_timestamp, traits::Permissioned, Log, Operation, PeripheryCommand,
    PeripheryCommandBuilder, PermissionLevel, Update, UpdateStatus, UpdateTarget,
};

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_command_check_permissions(
        &self,
        command_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<PeripheryCommand> {
        let command = self.db.get_command(command_id).await?;
        let permissions = command.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(command)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this command"
            ))
        }
    }

    pub async fn create_command(
        &self,
        name: &str,
        server_id: String,
        user: &RequestUser,
    ) -> anyhow::Result<PeripheryCommand> {
        self.get_server_check_permissions(&server_id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        let command = PeripheryCommandBuilder::default()
            .name(name.to_string())
            .server_id(server_id)
            .build()
            .context("failed to build command")?;
        let command_id = self
            .db
            .commands
            .create_one(command)
            .await
            .context("failed at adding command to db")?;
        let command = self.db.get_command(&command_id).await?;
        let update = Update {
            target: UpdateTarget::Command(command_id),
            operation: Operation::CreateCommand,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(command)
    }

    pub async fn create_full_command(
        &self,
        mut command: PeripheryCommand,
        user: &RequestUser,
    ) -> anyhow::Result<PeripheryCommand> {
        command.id = self
            .create_command(&command.name, command.server_id.clone(), user)
            .await?
            .id;
        let command = self.update_command(command, user).await?;
        Ok(command)
    }

    pub async fn copy_command(
        &self,
        target_id: &str,
        new_name: String,
        new_server_id: String,
        user: &RequestUser,
    ) -> anyhow::Result<PeripheryCommand> {
        let mut command = self
            .get_command_check_permissions(target_id, user, PermissionLevel::Update)
            .await?;
        command.name = new_name;
        command.server_id = new_server_id;
        let command = self.create_full_command(command, user).await?;
        Ok(command)
    }

    pub async fn delete_command(
        &self,
        command_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<PeripheryCommand> {
        if self.command_action_states.busy(command_id).await {
            return Err(anyhow!("command busy"));
        }
        let command = self
            .get_command_check_permissions(command_id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        self.db.commands.delete_one(command_id).await?;
        let update = Update {
            target: UpdateTarget::Command(command_id.to_string()),
            operation: Operation::DeleteCommand,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            logs: vec![Log::simple(
                "delete command",
                format!("deleted command {}", command.name),
            )],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(command)
    }

    pub async fn update_command(
        &self,
        mut new_command: PeripheryCommand,
        user: &RequestUser,
    ) -> anyhow::Result<PeripheryCommand> {
        let current_command = self
            .get_command_check_permissions(&new_command.id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();

        // none of these should be changed through this method
        new_command.permissions = current_command.permissions.clone();
        new_command.server_id = current_command.server_id.clone();
        new_command.created_at = current_command.created_at.clone();
        new_command.updated_at = start_ts.clone();

        self.db
            .commands
            .update_one(
                &new_command.id,
                mungos::Update::Regular(new_command.clone()),
            )
            .await
            .context("failed at update one command")?;

        let diff = current_command.diff(&new_command);

        let update = Update {
            operation: Operation::UpdateCommand,
            target: UpdateTarget::Command(new_command.id.clone()),
            start_ts,
            status: UpdateStatus::Complete,
            logs: vec![Log::simple(
                "command update",
                serde_json::to_string_pretty(&diff).unwrap(),
            )],
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        self.add_update(update.clone()).await?;

        self.update_update(update).await?;

        Ok(new_command)
    }

    pub async fn run_command(
        &self,
        command_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        if self.command_action_states.busy(command_id).await {
            return Err(anyhow!("command busy"));
        }
        self.command_action_states
            .update_entry(command_id.to_string(), |entry| {
                entry.running = true;
            })
            .await;
        let res = self.run_command_inner(command_id, user).await;
        self.command_action_states
            .update_entry(command_id.to_string(), |entry| {
                entry.running = false;
            })
            .await;
        res
    }

    async fn run_command_inner(
        &self,
        command_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        let command = self
            .get_command_check_permissions(command_id, user, PermissionLevel::Execute)
            .await?;

        if command.command.path.is_empty() || command.command.command.is_empty() {
            return Err(anyhow!("command or path is empty, aborting"));
        }

        let server = self.db.get_server(&command.server_id).await?;

        let mut update = Update {
            target: UpdateTarget::Command(command_id.to_string()),
            operation: Operation::RunCommand,
            start_ts,
            status: UpdateStatus::InProgress,
            success: true,
            operator: user.id.clone(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        match self.periphery.run_command(&server, &command.command).await {
            Ok(log) => {
                update.logs.push(log);
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("clone repo", format!("{e:#?}")));
            }
        }

        update.success = all_logs_success(&update.logs);
        update.status = UpdateStatus::Complete;
        update.end_ts = Some(monitor_timestamp());

        self.update_update(update.clone()).await?;

        Ok(update)
    }
}
