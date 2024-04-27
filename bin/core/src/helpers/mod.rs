use std::time::Duration;

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  permission::{Permission, PermissionLevel, UserTarget},
  server::Server,
  update::ResourceTarget,
  user::User,
};
use mungos::mongodb::bson::doc;
use periphery_client::PeripheryClient;
use rand::{thread_rng, Rng};

use crate::{config::core_config, state::db_client};

pub mod alert;
pub mod cache;
pub mod channel;
pub mod procedure;
pub mod prune;
pub mod query;
pub mod resource;
pub mod update;
pub mod action_state;

pub fn empty_or_only_spaces(word: &str) -> bool {
  if word.is_empty() {
    return true;
  }
  for char in word.chars() {
    if char != ' ' {
      return false;
    }
  }
  true
}

pub fn random_duration(min_ms: u64, max_ms: u64) -> Duration {
  Duration::from_millis(thread_rng().gen_range(min_ms..max_ms))
}

#[instrument]
pub async fn remove_from_recently_viewed<T>(
  resource: T,
) -> anyhow::Result<()>
where
  T: Into<ResourceTarget> + std::fmt::Debug,
{
  let resource: ResourceTarget = resource.into();
  let (ty, id) = resource.extract_variant_id();
  db_client()
    .await
    .users
    .update_many(
      doc! {},
      doc! {
        "$pull": {
          "recently_viewed": {
            "type": ty.to_string(),
            "id": id,
          }
        }
      },
      None,
    )
    .await
    .context(
      "failed to remove resource from users recently viewed",
    )?;
  Ok(())
}

//

pub fn periphery_client(
  server: &Server,
) -> anyhow::Result<PeripheryClient> {
  if !server.config.enabled {
    return Err(anyhow!("server not enabled"));
  }

  let client = PeripheryClient::new(
    &server.config.address,
    &core_config().passkey,
  );

  Ok(client)
}

#[instrument]
pub async fn create_permission<T>(
  user: &User,
  target: T,
  level: PermissionLevel,
) where
  T: Into<ResourceTarget> + std::fmt::Debug,
{
  // No need to actually create permissions for admins
  if user.admin {
    return;
  }
  let target: ResourceTarget = target.into();
  if let Err(e) = db_client()
    .await
    .permissions
    .insert_one(
      Permission {
        id: Default::default(),
        user_target: UserTarget::User(user.id.clone()),
        resource_target: target.clone(),
        level,
      },
      None,
    )
    .await
  {
    error!("failed to create permission for {target:?} | {e:#}");
  };
}
