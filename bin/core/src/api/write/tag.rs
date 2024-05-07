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
    procedure::Procedure, repo::Repo, server::Server,
    server_template::ServerTemplate, tag::Tag,
    update::ResourceTarget, user::User,
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
  state::{db_client, State},
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
      resource::remove_tag_from_all::<Server>(&id),
      resource::remove_tag_from_all::<Deployment>(&id),
      resource::remove_tag_from_all::<Build>(&id),
      resource::remove_tag_from_all::<Repo>(&id),
      resource::remove_tag_from_all::<Builder>(&id),
      resource::remove_tag_from_all::<Alerter>(&id),
      resource::remove_tag_from_all::<Procedure>(&id),
      resource::remove_tag_from_all::<ServerTemplate>(&id),
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
        resource::get_check_permissions::<Build>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Build>(&id, tags, user).await?;
      }
      ResourceTarget::Builder(id) => {
        resource::get_check_permissions::<Builder>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Builder>(&id, tags, user).await?
      }
      ResourceTarget::Deployment(id) => {
        resource::get_check_permissions::<Deployment>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Deployment>(&id, tags, user).await?
      }
      ResourceTarget::Server(id) => {
        resource::get_check_permissions::<Server>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Server>(&id, tags, user).await?
      }
      ResourceTarget::Repo(id) => {
        resource::get_check_permissions::<Repo>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Repo>(&id, tags, user).await?
      }
      ResourceTarget::Alerter(id) => {
        resource::get_check_permissions::<Alerter>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Alerter>(&id, tags, user).await?
      }
      ResourceTarget::Procedure(id) => {
        resource::get_check_permissions::<Procedure>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<Procedure>(&id, tags, user).await?
      }
      ResourceTarget::ServerTemplate(id) => {
        resource::get_check_permissions::<ServerTemplate>(
          &id,
          &user,
          PermissionLevel::Write,
        )
        .await?;
        resource::update_tags::<ServerTemplate>(&id, tags, user)
          .await?
      }
    };
    Ok(UpdateTagsOnResourceResponse {})
  }
}
