use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation,
    },
    monitor_timestamp,
    requests::write::{
        UpdateUserPermissions, UpdateUserPermissionsOnTarget,
    },
};
use mungos::mongodb::bson::{doc, Document};
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
            update_doc
                .insert("create_server_permissions", create_servers);
        }
        if let Some(create_builds) = create_builds {
            update_doc
                .insert("create_build_permissions", create_builds);
        }
        self.db
            .users
            .update_one(&user_id, mungos::Update::Set(update_doc))
            .await?;
        let end_ts = monitor_timestamp();
        let mut update = Update {
            target: ResourceTarget::System("system".to_string()),
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
        if !user.enabled {
            return Err(anyhow!("user not enabled"));
        }
        let log_text = match &target {
            ResourceTarget::System(_) => {
                return Err(anyhow!("target can not be system"))
            }
            ResourceTarget::Build(id) => {
                let build = self
                    .db
                    .builds
                    .find_one_by_id(id)
                    .await
                    .context("failed at find build query")?
                    .ok_or(anyhow!(
                        "failed to find a build with id {id}"
                    ))?;
                self.db
                    .builds
                    .update_one(
                        id,
                        mungos::Update::Set(doc! {
                            format!("permissions.{}", user_id): permission.to_string()
                        }),
                    )
                    .await?;
                format!(
                    "user {} given {} permissions on build {}",
                    user.username, permission, build.name
                )
            }
            ResourceTarget::Builder(id) => {
                let builder = self
                    .db
                    .builders
                    .find_one_by_id(id)
                    .await
                    .context("failed at find builder query")?
                    .ok_or(anyhow!(
                        "failed to find a builder with id {id}"
                    ))?;
                self.db
                    .builders
                    .update_one(
                        id,
                        mungos::Update::Set(doc! {
                            format!("permissions.{}", user_id): permission.to_string()
                        }),
                    )
                    .await?;
                format!(
                    "user {} given {} permissions on builder {}",
                    user.username, permission, builder.name
                )
            }
            ResourceTarget::Deployment(id) => {
                let deployment = self
                    .db
                    .deployments
                    .find_one_by_id(id)
                    .await
                    .context("failed at find deployment query")?
                    .ok_or(anyhow!(
                        "failed to find a deployment with id {id}"
                    ))?;
                self.db
                    .deployments
                    .update_one(
                        id,
                        mungos::Update::Set(doc! {
                            format!("permissions.{}", user_id): permission.to_string()
                        }),
                    )
                    .await?;
                format!(
                    "user {} given {} permissions on deployment {}",
                    user.username, permission, deployment.name
                )
            }
            ResourceTarget::Server(id) => {
                let server = self
                    .db
                    .servers
                    .find_one_by_id(id)
                    .await
                    .context("failed at find server query")?
                    .ok_or(anyhow!(
                        "failed to find a server with id {id}"
                    ))?;
                self.db
                    .servers
                    .update_one(
                        id,
                        mungos::Update::Set(doc! {
                            format!("permissions.{}", user_id): permission.to_string()
                        }),
                    )
                    .await?;
                format!(
                    "user {} given {} permissions on server {}",
                    user.username, permission, server.name
                )
            }
            ResourceTarget::Repo(id) => {
                let repo = self
                    .db
                    .repos
                    .find_one_by_id(id)
                    .await
                    .context("failed at find repo query")?
                    .ok_or(anyhow!(
                        "failed to find a repo with id {id}"
                    ))?;
                self.db
                    .repos
                    .update_one(
                        id,
                        mungos::Update::Set(doc! {
                            format!("permissions.{}", user_id): permission.to_string()
                        }),
                    )
                    .await?;
                format!(
                    "user {} given {} permissions on repo {}",
                    user.username, permission, repo.name
                )
            }
            ResourceTarget::Alerter(id) => {
                let alerter = self
                    .db
                    .alerters
                    .find_one_by_id(id)
                    .await
                    .context("failed at find alerter query")?
                    .ok_or(anyhow!(
                        "failed to find a alerter with id {id}"
                    ))?;
                self.db
                    .alerters
                    .update_one(
                        id,
                        mungos::Update::Set(doc! {
                            format!("permissions.{}", user_id): permission.to_string()
                        }),
                    )
                    .await?;
                format!(
                    "user {} given {} permissions on alerter {}",
                    user.username, permission, alerter.name
                )
            }
        };
        let mut update = Update {
            operation: Operation::UpdateUserPermissionsOnTarget,
            start_ts,
            success: true,
            operator: admin.id.clone(),
            status: UpdateStatus::Complete,
            target: target.clone(),
            logs: vec![Log::simple("modify permissions", log_text)],
            end_ts: monitor_timestamp().into(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;
        Ok(update)
    }
}
