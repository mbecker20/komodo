use async_trait::async_trait;
use monitor_types::{
    entities::tag::CustomTag,
    requests::write::{CreateTag, DeleteTag, UpdateTag},
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
        todo!()
    }
}

#[async_trait]
impl Resolve<UpdateTag, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateTag { id, config }: UpdateTag,
        user: RequestUser,
    ) -> anyhow::Result<CustomTag> {
        todo!()
    }
}

#[async_trait]
impl Resolve<DeleteTag, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteTag { id }: DeleteTag,
        user: RequestUser,
    ) -> anyhow::Result<CustomTag> {
        todo!()
    }
}
