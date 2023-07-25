use async_trait::async_trait;
use monitor_types::{entities::update::Update, requests::read::ListUpdates};
use mungos::mongodb::{bson::doc, options::FindOptions};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<ListUpdates, RequestUser> for State {
    async fn resolve(
        &self,
        ListUpdates { query }: ListUpdates,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Update>> {
        if user.is_admin {
            let updates = self
            .db
            .updates
            .get_some(
                query,
                FindOptions::builder().sort(doc! { "ts": -1 }).build(),
            )
            .await?;
             Ok(updates)
        } else {
            let build_ids = self.get_build_ids_for_non_admin(&user.id).await?;
            todo!()
        }
    }
}
