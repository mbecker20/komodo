use anyhow::Context;
use monitor_client::entities::{
  deployment::{Deployment, DeploymentState},
  docker::container::ContainerListItem,
  stack::{Stack, StackService, StackServiceNames},
};

use crate::{
  helpers::{
    query::get_stack_state_from_containers,
    stack::{
      compose_container_match_regex,
      services::extract_services_from_stack,
    },
  },
  state::{deployment_status_cache, stack_status_cache},
};

use super::{CachedDeploymentStatus, CachedStackStatus, History};

pub async fn update_deployment_cache(
  deployments: Vec<Deployment>,
  containers: &[ContainerListItem],
) {
  let deployment_status_cache = deployment_status_cache();
  for deployment in deployments {
    let container = containers
      .iter()
      .find(|container| container.name == deployment.name)
      .cloned();
    let prev = deployment_status_cache
      .get(&deployment.id)
      .await
      .map(|s| s.curr.state);
    let state = container
      .as_ref()
      .map(|c| c.state.into())
      .unwrap_or(DeploymentState::NotDeployed);
    deployment_status_cache
      .insert(
        deployment.id.clone(),
        History {
          curr: CachedDeploymentStatus {
            id: deployment.id,
            state,
            container,
          },
          prev,
        }
        .into(),
      )
      .await;
  }
}

pub async fn update_stack_cache(
  stacks: Vec<Stack>,
  containers: &[ContainerListItem],
) {
  let stack_status_cache = stack_status_cache();
  for stack in stacks {
    let services = match extract_services_from_stack(&stack, false)
      .await
    {
      Ok(services) => services,
      Err(e) => {
        warn!("failed to extract services for stack {}. cannot match services to containers. (update status cache) | {e:?}", stack.name);
        continue;
      }
    };
    let mut services_with_containers = services.iter().map(|StackServiceNames { service_name, container_name }| {
			let container = containers.iter().find(|container| {
				match compose_container_match_regex(container_name)
					.with_context(|| format!("failed to construct container name matching regex for service {service_name}")) 
				{
					Ok(regex) => regex,
					Err(e) => {
						warn!("{e:#}");
						return false
					}
				}.is_match(&container.name)
			}).cloned();
			StackService {
				service: service_name.clone(),
				container,
			}
		}).collect::<Vec<_>>();
    services_with_containers
      .sort_by(|a, b| a.service.cmp(&b.service));
    let prev = stack_status_cache
      .get(&stack.id)
      .await
      .map(|s| s.curr.state);
    let status = CachedStackStatus {
      id: stack.id.clone(),
      state: get_stack_state_from_containers(
        &stack.config.ignore_services,
        &services,
        containers,
      ),
      services: services_with_containers,
    };
    stack_status_cache
      .insert(stack.id, History { curr: status, prev }.into())
      .await;
  }
}
