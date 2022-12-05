use anyhow::Context;
use db::DbClient;
use diff::{Diff, OptionDiff};
use types::{Log, Update};

use crate::ws::update;

#[macro_export]
macro_rules! response {
    ($x:expr) => {
        Ok::<_, (axum::http::StatusCode, String)>($x)
    };
}

pub fn option_diff_is_some<T: Diff>(diff: &OptionDiff<T>) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    diff != &OptionDiff::NoChange && diff != &OptionDiff::None
}

pub fn any_option_diff_is_some<T: Diff>(diffs: &[&OptionDiff<T>]) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    for diff in diffs {
        if diff != &&OptionDiff::NoChange && diff != &&OptionDiff::None {
            return true;
        }
    }
    return false;
}

pub fn all_logs_success(logs: &Vec<Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}

pub async fn add_update(
    mut update: Update,
    db: &DbClient,
    update_ws: &update::UpdateWsSender,
) -> anyhow::Result<String> {
    update.id = db
        .updates
        .create_one(update.clone())
        .await
        .context("failed to insert update into db")?
        .to_string();
    let id = update.id.clone();
    let _ = update_ws.lock().await.send(update);
    Ok(id)
}

pub async fn update_update(
    mut update: Update,
    db: &DbClient,
    update_ws: &update::UpdateWsSender,
) -> anyhow::Result<()> {
    let mut update_id = String::new();
    std::mem::swap(&mut update.id, &mut update_id);
    db.updates
        .update_one(&update_id, mungos::Update::Regular(update.clone()))
        .await
        .context("failed to update the update on db. the update build process was deleted")?;
    std::mem::swap(&mut update.id, &mut update_id);
    let _ = update_ws.lock().await.send(update);
    Ok(())
}
