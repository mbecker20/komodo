use async_trait::async_trait;
use monitor_types::{entities::update::Update, requests::read::ListUpdates};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<ListUpdates, RequestUser> for State {
    async fn resolve(
        &self,
        ListUpdates { query }: ListUpdates,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Update>> {
		
        todo!()
    }
}
