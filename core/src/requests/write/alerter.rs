use async_trait::async_trait;
use monitor_types::{
    entities::alerter::Alerter,
    requests::write::{CopyAlerter, CreateAlerter, DeleteAlerter, UpdateAlerter},
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<CreateAlerter, RequestUser> for State {
    async fn resolve(
        &self,
        CreateAlerter { name, config }: CreateAlerter,
        user: RequestUser,
    ) -> anyhow::Result<Alerter> {
        todo!()
    }
}

#[async_trait]
impl Resolve<CopyAlerter, RequestUser> for State {
    async fn resolve(
        &self,
        CopyAlerter { name, id }: CopyAlerter,
        user: RequestUser,
    ) -> anyhow::Result<Alerter> {
        todo!()
    }
}

#[async_trait]
impl Resolve<DeleteAlerter, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteAlerter { id }: DeleteAlerter,
        user: RequestUser,
    ) -> anyhow::Result<Alerter> {
        todo!()
    }
}

#[async_trait]
impl Resolve<UpdateAlerter, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateAlerter { id, config }: UpdateAlerter,
        user: RequestUser,
    ) -> anyhow::Result<Alerter> {
        todo!()
    }
}
