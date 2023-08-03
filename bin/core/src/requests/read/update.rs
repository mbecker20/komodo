use std::collections::HashMap;

use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        alerter::Alerter, build::Build, builder::Builder, deployment::Deployment, repo::Repo,
        server::Server,
    },
    requests::read::{ListUpdates, ListUpdatesResponse, UpdateListItem},
};
use mungos::mongodb::{bson::doc, options::FindOptions};
use resolver_api::Resolve;

use crate::{auth::RequestUser, resource::Resource, state::State};

const UPDATES_PER_PAGE: i64 = 20;

#[async_trait]
impl Resolve<ListUpdates, RequestUser> for State {
    async fn resolve(
        &self,
        ListUpdates { query, page }: ListUpdates,
        user: RequestUser,
    ) -> anyhow::Result<ListUpdatesResponse> {
        let query = if user.is_admin {
            query
        } else {
            let server_ids =
                <State as Resource<Server>>::get_resource_ids_for_non_admin(self, &user.id).await?;
            let deployment_ids =
                <State as Resource<Deployment>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
            let build_ids =
                <State as Resource<Build>>::get_resource_ids_for_non_admin(self, &user.id).await?;
            let repo_ids =
                <State as Resource<Repo>>::get_resource_ids_for_non_admin(self, &user.id).await?;
            let builder_ids =
                <State as Resource<Builder>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
            let alerter_ids =
                <State as Resource<Alerter>>::get_resource_ids_for_non_admin(self, &user.id)
                    .await?;
            let mut query = query.unwrap_or_default();
            query.extend(doc! {
                "$or": [
                   { "target.type": "Server", "target.id": { "$in": &server_ids } },
                   { "target.type": "Deployment", "target.id": { "$in": &deployment_ids } },
                   { "target.type": "Build", "target.id": { "$in": &build_ids } },
                   { "target.type": "Repo", "target.id": { "$in": &repo_ids } },
                   { "target.type": "Builder", "target.id": { "$in": &builder_ids } },
                   { "target.type": "Alerter", "target.id": { "$in": &alerter_ids } },
                ]
            });
            query.into()
        };

        let usernames = self
            .db
            .users
            .get_some(None, None)
            .await
            .context("failed to pull users from db")?
            .into_iter()
            .map(|u| (u.id, u.username))
            .collect::<HashMap<_, _>>();

        let updates = self
            .db
            .updates
            .get_some(
                query,
                FindOptions::builder()
                    .sort(doc! { "start_ts": -1 })
                    .skip(page as u64 * UPDATES_PER_PAGE as u64)
                    .limit(UPDATES_PER_PAGE)
                    .build(),
            )
            .await?
            .into_iter()
            .map(|u| UpdateListItem {
                id: u.id,
                operation: u.operation,
                start_ts: u.start_ts,
                success: u.success,
                username: usernames
                    .get(&u.operator)
                    .cloned()
                    .unwrap_or("unknown".to_string()),
                operator: u.operator,
                target: u.target,
                status: u.status,
                version: u.version,
            })
            .collect::<Vec<_>>();

        let next_page = if updates.len() == UPDATES_PER_PAGE as usize {
            Some(page + 1)
        } else {
            None
        };

        Ok(ListUpdatesResponse { updates, next_page })
    }
}
