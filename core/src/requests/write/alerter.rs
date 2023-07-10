use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        alerter::Alerter,
        update::{Log, ResourceTarget, Update},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
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
        let start_ts = monitor_timestamp();
        let is_default = self.db.alerters.find_one(None, None).await?.is_none();
        let alerter = Alerter {
            id: Default::default(),
            name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            tags: Default::default(),
            is_default,
            config: config.into(),
        };
        let alerter_id = self
            .db
            .alerters
            .create_one(alerter)
            .await
            .context("failed to add alerter to db")?;
        let alerter = self.get_alerter(&alerter_id).await?;
        let update = Update {
            target: ResourceTarget::Alerter(alerter_id),
            operation: Operation::CreateAlerter,
            start_ts,
            end_ts: monitor_timestamp().into(),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create alerter",
                    format!(
                        "created alerter\nid: {}\nname: {}",
                        alerter.id, alerter.name
                    ),
                ),
                Log::simple("config", format!("{:#?}", alerter.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(alerter)
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
