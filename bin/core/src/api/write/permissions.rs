use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::{
    UpdateUserPermissions, UpdateUserPermissionsOnTarget,
  },
  entities::{
    update::{ResourceTarget, Update},
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{find_one_by_id, update_one_by_id},
  mongodb::{
    bson::{doc, Document},
    options::UpdateOptions,
  },
};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    query::get_user,
    update::{add_update, make_update},
  },
  state::State,
};

#[async_trait]
impl Resolve<UpdateUserPermissions, User> for State {
  #[instrument(name = "UpdateUserPermissions", skip(self, admin))]
  async fn resolve(
    &self,
    UpdateUserPermissions {
      user_id,
      enabled,
      create_servers,
      create_builds,
    }: UpdateUserPermissions,
    admin: User,
  ) -> anyhow::Result<Update> {
    if !admin.admin {
      return Err(anyhow!("this method is admin only"));
    }
    let user = find_one_by_id(&db_client().await.users, &user_id)
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

    update_one_by_id(
      &db_client().await.users,
      &user_id,
      mungos::update::Update::Set(update_doc),
      None,
    )
    .await?;

    let mut update = make_update(
      ResourceTarget::System("system".to_string()),
      Operation::UpdateUserPermissions,
      &admin,
    );
    update.push_simple_log("modify user enabled", format!(
      "update permissions for {} ({})\nenabled: {enabled:?}\ncreate servers: {create_servers:?}\ncreate builds: {create_builds:?}", 
      user.username,
      user.id,
    ));
    update.finalize();
    update.id = add_update(update.clone()).await?;
    Ok(update)
  }
}

#[async_trait]
impl Resolve<UpdateUserPermissionsOnTarget, User> for State {
  #[instrument(
    name = "UpdateUserPermissionsOnTarget",
    skip(self, admin)
  )]
  async fn resolve(
    &self,
    UpdateUserPermissionsOnTarget {
      user_id,
      permission,
      target,
    }: UpdateUserPermissionsOnTarget,
    admin: User,
  ) -> anyhow::Result<Update> {
    if !admin.admin {
      return Err(anyhow!("this method is admin only"));
    }
    let user = get_user(&user_id).await?;
    if user.admin {
      return Err(anyhow!(
        "cannot use this method to update other admins permissions"
      ));
    }
    if !user.enabled {
      return Err(anyhow!("user not enabled"));
    }
    let (variant, id) = target.extract_variant_id();
    db_client().await.permissions.update_one(
      doc! { "user_id": &user.id, "target.type": variant.as_ref(), "target.id": id },
      doc! {
        "$set": {
          "user_id": &user.id,
          "target.type": variant.as_ref(),
          "target.id": id,
          "level": permission.as_ref(),
        }
      },
      UpdateOptions::builder().upsert(true).build()
    ).await?;
    let log_text = format!(
      "user {} given {} permissions on {target:?}",
      user.username, permission,
    );
    let mut update = make_update(
      target,
      Operation::UpdateUserPermissionsOnTarget,
      &admin,
    );
    update.push_simple_log("modify permissions", log_text);
    update.finalize();
    update.id = add_update(update.clone()).await?;
    Ok(update)
  }
}
