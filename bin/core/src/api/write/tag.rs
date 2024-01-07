use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::write::{CreateTag, DeleteTag, UpdateTag},
  entities::tag::CustomTag,
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_bson},
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<CreateTag, RequestUser> for State {
  async fn resolve(
    &self,
    CreateTag {
      name,
      category,
      color,
    }: CreateTag,
    user: RequestUser,
  ) -> anyhow::Result<CustomTag> {
    let mut tag = CustomTag {
      id: Default::default(),
      name,
      category,
      color,
      owner: user.id.clone(),
    };
    tag.id = self
      .db
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
impl Resolve<UpdateTag, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateTag { id, config }: UpdateTag,
    user: RequestUser,
  ) -> anyhow::Result<CustomTag> {
    self.get_tag_check_owner(&id, &user).await?;

    update_one_by_id(
      &self.db.tags,
      &id,
      doc! { "$set": to_bson(&config)? },
      None,
    )
    .await
    .context("context")?;
    self.get_tag(&id).await
  }
}

#[async_trait]
impl Resolve<DeleteTag, RequestUser> for State {
  async fn resolve(
    &self,
    DeleteTag { id }: DeleteTag,
    user: RequestUser,
  ) -> anyhow::Result<CustomTag> {
    let tag = self.get_tag_check_owner(&id, &user).await?;
    delete_one_by_id(&self.db.tags, &id, None).await?;
    Ok(tag)
  }
}
