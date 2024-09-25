use std::{cmp, collections::HashSet};

use anyhow::{anyhow, Context};
use komodo_client::{
  api::read::*,
  entities::{
    deployment::{
      Deployment, DeploymentActionState, DeploymentConfig,
      DeploymentListItem, DeploymentState,
    },
    docker::container::ContainerStats,
    permission::PermissionLevel,
    server::Server,
    update::Log,
    user::User,
  },
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  helpers::periphery_client,
  resource,
  state::{action_states, deployment_status_cache, State},
};

impl Resolve<GetDeployment, User> for State {
  async fn resolve(
    &self,
    GetDeployment { deployment }: GetDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    resource::get_check_permissions::<Deployment>(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await
  }
}

impl Resolve<ListDeployments, User> for State {
  async fn resolve(
    &self,
    ListDeployments { query }: ListDeployments,
    user: User,
  ) -> anyhow::Result<Vec<DeploymentListItem>> {
    resource::list_for_user::<Deployment>(query, &user).await
  }
}

impl Resolve<ListFullDeployments, User> for State {
  async fn resolve(
    &self,
    ListFullDeployments { query }: ListFullDeployments,
    user: User,
  ) -> anyhow::Result<ListFullDeploymentsResponse> {
    resource::list_full_for_user::<Deployment>(query, &user).await
  }
}

impl Resolve<GetDeploymentContainer, User> for State {
  async fn resolve(
    &self,
    GetDeploymentContainer { deployment }: GetDeploymentContainer,
    user: User,
  ) -> anyhow::Result<GetDeploymentContainerResponse> {
    let deployment = resource::get_check_permissions::<Deployment>(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let status = deployment_status_cache()
      .get(&deployment.id)
      .await
      .unwrap_or_default();
    let response = GetDeploymentContainerResponse {
      state: status.curr.state,
      container: status.curr.container.clone(),
    };
    Ok(response)
  }
}

const MAX_LOG_LENGTH: u64 = 5000;

impl Resolve<GetDeploymentLog, User> for State {
  async fn resolve(
    &self,
    GetDeploymentLog { deployment, tail }: GetDeploymentLog,
    user: User,
  ) -> anyhow::Result<Log> {
    let Deployment {
      name,
      config: DeploymentConfig { server_id, .. },
      ..
    } = resource::get_check_permissions::<Deployment>(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    if server_id.is_empty() {
      return Ok(Log::default());
    }
    let server = resource::get::<Server>(&server_id).await?;
    periphery_client(&server)?
      .request(api::container::GetContainerLog {
        name,
        tail: cmp::min(tail, MAX_LOG_LENGTH),
      })
      .await
      .context("failed at call to periphery")
  }
}

impl Resolve<SearchDeploymentLog, User> for State {
  async fn resolve(
    &self,
    SearchDeploymentLog {
      deployment,
      terms,
      combinator,
      invert,
    }: SearchDeploymentLog,
    user: User,
  ) -> anyhow::Result<Log> {
    let Deployment {
      name,
      config: DeploymentConfig { server_id, .. },
      ..
    } = resource::get_check_permissions::<Deployment>(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    if server_id.is_empty() {
      return Ok(Log::default());
    }
    let server = resource::get::<Server>(&server_id).await?;
    periphery_client(&server)?
      .request(api::container::GetContainerLogSearch {
        name,
        terms,
        combinator,
        invert,
      })
      .await
      .context("failed at call to periphery")
  }
}

impl Resolve<GetDeploymentStats, User> for State {
  async fn resolve(
    &self,
    GetDeploymentStats { deployment }: GetDeploymentStats,
    user: User,
  ) -> anyhow::Result<ContainerStats> {
    let Deployment {
      name,
      config: DeploymentConfig { server_id, .. },
      ..
    } = resource::get_check_permissions::<Deployment>(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    if server_id.is_empty() {
      return Err(anyhow!("deployment has no server attached"));
    }
    let server = resource::get::<Server>(&server_id).await?;
    periphery_client(&server)?
      .request(api::container::GetContainerStats { name })
      .await
      .context("failed to get stats from periphery")
  }
}

impl Resolve<GetDeploymentActionState, User> for State {
  async fn resolve(
    &self,
    GetDeploymentActionState { deployment }: GetDeploymentActionState,
    user: User,
  ) -> anyhow::Result<DeploymentActionState> {
    let deployment = resource::get_check_permissions::<Deployment>(
      &deployment,
      &user,
      PermissionLevel::Read,
    )
    .await?;
    let action_state = action_states()
      .deployment
      .get(&deployment.id)
      .await
      .unwrap_or_default()
      .get()?;
    Ok(action_state)
  }
}

impl Resolve<GetDeploymentsSummary, User> for State {
  async fn resolve(
    &self,
    GetDeploymentsSummary {}: GetDeploymentsSummary,
    user: User,
  ) -> anyhow::Result<GetDeploymentsSummaryResponse> {
    let deployments = resource::list_full_for_user::<Deployment>(
      Default::default(),
      &user,
    )
    .await
    .context("failed to get deployments from db")?;
    let mut res = GetDeploymentsSummaryResponse::default();
    let status_cache = deployment_status_cache();
    for deployment in deployments {
      res.total += 1;
      let status =
        status_cache.get(&deployment.id).await.unwrap_or_default();
      match status.curr.state {
        DeploymentState::Running => {
          res.running += 1;
        }
        DeploymentState::Exited | DeploymentState::Paused => {
          res.stopped += 1;
        }
        DeploymentState::NotDeployed => {
          res.not_deployed += 1;
        }
        DeploymentState::Unknown => {
          res.unknown += 1;
        }
        _ => {
          res.unhealthy += 1;
        }
      }
    }
    Ok(res)
  }
}

impl Resolve<ListCommonDeploymentExtraArgs, User> for State {
  async fn resolve(
    &self,
    ListCommonDeploymentExtraArgs { query }: ListCommonDeploymentExtraArgs,
    user: User,
  ) -> anyhow::Result<ListCommonDeploymentExtraArgsResponse> {
    let deployments =
      resource::list_full_for_user::<Deployment>(query, &user)
        .await
        .context("failed to get resources matching query")?;

    // first collect with guaranteed uniqueness
    let mut res = HashSet::<String>::new();

    for deployment in deployments {
      for extra_arg in deployment.config.extra_args {
        res.insert(extra_arg);
      }
    }

    let mut res = res.into_iter().collect::<Vec<_>>();
    res.sort();
    Ok(res)
  }
}
