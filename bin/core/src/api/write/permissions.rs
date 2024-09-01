use std::str::FromStr;

use anyhow::{anyhow, Context};
use komodo_client::{
  api::write::{
    UpdatePermissionOnResourceType,
    UpdatePermissionOnResourceTypeResponse, UpdatePermissionOnTarget,
    UpdatePermissionOnTargetResponse, UpdateUserBasePermissions,
    UpdateUserBasePermissionsResponse,
  },
  entities::{
    permission::{UserTarget, UserTargetVariant},
    user::User,
    ResourceTarget, ResourceTargetVariant,
  },
};
use mungos::{
  by_id::{find_one_by_id, update_one_by_id},
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::UpdateOptions,
  },
};
use resolver_api::Resolve;

use crate::{
  helpers::query::get_user,
  state::{db_client, State},
};

impl Resolve<UpdateUserBasePermissions, User> for State {
  #[instrument(name = "UpdateUserBasePermissions", skip(self, admin))]
  async fn resolve(
    &self,
    UpdateUserBasePermissions {
      user_id,
      enabled,
      create_servers,
      create_builds,
    }: UpdateUserBasePermissions,
    admin: User,
  ) -> anyhow::Result<UpdateUserBasePermissionsResponse> {
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

    Ok(UpdateUserBasePermissionsResponse {})
  }
}

impl Resolve<UpdatePermissionOnResourceType, User> for State {
  #[instrument(
    name = "UpdatePermissionOnResourceType",
    skip(self, admin)
  )]
  async fn resolve(
    &self,
    UpdatePermissionOnResourceType {
      user_target,
      resource_type,
      permission,
    }: UpdatePermissionOnResourceType,
    admin: User,
  ) -> anyhow::Result<UpdatePermissionOnResourceTypeResponse> {
    if !admin.admin {
      return Err(anyhow!("this method is admin only"));
    }

    // Some extra checks if user target is an actual User
    if let UserTarget::User(user_id) = &user_target {
      let user = get_user(user_id).await?;
      if user.admin {
        return Err(anyhow!(
          "cannot use this method to update other admins permissions"
        ));
      }
      if !user.enabled {
        return Err(anyhow!("user not enabled"));
      }
    }

    let (user_target_variant, user_target_id) =
      extract_user_target_with_validation(&user_target).await?;

    let id = ObjectId::from_str(&user_target_id)
      .context("id is not ObjectId")?;
    let field = format!("all.{resource_type}");
    let filter = doc! { "_id": id };
    let update = doc! { "$set": { &field: permission.as_ref() } };

    match user_target_variant {
      UserTargetVariant::User => {
        db_client()
          .await
          .users
          .update_one(filter, update)
          .await
          .with_context(|| {
            format!("failed to set {field}: {permission} on db")
          })?;
      }
      UserTargetVariant::UserGroup => {
        db_client()
          .await
          .user_groups
          .update_one(filter, update)
          .await
          .with_context(|| {
            format!("failed to set {field}: {permission} on db")
          })?;
      }
    }

    Ok(UpdatePermissionOnResourceTypeResponse {})
  }
}

impl Resolve<UpdatePermissionOnTarget, User> for State {
  #[instrument(name = "UpdatePermissionOnTarget", skip(self, admin))]
  async fn resolve(
    &self,
    UpdatePermissionOnTarget {
      user_target,
      resource_target,
      permission,
    }: UpdatePermissionOnTarget,
    admin: User,
  ) -> anyhow::Result<UpdatePermissionOnTargetResponse> {
    if !admin.admin {
      return Err(anyhow!("this method is admin only"));
    }

    // Some extra checks if user target is an actual User
    if let UserTarget::User(user_id) = &user_target {
      let user = get_user(user_id).await?;
      if user.admin {
        return Err(anyhow!(
          "cannot use this method to update other admins permissions"
        ));
      }
      if !user.enabled {
        return Err(anyhow!("user not enabled"));
      }
    }

    let (user_target_variant, user_target_id) =
      extract_user_target_with_validation(&user_target).await?;
    let (resource_variant, resource_id) =
      extract_resource_target_with_validation(&resource_target)
        .await?;

    let (user_target_variant, resource_variant) =
      (user_target_variant.as_ref(), resource_variant.as_ref());

    db_client()
      .await
      .permissions
      .update_one(
        doc! {
          "user_target.type": user_target_variant,
          "user_target.id": &user_target_id,
          "resource_target.type": resource_variant,
          "resource_target.id": &resource_id
        },
        doc! {
          "$set": {
            "user_target.type": user_target_variant,
            "user_target.id": user_target_id,
            "resource_target.type": resource_variant,
            "resource_target.id": resource_id,
            "level": permission.as_ref(),
          }
        },
      )
      .with_options(UpdateOptions::builder().upsert(true).build())
      .await?;

    Ok(UpdatePermissionOnTargetResponse {})
  }
}

/// checks if inner id is actually a `name`, and replaces it with id if so.
async fn extract_user_target_with_validation(
  user_target: &UserTarget,
) -> anyhow::Result<(UserTargetVariant, String)> {
  match user_target {
    UserTarget::User(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "username": ident },
      };
      let id = db_client()
        .await
        .users
        .find_one(filter)
        .await
        .context("failed to query db for users")?
        .context("no matching user found")?
        .id;
      Ok((UserTargetVariant::User, id))
    }
    UserTarget::UserGroup(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .user_groups
        .find_one(filter)
        .await
        .context("failed to query db for user_groups")?
        .context("no matching user_group found")?
        .id;
      Ok((UserTargetVariant::UserGroup, id))
    }
  }
}

/// checks if inner id is actually a `name`, and replaces it with id if so.
async fn extract_resource_target_with_validation(
  resource_target: &ResourceTarget,
) -> anyhow::Result<(ResourceTargetVariant, String)> {
  match resource_target {
    ResourceTarget::System(_) => {
      let res = resource_target.extract_variant_id();
      Ok((res.0, res.1.clone()))
    }
    ResourceTarget::Build(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .builds
        .find_one(filter)
        .await
        .context("failed to query db for builds")?
        .context("no matching build found")?
        .id;
      Ok((ResourceTargetVariant::Build, id))
    }
    ResourceTarget::Builder(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .builders
        .find_one(filter)
        .await
        .context("failed to query db for builders")?
        .context("no matching builder found")?
        .id;
      Ok((ResourceTargetVariant::Builder, id))
    }
    ResourceTarget::Deployment(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .deployments
        .find_one(filter)
        .await
        .context("failed to query db for deployments")?
        .context("no matching deployment found")?
        .id;
      Ok((ResourceTargetVariant::Deployment, id))
    }
    ResourceTarget::Server(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .servers
        .find_one(filter)
        .await
        .context("failed to query db for servers")?
        .context("no matching server found")?
        .id;
      Ok((ResourceTargetVariant::Server, id))
    }
    ResourceTarget::Repo(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .repos
        .find_one(filter)
        .await
        .context("failed to query db for repos")?
        .context("no matching repo found")?
        .id;
      Ok((ResourceTargetVariant::Repo, id))
    }
    ResourceTarget::Alerter(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .alerters
        .find_one(filter)
        .await
        .context("failed to query db for alerters")?
        .context("no matching alerter found")?
        .id;
      Ok((ResourceTargetVariant::Alerter, id))
    }
    ResourceTarget::Procedure(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .procedures
        .find_one(filter)
        .await
        .context("failed to query db for procedures")?
        .context("no matching procedure found")?
        .id;
      Ok((ResourceTargetVariant::Procedure, id))
    }
    ResourceTarget::ServerTemplate(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .server_templates
        .find_one(filter)
        .await
        .context("failed to query db for server templates")?
        .context("no matching server template found")?
        .id;
      Ok((ResourceTargetVariant::ServerTemplate, id))
    }
    ResourceTarget::ResourceSync(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .resource_syncs
        .find_one(filter)
        .await
        .context("failed to query db for resource syncs")?
        .context("no matching resource sync found")?
        .id;
      Ok((ResourceTargetVariant::ResourceSync, id))
    }
    ResourceTarget::Stack(ident) => {
      let filter = match ObjectId::from_str(ident) {
        Ok(id) => doc! { "_id": id },
        Err(_) => doc! { "name": ident },
      };
      let id = db_client()
        .await
        .stacks
        .find_one(filter)
        .await
        .context("failed to query db for stacks")?
        .context("no matching stack found")?
        .id;
      Ok((ResourceTargetVariant::Stack, id))
    }
  }
}
