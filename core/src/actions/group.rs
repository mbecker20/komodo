use anyhow::{anyhow, Context};
use diff::Diff;
use helpers::to_monitor_name;
use types::{traits::Permissioned, Group, PermissionLevel, monitor_timestamp, Update, UpdateTarget, Operation, Log, UpdateStatus};

use crate::{auth::RequestUser, state::State};

impl State {
    pub async fn get_group_check_permissions(
        &self,
        deployment_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Group> {
        let group = self.db.get_group(deployment_id).await?;
        let permissions = group.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(group)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this deployment"
            ))
        }
    }

    pub async fn create_group(&self, name: &str, user: &RequestUser) -> anyhow::Result<Group> {
        let start_ts = monitor_timestamp();
        let group = Group {
            name: to_monitor_name(name),
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            created_at: start_ts.clone(),
            updated_at: start_ts.clone(),
            ..Default::default()
        };
        let group_id = self
            .db
            .groups
            .create_one(group)
            .await
            .context("failed to add group to db")?;
        let group = self.db.get_group(&group_id).await?;
        let update = Update {
            target: UpdateTarget::Group(group_id),
            operation: Operation::CreateGroup,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(group)
    }

    pub async fn create_full_group(
        &self,
        mut full_group: Group,
        user: &RequestUser,
    ) -> anyhow::Result<Group> {
        let group = self.create_group(&full_group.name, user).await?;
        full_group.id = group.id;
        let group = self.update_group(full_group, user).await?;
        Ok(group)
    }

    pub async fn delete_group(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Group> {
        let group = self
            .get_group_check_permissions(id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        self.db
            .groups
            .delete_one(id)
            .await
            .context(format!("failed at deleting group at {id} from mongo"))?;
        let update = Update {
            target: UpdateTarget::System,
            operation: Operation::DeleteGroup,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            logs: vec![Log::simple(
                "delete group",
                format!("deleted group {}", group.name),
            )],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(group)
    }

    pub async fn update_group(
        &self,
        mut new_group: Group,
        user: &RequestUser,
    ) -> anyhow::Result<Group> {
        let current_group = self
            .get_group_check_permissions(&new_group.id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();

        // none of these should be changed through this method
        new_group.name = current_group.name.clone();
        new_group.permissions = current_group.permissions.clone();
        new_group.created_at = current_group.created_at.clone();
        new_group.updated_at = start_ts.clone();

        self.db
            .groups
            .update_one(
                &new_group.id,
                mungos::Update::Regular(new_group.clone()),
            )
            .await
            .context("failed at update one group")?;

        let diff = current_group.diff(&new_group);

        let update = Update {
            operation: Operation::UpdateGroup,
            target: UpdateTarget::Group(new_group.id.clone()),
            end_ts: Some(start_ts.clone()),
            start_ts,
            status: UpdateStatus::Complete,
            logs: vec![Log::simple(
                "group update",
                serde_json::to_string_pretty(&diff).unwrap(),
            )],
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(new_group)
    }
}
