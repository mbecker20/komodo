use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::tag::CustomTag,
    requests::write::{CreateTag, DeleteTag, UpdateTag},
};
use mungos::mongodb::bson::{doc, to_bson};
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
            .create_one(&tag)
            .await
            .context("failed to create tag on db")?;
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
        self.db
            .tags
            .update_one(
                &id,
                mungos::Update::Custom(doc! { "$set": to_bson(&config)? }),
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
        self.db.tags.delete_one(&id).await?;
        Ok(tag)
    }
}
