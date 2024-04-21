use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::{
    CreateTag, DeleteTag, RenameTag, UpdateTagsOnResource,
    UpdateTagsOnResourceResponse,
  },
  entities::{
    alerter::Alerter, build::Build, builder::Builder,
    deployment::Deployment, permission::PermissionLevel,
    procedure::Procedure, repo::Repo, server::Server, tag::Tag,
    update::ResourceTarget, user::User,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId},
};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    query::{get_tag, get_tag_check_owner},
    resource::StateResource,
  },
  state::State,
};

#[async_trait]
impl Resolve<CreateTag, User> for State {
  #[instrument(name = "CreateTag", skip(self, user))]
  async fn resolve(
    &self,
    CreateTag { name }: CreateTag,
    user: User,
  ) -> anyhow::Result<Tag> {
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("tag name cannot be ObjectId"));
    }

    let mut tag = Tag {
      id: Default::default(),
      name,
      owner: user.id.clone(),
    };

    tag.id = db_client()
      .await
      .tags
      .insert_one(&tag, None)
      .await
      .context("failed to create tag on db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();

    Ok(tag)
  }
}

#[async_trait]
impl Resolve<RenameTag, User> for State {
  async fn resolve(
    &self,
    RenameTag { id, name }: RenameTag,
    user: User,
  ) -> anyhow::Result<Tag> {
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("tag name cannot be ObjectId"));
    }

    get_tag_check_owner(&id, &user).await?;

    update_one_by_id(
      &db_client().await.tags,
      &id,
      doc! { "$set": { "name": name } },
      None,
    )
    .await
    .context("failed to rename tag on db")?;

    get_tag(&id).await
  }
}

#[async_trait]
impl Resolve<DeleteTag, User> for State {
  #[instrument(name = "DeleteTag", skip(self, user))]
  async fn resolve(
    &self,
    DeleteTag { id }: DeleteTag,
    user: User,
  ) -> anyhow::Result<Tag> {
    let tag = get_tag_check_owner(&id, &user).await?;

    tokio::try_join!(
      Server::remove_tag_from_resources(&id,),
      Deployment::remove_tag_from_resources(&id,),
      Build::remove_tag_from_resources(&id,),
      Repo::remove_tag_from_resources(&id,),
      Builder::remove_tag_from_resources(&id,),
      Alerter::remove_tag_from_resources(&id,),
      Procedure::remove_tag_from_resources(&id,),
    )?;

    delete_one_by_id(&db_client().await.tags, &id, None).await?;

    Ok(tag)
  }
}

#[async_trait]
impl Resolve<UpdateTagsOnResource, User> for State {
  #[instrument(name = "UpdateTagsOnResource", skip(self, user))]
  async fn resolve(
    &self,
    UpdateTagsOnResource { target, tags }: UpdateTagsOnResource,
    user: User,
  ) -> anyhow::Result<UpdateTagsOnResourceResponse> {
    match target {
      ResourceTarget::System(_) => return Err(anyhow!("")),
      ResourceTarget::Build(id) => {
        Build::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Build::update_tags_on_resource(&id, tags, user).await?;
      }
      ResourceTarget::Builder(id) => {
        Builder::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Builder::update_tags_on_resource(&id, tags, user).await?
      }
      ResourceTarget::Deployment(id) => {
        Deployment::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Deployment::update_tags_on_resource(&id, tags, user).await?
      }
      ResourceTarget::Server(id) => {
        Server::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Server::update_tags_on_resource(&id, tags, user).await?
      }
      ResourceTarget::Repo(id) => {
        Repo::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Repo::update_tags_on_resource(&id, tags, user).await?
      }
      ResourceTarget::Alerter(id) => {
        Alerter::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Alerter::update_tags_on_resource(&id, tags, user).await?
      }
      ResourceTarget::Procedure(id) => {
        Procedure::get_resource_check_permissions(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        Procedure::update_tags_on_resource(&id, tags, user).await?
      }
    };
    Ok(UpdateTagsOnResourceResponse {})
  }
}
