use std::str::FromStr;

use anyhow::{anyhow, Context};
use komodo_client::{
  api::write::{
    CreateTag, DeleteTag, RenameTag, UpdateTagsOnResource,
    UpdateTagsOnResourceResponse,
  },
  entities::{
    action::Action, alerter::Alerter, build::Build, builder::Builder,
    deployment::Deployment, permission::PermissionLevel,
    procedure::Procedure, repo::Repo, server::Server,
    server_template::ServerTemplate, stack::Stack,
    sync::ResourceSync, tag::Tag, ResourceTarget,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;

use crate::{
  helpers::query::{get_tag, get_tag_check_owner},
  resource,
  state::db_client,
};

use super::WriteArgs;

impl Resolve<WriteArgs> for CreateTag {
  #[instrument(name = "CreateTag", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<Tag> {
    if ObjectId::from_str(&self.name).is_ok() {
      return Err(anyhow!("tag name cannot be ObjectId").into());
    }

    let mut tag = Tag {
      id: Default::default(),
      name: self.name,
      owner: user.id.clone(),
    };

    tag.id = db_client()
      .tags
      .insert_one(&tag)
      .await
      .context("failed to create tag on db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();

    Ok(tag)
  }
}

impl Resolve<WriteArgs> for RenameTag {
  #[instrument(name = "RenameTag", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<Tag> {
    if ObjectId::from_str(&self.name).is_ok() {
      return Err(anyhow!("tag name cannot be ObjectId").into());
    }

    get_tag_check_owner(&self.id, &user).await?;

    update_one_by_id(
      &db_client().tags,
      &self.id,
      doc! { "$set": { "name": self.name } },
      None,
    )
    .await
    .context("failed to rename tag on db")?;

    Ok(get_tag(&self.id).await?)
  }
}

impl Resolve<WriteArgs> for DeleteTag {
  #[instrument(name = "DeleteTag", skip(user))]
  async fn resolve(
    self,
    WriteArgs { user }: &WriteArgs,
  ) -> serror::Result<Tag> {
    let tag = get_tag_check_owner(&self.id, &user).await?;

    tokio::try_join!(
      resource::remove_tag_from_all::<Server>(&self.id),
      resource::remove_tag_from_all::<Deployment>(&self.id),
      resource::remove_tag_from_all::<Stack>(&self.id),
      resource::remove_tag_from_all::<Build>(&self.id),
      resource::remove_tag_from_all::<Repo>(&self.id),
      resource::remove_tag_from_all::<Builder>(&self.id),
      resource::remove_tag_from_all::<Alerter>(&self.id),
      resource::remove_tag_from_all::<Procedure>(&self.id),
      resource::remove_tag_from_all::<ServerTemplate>(&self.id),
    )?;

    delete_one_by_id(&db_client().tags, &self.id, None).await?;

    Ok(tag)
  }
}

impl Resolve<WriteArgs> for UpdateTagsOnResource {
  #[instrument(name = "UpdateTagsOnResource", skip(args))]
  async fn resolve(
    self,
    args: &WriteArgs,
  ) -> serror::Result<UpdateTagsOnResourceResponse> {
    let WriteArgs { user } = args;
    match self.target {
      ResourceTarget::System(_) => {
        return Err(anyhow!("Invalid target type: System").into())
      }
      ResourceTarget::Build(id) => {
        resource::get_check_permissions::<Build>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Build>(&id, self.tags, args).await?;
      }
      ResourceTarget::Builder(id) => {
        resource::get_check_permissions::<Builder>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Builder>(&id, self.tags, args).await?
      }
      ResourceTarget::Deployment(id) => {
        resource::get_check_permissions::<Deployment>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Deployment>(&id, self.tags, args)
          .await?
      }
      ResourceTarget::Server(id) => {
        resource::get_check_permissions::<Server>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Server>(&id, self.tags, args).await?
      }
      ResourceTarget::Repo(id) => {
        resource::get_check_permissions::<Repo>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Repo>(&id, self.tags, args).await?
      }
      ResourceTarget::Alerter(id) => {
        resource::get_check_permissions::<Alerter>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Alerter>(&id, self.tags, args).await?
      }
      ResourceTarget::Procedure(id) => {
        resource::get_check_permissions::<Procedure>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Procedure>(&id, self.tags, args)
          .await?
      }
      ResourceTarget::Action(id) => {
        resource::get_check_permissions::<Action>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Action>(&id, self.tags, args).await?
      }
      ResourceTarget::ServerTemplate(id) => {
        resource::get_check_permissions::<ServerTemplate>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<ServerTemplate>(&id, self.tags, args)
          .await?
      }
      ResourceTarget::ResourceSync(id) => {
        resource::get_check_permissions::<ResourceSync>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<ResourceSync>(&id, self.tags, args)
          .await?
      }
      ResourceTarget::Stack(id) => {
        resource::get_check_permissions::<Stack>(
          &id,
          user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Stack>(&id, self.tags, args).await?
      }
    };
    Ok(UpdateTagsOnResourceResponse {})
  }
}
