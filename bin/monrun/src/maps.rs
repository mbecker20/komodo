use std::{collections::HashMap, sync::OnceLock};

use monitor_client::{
  api::read,
  entities::{
    alerter::AlerterListItem, build::BuildListItem,
    builder::BuilderListItem, deployment::DeploymentListItem,
    procedure::ProcedureListItem, repo::RepoListItem,
    server::ServerListItem, user::User, user_group::UserGroup,
  },
};

use crate::monitor_client;

pub fn name_to_build() -> &'static HashMap<String, BuildListItem> {
  static NAME_TO_BUILD: OnceLock<HashMap<String, BuildListItem>> =
    OnceLock::new();
  NAME_TO_BUILD.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListBuilds::default()),
    )
    .expect("failed to get builds from monitor")
    .into_iter()
    .map(|build| (build.name.clone(), build))
    .collect()
  })
}

pub fn id_to_build() -> &'static HashMap<String, BuildListItem> {
  static ID_TO_BUILD: OnceLock<HashMap<String, BuildListItem>> =
    OnceLock::new();
  ID_TO_BUILD.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListBuilds::default()),
    )
    .expect("failed to get builds from monitor")
    .into_iter()
    .map(|build| (build.id.clone(), build))
    .collect()
  })
}

pub fn name_to_deployment(
) -> &'static HashMap<String, DeploymentListItem> {
  static NAME_TO_DEPLOYMENT: OnceLock<
    HashMap<String, DeploymentListItem>,
  > = OnceLock::new();
  NAME_TO_DEPLOYMENT.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListDeployments::default()),
    )
    .expect("failed to get deployments from monitor")
    .into_iter()
    .map(|deployment| (deployment.name.clone(), deployment))
    .collect()
  })
}

pub fn id_to_deployment(
) -> &'static HashMap<String, DeploymentListItem> {
  static ID_TO_DEPLOYMENT: OnceLock<
    HashMap<String, DeploymentListItem>,
  > = OnceLock::new();
  ID_TO_DEPLOYMENT.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListDeployments::default()),
    )
    .expect("failed to get deployments from monitor")
    .into_iter()
    .map(|deployment| (deployment.id.clone(), deployment))
    .collect()
  })
}

pub fn name_to_server() -> &'static HashMap<String, ServerListItem> {
  static NAME_TO_SERVER: OnceLock<HashMap<String, ServerListItem>> =
    OnceLock::new();
  NAME_TO_SERVER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListServers::default()),
    )
    .expect("failed to get servers from monitor")
    .into_iter()
    .map(|server| (server.name.clone(), server))
    .collect()
  })
}

pub fn id_to_server() -> &'static HashMap<String, ServerListItem> {
  static ID_TO_SERVER: OnceLock<HashMap<String, ServerListItem>> =
    OnceLock::new();
  ID_TO_SERVER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListServers::default()),
    )
    .expect("failed to get servers from monitor")
    .into_iter()
    .map(|server| (server.id.clone(), server))
    .collect()
  })
}

pub fn name_to_builder() -> &'static HashMap<String, BuilderListItem>
{
  static NAME_TO_BUILDER: OnceLock<HashMap<String, BuilderListItem>> =
    OnceLock::new();
  NAME_TO_BUILDER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListBuilders::default()),
    )
    .expect("failed to get builders from monitor")
    .into_iter()
    .map(|builder| (builder.name.clone(), builder))
    .collect()
  })
}

pub fn id_to_builder() -> &'static HashMap<String, BuilderListItem> {
  static ID_TO_BUILDER: OnceLock<HashMap<String, BuilderListItem>> =
    OnceLock::new();
  ID_TO_BUILDER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListBuilders::default()),
    )
    .expect("failed to get builders from monitor")
    .into_iter()
    .map(|builder| (builder.id.clone(), builder))
    .collect()
  })
}

pub fn name_to_alerter() -> &'static HashMap<String, AlerterListItem>
{
  static NAME_TO_ALERTER: OnceLock<HashMap<String, AlerterListItem>> =
    OnceLock::new();
  NAME_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListAlerters::default()),
    )
    .expect("failed to get alerters from monitor")
    .into_iter()
    .map(|alerter| (alerter.name.clone(), alerter))
    .collect()
  })
}

pub fn id_to_alerter() -> &'static HashMap<String, AlerterListItem> {
  static ID_TO_ALERTER: OnceLock<HashMap<String, AlerterListItem>> =
    OnceLock::new();
  ID_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListAlerters::default()),
    )
    .expect("failed to get alerters from monitor")
    .into_iter()
    .map(|alerter| (alerter.id.clone(), alerter))
    .collect()
  })
}

pub fn name_to_repo() -> &'static HashMap<String, RepoListItem> {
  static NAME_TO_ALERTER: OnceLock<HashMap<String, RepoListItem>> =
    OnceLock::new();
  NAME_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListRepos::default()),
    )
    .expect("failed to get repos from monitor")
    .into_iter()
    .map(|repo| (repo.name.clone(), repo))
    .collect()
  })
}

pub fn id_to_repo() -> &'static HashMap<String, RepoListItem> {
  static ID_TO_ALERTER: OnceLock<HashMap<String, RepoListItem>> =
    OnceLock::new();
  ID_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListRepos::default()),
    )
    .expect("failed to get repos from monitor")
    .into_iter()
    .map(|repo| (repo.id.clone(), repo))
    .collect()
  })
}

pub fn name_to_procedure(
) -> &'static HashMap<String, ProcedureListItem> {
  static NAME_TO_PROCEDURE: OnceLock<
    HashMap<String, ProcedureListItem>,
  > = OnceLock::new();
  NAME_TO_PROCEDURE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListProcedures::default()),
    )
    .expect("failed to get procedures from monitor")
    .into_iter()
    .map(|procedure| (procedure.name.clone(), procedure))
    .collect()
  })
}

pub fn id_to_procedure() -> &'static HashMap<String, ProcedureListItem>
{
  static ID_TO_PROCEDURE: OnceLock<
    HashMap<String, ProcedureListItem>,
  > = OnceLock::new();
  ID_TO_PROCEDURE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListProcedures::default()),
    )
    .expect("failed to get procedures from monitor")
    .into_iter()
    .map(|procedure| (procedure.id.clone(), procedure))
    .collect()
  })
}

pub fn name_to_user_group() -> &'static HashMap<String, UserGroup> {
  static NAME_TO_USER_GROUP: OnceLock<HashMap<String, UserGroup>> =
    OnceLock::new();
  NAME_TO_USER_GROUP.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListUserGroups::default()),
    )
    .expect("failed to get procedures from monitor")
    .into_iter()
    .map(|user_group| (user_group.name.clone(), user_group))
    .collect()
  })
}

pub fn id_to_user() -> &'static HashMap<String, User> {
  static ID_TO_USER: OnceLock<HashMap<String, User>> =
    OnceLock::new();
  ID_TO_USER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListUsers::default()),
    )
    .expect("failed to get procedures from monitor")
    .into_iter()
    .map(|user| (user.id.clone(), user))
    .collect()
  })
}
