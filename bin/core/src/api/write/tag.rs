use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::{
    CreateTag, DeleteTag, UpdateTag, UpdateTagsOnResource,
    UpdateTagsOnResourceResponse,
  },
  entities::{
    alerter::Alerter, build::Build, builder::Builder,
    deployment::Deployment, procedure::Procedure, repo::Repo,
    server::Server, tag::CustomTag, update::ResourceTarget,
    user::User, PermissionLevel,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_bson},
};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{get_tag, get_tag_check_owner, resource::StateResource},
  state::State,
};

#[async_trait]
impl Resolve<CreateTag, User> for State {
  async fn resolve(
    &self,
    CreateTag {
      name,
      category,
      color,
    }: CreateTag,
    user: User,
  ) -> anyhow::Result<CustomTag> {
    let mut tag = CustomTag {
      id: Default::default(),
      name,
      category,
      color,
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
impl Resolve<UpdateTag, User> for State {
  async fn resolve(
    &self,
    UpdateTag { id, config }: UpdateTag,
    user: User,
  ) -> anyhow::Result<CustomTag> {
    get_tag_check_owner(&id, &user).await?;

    update_one_by_id(
      &db_client().await.tags,
      &id,
      doc! { "$set": to_bson(&config)? },
      None,
    )
    .await
    .context("context")?;
    get_tag(&id).await
  }
}

#[async_trait]
impl Resolve<DeleteTag, User> for State {
  async fn resolve(
    &self,
    DeleteTag { id }: DeleteTag,
    user: User,
  ) -> anyhow::Result<CustomTag> {
    let tag = get_tag_check_owner(&id, &user).await?;

    tokio::try_join!(
      <State as StateResource<Server>>::remove_tag_from_resources(
        self, &id,
      ),
      <State as StateResource<Deployment>>::remove_tag_from_resources(
        self, &id,
      ),
      <State as StateResource<Build>>::remove_tag_from_resources(
        self, &id,
      ),
      <State as StateResource<Repo>>::remove_tag_from_resources(
        self, &id,
      ),
      <State as StateResource<Builder>>::remove_tag_from_resources(
        self, &id,
      ),
      <State as StateResource<Alerter>>::remove_tag_from_resources(
        self, &id,
      ),
      <State as StateResource<Procedure>>::remove_tag_from_resources(
        self, &id,
      ),
    )?;

    delete_one_by_id(&db_client().await.tags, &id, None).await?;

    Ok(tag)
  }
}

#[async_trait]
impl Resolve<UpdateTagsOnResource, User> for State {
  async fn resolve(
    &self,
    UpdateTagsOnResource { target, tags }: UpdateTagsOnResource,
    user: User,
  ) -> anyhow::Result<UpdateTagsOnResourceResponse> {
    match target {
      ResourceTarget::System(_) => return Err(anyhow!("")),
      ResourceTarget::Build(id) => {
        <State as StateResource<Build>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Build>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?;
      }
      ResourceTarget::Builder(id) => {
        <State as StateResource<Builder>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Builder>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?
      }
      ResourceTarget::Deployment(id) => {
        <State as StateResource<Deployment>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Deployment>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?
      }
      ResourceTarget::Server(id) => {
        <State as StateResource<Server>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Server>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?
      }
      ResourceTarget::Repo(id) => {
        <State as StateResource<Repo>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Repo>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?
      }
      ResourceTarget::Alerter(id) => {
        <State as StateResource<Alerter>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Alerter>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?
      }
      ResourceTarget::Procedure(id) => {
        <State as StateResource<Procedure>>::get_resource_check_permissions(
          self, &id, &user, PermissionLevel::Update
        )
        .await?;
        <State as StateResource<Procedure>>::update_tags_on_resource(
          self, &id, tags,
        )
        .await?
      }
    };
    Ok(UpdateTagsOnResourceResponse {})
  }
}
