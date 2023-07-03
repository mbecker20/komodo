use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation,
    },
    monitor_timestamp,
    requests::api::{UpdateUserPermissions, UpdateUserPermissionsOnTarget},
};
use mungos::mongodb::bson::Document;
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<UpdateUserPermissions, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateUserPermissions {
            user_id,
            enabled,
            create_servers,
            create_builds,
        }: UpdateUserPermissions,
        admin: RequestUser,
    ) -> anyhow::Result<Update> {
        let start_ts = monitor_timestamp();
        if !admin.is_admin {
            return Err(anyhow!("this method is admin only"));
        }
        let user = self
            .db
            .users
            .find_one_by_id(&user_id)
            .await
            .context("failed to query mongo for user")?
            .context("did not find user with given id")?;
        if user.admin {
            return Err(anyhow!(
                "cannot use this method to update other admins permissions"
            ));
        }
        let mut update_doc = Document::new();
        if let Some(enabled) = enabled {
            update_doc.insert("enabled", enabled);
        }
        if let Some(create_servers) = create_servers {
            update_doc.insert("create_server_permissions", create_servers);
        }
        if let Some(create_builds) = create_builds {
            update_doc.insert("create_build_permissions", create_builds);
        }
        self.db
            .users
            .update_one::<Document>(&user_id, mungos::Update::Set(update_doc))
            .await?;
        let end_ts = monitor_timestamp();
        let mut update = Update {
            target: ResourceTarget::System,
            operation: Operation::UpdateUserPermissions,
            logs: vec![Log::simple(
                "modify user enabled",
                format!(
                    "update permissions for {} ({})\nenabled: {enabled:?}\ncreate servers: {create_servers:?}\ncreate builds: {create_builds:?}", 
                    user.username, 
                    user.id,

                ),
            )],
            start_ts,
            end_ts: end_ts.into(),
            status: UpdateStatus::Complete,
            success: true,
            operator: admin.id.clone(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;
        Ok(update)
    }
}

#[async_trait]
impl Resolve<UpdateUserPermissionsOnTarget, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateUserPermissionsOnTarget {
            user_id,
            permission,
            target,
        }: UpdateUserPermissionsOnTarget,
        admin: RequestUser,
    ) -> anyhow::Result<Update> {
        if !admin.is_admin {
            return Err(anyhow!("this method is admin only"));
        }
        todo!()
    }
}
