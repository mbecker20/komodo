use std::{collections::HashMap, sync::OnceLock};

use monitor_client::{
  api::read,
  entities::{
    alerter::Alerter, build::Build, builder::Builder,
    deployment::Deployment, procedure::Procedure, repo::Repo,
    server::Server, server_template::ServerTemplate, tag::Tag,
    user::User, user_group::UserGroup, variable::Variable,
  },
};

use crate::monitor_client;

pub fn name_to_build() -> &'static HashMap<String, Build> {
  static NAME_TO_BUILD: OnceLock<HashMap<String, Build>> =
    OnceLock::new();
  NAME_TO_BUILD.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullBuilds::default()),
    )
    .expect("failed to get builds from monitor")
    .into_iter()
    .map(|build| (build.name.clone(), build))
    .collect()
  })
}

pub fn id_to_build() -> &'static HashMap<String, Build> {
  static ID_TO_BUILD: OnceLock<HashMap<String, Build>> =
    OnceLock::new();
  ID_TO_BUILD.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullBuilds::default()),
    )
    .expect("failed to get builds from monitor")
    .into_iter()
    .map(|build| (build.id.clone(), build))
    .collect()
  })
}

pub fn name_to_deployment() -> &'static HashMap<String, Deployment> {
  static NAME_TO_DEPLOYMENT: OnceLock<HashMap<String, Deployment>> =
    OnceLock::new();
  NAME_TO_DEPLOYMENT.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullDeployments::default()),
    )
    .expect("failed to get deployments from monitor")
    .into_iter()
    .map(|deployment| (deployment.name.clone(), deployment))
    .collect()
  })
}

pub fn id_to_deployment() -> &'static HashMap<String, Deployment> {
  static ID_TO_DEPLOYMENT: OnceLock<HashMap<String, Deployment>> =
    OnceLock::new();
  ID_TO_DEPLOYMENT.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullDeployments::default()),
    )
    .expect("failed to get deployments from monitor")
    .into_iter()
    .map(|deployment| (deployment.id.clone(), deployment))
    .collect()
  })
}

pub fn name_to_server() -> &'static HashMap<String, Server> {
  static NAME_TO_SERVER: OnceLock<HashMap<String, Server>> =
    OnceLock::new();
  NAME_TO_SERVER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullServers::default()),
    )
    .expect("failed to get servers from monitor")
    .into_iter()
    .map(|server| (server.name.clone(), server))
    .collect()
  })
}

pub fn id_to_server() -> &'static HashMap<String, Server> {
  static ID_TO_SERVER: OnceLock<HashMap<String, Server>> =
    OnceLock::new();
  ID_TO_SERVER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullServers::default()),
    )
    .expect("failed to get servers from monitor")
    .into_iter()
    .map(|server| (server.id.clone(), server))
    .collect()
  })
}

pub fn name_to_builder() -> &'static HashMap<String, Builder> {
  static NAME_TO_BUILDER: OnceLock<HashMap<String, Builder>> =
    OnceLock::new();
  NAME_TO_BUILDER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullBuilders::default()),
    )
    .expect("failed to get builders from monitor")
    .into_iter()
    .map(|builder| (builder.name.clone(), builder))
    .collect()
  })
}

pub fn id_to_builder() -> &'static HashMap<String, Builder> {
  static ID_TO_BUILDER: OnceLock<HashMap<String, Builder>> =
    OnceLock::new();
  ID_TO_BUILDER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullBuilders::default()),
    )
    .expect("failed to get builders from monitor")
    .into_iter()
    .map(|builder| (builder.id.clone(), builder))
    .collect()
  })
}

pub fn name_to_alerter() -> &'static HashMap<String, Alerter> {
  static NAME_TO_ALERTER: OnceLock<HashMap<String, Alerter>> =
    OnceLock::new();
  NAME_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullAlerters::default()),
    )
    .expect("failed to get alerters from monitor")
    .into_iter()
    .map(|alerter| (alerter.name.clone(), alerter))
    .collect()
  })
}

pub fn id_to_alerter() -> &'static HashMap<String, Alerter> {
  static ID_TO_ALERTER: OnceLock<HashMap<String, Alerter>> =
    OnceLock::new();
  ID_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullAlerters::default()),
    )
    .expect("failed to get alerters from monitor")
    .into_iter()
    .map(|alerter| (alerter.id.clone(), alerter))
    .collect()
  })
}

pub fn name_to_repo() -> &'static HashMap<String, Repo> {
  static NAME_TO_ALERTER: OnceLock<HashMap<String, Repo>> =
    OnceLock::new();
  NAME_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullRepos::default()),
    )
    .expect("failed to get repos from monitor")
    .into_iter()
    .map(|repo| (repo.name.clone(), repo))
    .collect()
  })
}

pub fn id_to_repo() -> &'static HashMap<String, Repo> {
  static ID_TO_ALERTER: OnceLock<HashMap<String, Repo>> =
    OnceLock::new();
  ID_TO_ALERTER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullRepos::default()),
    )
    .expect("failed to get repos from monitor")
    .into_iter()
    .map(|repo| (repo.id.clone(), repo))
    .collect()
  })
}

pub fn name_to_procedure() -> &'static HashMap<String, Procedure> {
  static NAME_TO_PROCEDURE: OnceLock<HashMap<String, Procedure>> =
    OnceLock::new();
  NAME_TO_PROCEDURE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullProcedures::default()),
    )
    .expect("failed to get procedures from monitor")
    .into_iter()
    .map(|procedure| (procedure.name.clone(), procedure))
    .collect()
  })
}

pub fn id_to_procedure() -> &'static HashMap<String, Procedure> {
  static ID_TO_PROCEDURE: OnceLock<HashMap<String, Procedure>> =
    OnceLock::new();
  ID_TO_PROCEDURE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullProcedures::default()),
    )
    .expect("failed to get procedures from monitor")
    .into_iter()
    .map(|procedure| (procedure.id.clone(), procedure))
    .collect()
  })
}

pub fn name_to_server_template(
) -> &'static HashMap<String, ServerTemplate> {
  static NAME_TO_SERVER_TEMPLATE: OnceLock<
    HashMap<String, ServerTemplate>,
  > = OnceLock::new();
  NAME_TO_SERVER_TEMPLATE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullServerTemplates::default()),
    )
    .expect("failed to get server templates from monitor")
    .into_iter()
    .map(|procedure| (procedure.name.clone(), procedure))
    .collect()
  })
}

pub fn id_to_server_template(
) -> &'static HashMap<String, ServerTemplate> {
  static ID_TO_SERVER_TEMPLATE: OnceLock<
    HashMap<String, ServerTemplate>,
  > = OnceLock::new();
  ID_TO_SERVER_TEMPLATE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListFullServerTemplates::default()),
    )
    .expect("failed to get server templates from monitor")
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
    .expect("failed to get user groups from monitor")
    .into_iter()
    .map(|user_group| (user_group.name.clone(), user_group))
    .collect()
  })
}

pub fn name_to_variable() -> &'static HashMap<String, Variable> {
  static NAME_TO_VARIABLE: OnceLock<HashMap<String, Variable>> =
    OnceLock::new();
  NAME_TO_VARIABLE.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListVariables::default()),
    )
    .expect("failed to get user groups from monitor")
    .variables
    .into_iter()
    .map(|variable| (variable.name.clone(), variable))
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
    .expect("failed to get users from monitor")
    .into_iter()
    .map(|user| (user.id.clone(), user))
    .collect()
  })
}

pub fn id_to_tag() -> &'static HashMap<String, Tag> {
  static ID_TO_TAG: OnceLock<HashMap<String, Tag>> = OnceLock::new();
  ID_TO_TAG.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListTags::default()),
    )
    .expect("failed to get tags from monitor")
    .into_iter()
    .map(|tag| (tag.id.clone(), tag))
    .collect()
  })
}
