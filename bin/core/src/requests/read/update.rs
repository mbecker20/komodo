use async_trait::async_trait;
use monitor_types::{
    entities::{build::Build, update::Update},
    requests::read::ListUpdates,
};
use mungos::mongodb::{bson::doc, options::FindOptions};
use resolver_api::Resolve;

use crate::{auth::RequestUser, resource::Resource, state::State};

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
            let build_ids =
                <State as Resource<Build>>::get_resource_ids_for_non_admin(self, &user.id).await?;
            todo!()
        }
    }
}
