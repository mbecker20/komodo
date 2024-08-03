use std::{collections::HashSet, time::Duration};

use anyhow::{anyhow, Context};
use mongo_indexed::Document;
use monitor_client::entities::{
  permission::{Permission, PermissionLevel, UserTarget},
  server::Server,
  update::{ResourceTarget, Update},
  user::User,
  EnvironmentVar,
};
use mungos::mongodb::bson::{doc, Bson};
use periphery_client::PeripheryClient;
use query::get_global_variables;
use rand::{thread_rng, Rng};

use crate::{config::core_config, state::db_client};

pub mod action_state;
pub mod alert;
pub mod cache;
pub mod channel;
pub mod procedure;
pub mod prune;
pub mod query;
pub mod stack;
pub mod sync;
pub mod update;

// pub mod resource;

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

pub fn git_token(
  provider_domain: &str,
  account_username: &str,
) -> Option<String> {
  core_config()
    .git_providers
    .iter()
    .find(|provider| provider.domain == provider_domain)
    .and_then(|provider| {
      provider
        .accounts
        .iter()
        .find(|account| account.username == account_username)
        .map(|account| account.token.clone())
    })
}

pub fn registry_token(
  provider_domain: &str,
  account_username: &str,
) -> Option<String> {
  core_config()
    .docker_registries
    .iter()
    .find(|provider| provider.domain == provider_domain)
    .and_then(|provider| {
      provider
        .accounts
        .iter()
        .find(|account| account.username == account_username)
        .map(|account| account.token.clone())
    })
}

#[instrument]
pub async fn remove_from_recently_viewed<T>(resource: T)
where
  T: Into<ResourceTarget> + std::fmt::Debug,
{
  let resource: ResourceTarget = resource.into();
  let (ty, id) = resource.extract_variant_id();
  if let Err(e) = db_client()
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
    )
    .await
    .context("failed to remove resource from users recently viewed")
  {
    warn!("{e:#}");
  }
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
    .insert_one(Permission {
      id: Default::default(),
      user_target: UserTarget::User(user.id.clone()),
      resource_target: target.clone(),
      level,
    })
    .await
  {
    error!("failed to create permission for {target:?} | {e:#}");
  };
}

/// Flattens a document only one level deep
///
/// eg `{ config: { label: "yes", thing: { field1: "ok", field2: "ok" } } }` ->
/// `{ "config.label": "yes", "config.thing": { field1: "ok", field2: "ok" } }`
pub fn flatten_document(doc: Document) -> Document {
  let mut target = Document::new();

  for (outer_field, bson) in doc {
    if let Bson::Document(doc) = bson {
      for (inner_field, bson) in doc {
        target.insert(format!("{outer_field}.{inner_field}"), bson);
      }
    } else {
      target.insert(outer_field, bson);
    }
  }

  target
}

/// Returns the secret replacers
pub async fn interpolate_variables_secrets_into_environment(
  environment: &mut Vec<EnvironmentVar>,
  update: &mut Update,
) -> anyhow::Result<HashSet<(String, String)>> {
  // Interpolate variables into environment
  let variables = get_global_variables().await?;
  let core_config = core_config();

  let mut global_replacers = HashSet::new();
  let mut secret_replacers = HashSet::new();

  for env in environment {
    // first pass - global variables
    let (res, more_replacers) = svi::interpolate_variables(
      &env.value,
      &variables,
      svi::Interpolator::DoubleBrackets,
      false,
    )
    .with_context(|| {
      format!(
        "failed to interpolate global variables - {}",
        env.variable
      )
    })?;
    global_replacers.extend(more_replacers);
    // second pass - core secrets
    let (res, more_replacers) = svi::interpolate_variables(
      &res,
      &core_config.secrets,
      svi::Interpolator::DoubleBrackets,
      false,
    )
    .context("failed to interpolate core secrets")?;
    secret_replacers.extend(more_replacers);

    // set env value with the result
    env.value = res;
  }

  // Show which variables were interpolated
  if !global_replacers.is_empty() {
    update.push_simple_log(
      "interpolate global variables",
      global_replacers
        .into_iter()
        .map(|(value, variable)| format!("<span class=\"text-muted-foreground\">{variable} =></span> {value}"))
        .collect::<Vec<_>>()
        .join("\n"),
    );
  }

  if !secret_replacers.is_empty() {
    update.push_simple_log(
      "interpolate core secrets",
      secret_replacers
        .iter()
        .map(|(_, variable)| format!("<span class=\"text-muted-foreground\">replaced:</span> {variable}"))
        .collect::<Vec<_>>()
        .join("\n"),
    );
  }

  Ok(secret_replacers)
}
