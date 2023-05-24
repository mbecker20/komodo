use anyhow::Context;
use types::Update;

use crate::state::State;

mod build;
mod command;
mod deployment;
mod group;
mod procedure;
mod server;

impl State {
    pub async fn send_update(&self, update: Update) -> anyhow::Result<()> {
        self.update.sender.lock().await.send(update)?;
        Ok(())
    }

    pub async fn add_update(&self, mut update: Update) -> anyhow::Result<String> {
        update.id = self
            .db
            .updates
            .create_one(update.clone())
            .await
            .context("failed to insert update into db")?
            .to_string();
        let id = update.id.clone();
        let _ = self.send_update(update).await;
        Ok(id)
    }

    pub async fn update_update(&self, mut update: Update) -> anyhow::Result<()> {
        let mut update_id = String::new();
        std::mem::swap(&mut update.id, &mut update_id);
        self.db
            .updates
            .update_one(&update_id, mungos::Update::Regular(update.clone()))
            .await
            .context("failed to update the update on db. the update build process was deleted")?;
        std::mem::swap(&mut update.id, &mut update_id);
        let _ = self.send_update(update).await;
        Ok(())
    }
}
