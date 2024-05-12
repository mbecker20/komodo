use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  build::BuildState,
  deployment::{Deployment, DeploymentState},
  permission::PermissionLevel,
  repo::RepoState,
  server::{Server, ServerState},
  tag::Tag,
  update::ResourceTargetVariant,
  user::{admin_service_user, User},
};
use mungos::{
  by_id::find_one_by_id,
  find::find_collect,
  mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::FindOneOptions,
  },
};

use crate::{
  resource,
  state::{action_states, db_client},
};

#[instrument(level = "debug")]
pub async fn get_user(user_id: &str) -> anyhow::Result<User> {
  if let Some(user) = admin_service_user(user_id) {
    return Ok(user);
  }
  find_one_by_id(&db_client().await.users, user_id)
    .await
    .context("failed to query mongo for user")?
    .with_context(|| format!("no user found with id {user_id}"))
}

#[instrument(level = "debug")]
pub async fn get_server_with_status(
  server_id_or_name: &str,
) -> anyhow::Result<(Server, ServerState)> {
  let server = resource::get::<Server>(server_id_or_name).await?;
  if !server.config.enabled {
    return Ok((server, ServerState::Disabled));
  }
  let status = match super::periphery_client(&server)?
    .request(periphery_client::api::GetHealth {})
    .await
  {
    Ok(_) => ServerState::Ok,
    Err(_) => ServerState::NotOk,
  };
  Ok((server, status))
}

#[instrument(level = "debug")]
pub async fn get_deployment_state(
  deployment: &Deployment,
) -> anyhow::Result<DeploymentState> {
  if deployment.config.server_id.is_empty() {
    return Ok(DeploymentState::NotDeployed);
  }
  let (server, status) =
    get_server_with_status(&deployment.config.server_id).await?;
  if status != ServerState::Ok {
    return Ok(DeploymentState::Unknown);
  }
  let container = super::periphery_client(&server)?
    .request(periphery_client::api::container::GetContainerList {})
    .await?
    .into_iter()
    .find(|container| container.name == deployment.name);

  let state = match container {
    Some(container) => container.state,
    None => DeploymentState::NotDeployed,
  };

  Ok(state)
}

#[instrument(level = "debug")]
pub async fn get_tag(id_or_name: &str) -> anyhow::Result<Tag> {
  let query = match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
  };
  db_client()
    .await
    .tags
    .find_one(query, None)
    .await
    .context("failed to query mongo for tag")?
    .with_context(|| format!("no tag found matching {id_or_name}"))
}

#[instrument(level = "debug")]
pub async fn get_tag_check_owner(
  id_or_name: &str,
  user: &User,
) -> anyhow::Result<Tag> {
  let tag = get_tag(id_or_name).await?;
  if user.admin || tag.owner == user.id {
    return Ok(tag);
  }
  Err(anyhow!("user must be tag owner or admin"))
}

#[instrument(level = "debug")]
pub async fn get_user_user_group_ids(
  user_id: &str,
) -> anyhow::Result<Vec<String>> {
  let res = find_collect(
    &db_client().await.user_groups,
    doc! {
      "users": user_id
    },
    None,
  )
  .await
  .context("failed to query db for user groups")?
  .into_iter()
  .map(|ug| ug.id)
  .collect();
  Ok(res)
}

/// Returns Vec of all queries on permissions that match against the user
/// or any user groups that the user is a part of.
/// Result used with Mongodb '$or'.
#[instrument(level = "debug")]
pub async fn user_target_query(
  user_id: &str,
) -> anyhow::Result<Vec<Document>> {
  let mut user_target_query = vec![
    doc! { "user_target.type": "User", "user_target.id": user_id },
  ];
  let user_groups = get_user_user_group_ids(user_id)
    .await?
    .into_iter()
    .map(|ug_id| {
      doc! {
        "user_target.type": "UserGroup", "user_target.id": ug_id,
      }
    });
  user_target_query.extend(user_groups);
  Ok(user_target_query)
}

#[instrument(level = "debug")]
pub async fn get_user_permission_on_resource(
  user_id: &str,
  resource_variant: ResourceTargetVariant,
  resource_id: &str,
) -> anyhow::Result<PermissionLevel> {
  let permission = find_collect(
    &db_client().await.permissions,
    doc! {
      "$or": user_target_query(user_id).await?,
      "resource_target.type": resource_variant.as_ref(),
      "resource_target.id": resource_id
    },
    None,
  )
  .await
  .context("failed to query db for permissions")?
  .into_iter()
  // get the max permission user has between personal / any user groups
  .fold(PermissionLevel::None, |level, permission| {
    if permission.level > level {
      permission.level
    } else {
      level
    }
  });
  Ok(permission)
}

#[instrument(level = "debug")]
pub async fn get_resource_ids_for_non_admin(
  user_id: &str,
  resource_type: ResourceTargetVariant,
) -> anyhow::Result<Vec<String>> {
  let permissions = find_collect(
    &db_client().await.permissions,
    doc! {
      "$or": user_target_query(user_id).await?,
      "resource_target.type": resource_type.as_ref(),
      "level": { "$in": ["Read", "Execute", "Write"] }
    },
    None,
  )
  .await
  .context("failed to query permissions on db")?
  .into_iter()
  .map(|p| p.resource_target.extract_variant_id().1.to_string())
  // collect into hashset first to remove any duplicates
  .collect::<HashSet<_>>();
  Ok(permissions.into_iter().collect())
}

pub fn id_or_name_filter(id_or_name: &str) -> Document {
  match ObjectId::from_str(id_or_name) {
    Ok(id) => doc! { "_id": id },
    Err(_) => doc! { "name": id_or_name },
  }
}

pub async fn get_build_state(id: &String) -> BuildState {
  async {
    if action_states()
      .build
      .get(id)
      .await
      .map(|s| s.get().map(|s| s.building))
      .transpose()?
      .unwrap_or_default()
    {
      return Ok(BuildState::Building);
    }
    let status = db_client()
      .await
      .updates
      .find_one(
        doc! {
          "target.type": "Build",
          "target.id": id,
          "operation": "RunBuild"
        },
        FindOneOptions::builder()
          .sort(doc! { "start_ts": -1 })
          .build(),
      )
      .await?
      .map(|u| {
        if u.success {
          BuildState::Ok
        } else {
          BuildState::Failed
        }
      })
      .unwrap_or(BuildState::Ok);
    anyhow::Ok(status)
  }
  .await
  .inspect_err(|e| {
    warn!("failed to get build status for {id} | {e:#}")
  })
  .unwrap_or(BuildState::Unknown)
}

pub async fn get_repo_state(id: &String) -> RepoState {
  async {
    if let Some(status) = action_states()
      .repo
      .get(id)
      .await
      .map(|s| {
        s.get().map(|s| {
          if s.cloning {
            Some(RepoState::Cloning)
          } else if s.pulling {
            Some(RepoState::Pulling)
          } else {
            None
          }
        })
      })
      .transpose()?
      .flatten()
    {
      return Ok(status);
    }
    let status = db_client()
      .await
      .updates
      .find_one(
        doc! {
          "target.type": "Repo",
          "target.id": id,
          "$or": [
            { "operation": "CloneRepo" },
            { "operation": "PullRepo" },
          ],
        },
        FindOneOptions::builder()
          .sort(doc! { "start_ts": -1 })
          .build(),
      )
      .await?
      .map(|u| {
        if u.success {
          RepoState::Ok
        } else {
          RepoState::Failed
        }
      })
      .unwrap_or(RepoState::Ok);
    anyhow::Ok(status)
  }
  .await
  .inspect_err(|e| {
    warn!("failed to get repo status for {id} | {e:#}")
  })
  .unwrap_or(RepoState::Unknown)
}
