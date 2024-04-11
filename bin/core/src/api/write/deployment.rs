use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    all_logs_success,
    build::Build,
    deployment::{
      Deployment, DeploymentImage, DockerContainerState,
      PartialDeploymentConfig,
    },
    monitor_timestamp,
    permission::PermissionLevel,
    server::Server,
    to_monitor_name,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    add_update, create_permission, empty_or_only_spaces,
    get_deployment_state, make_update, periphery_client,
    remove_from_recently_viewed,
    resource::{delete_all_permissions_on_resource, StateResource},
    update_update,
  },
  state::{action_states, State},
};

#[instrument(skip(user))]
async fn validate_config(
  config: &mut PartialDeploymentConfig,
  user: &User,
) -> anyhow::Result<()> {
  if let Some(server_id) = &config.server_id {
    if !server_id.is_empty() {
      let server = Server::get_resource_check_permissions(server_id, user, PermissionLevel::Write)
          .await
          .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
      config.server_id = Some(server.id);
    }
  }
  if let Some(DeploymentImage::Build { build_id, version }) =
    &config.image
  {
    if !build_id.is_empty() {
      let build = Build::get_resource_check_permissions(build_id, user, PermissionLevel::Read)
          .await
          .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
      config.image = Some(DeploymentImage::Build {
        build_id: build.id,
        version: version.clone(),
      });
    }
  }
  if let Some(volumes) = &mut config.volumes {
    volumes.retain(|v| {
      !empty_or_only_spaces(&v.local)
        && !empty_or_only_spaces(&v.container)
    })
  }
  if let Some(ports) = &mut config.ports {
    ports.retain(|v| {
      !empty_or_only_spaces(&v.local)
        && !empty_or_only_spaces(&v.container)
    })
  }
  if let Some(environment) = &mut config.environment {
    environment.retain(|v| {
      !empty_or_only_spaces(&v.variable)
        && !empty_or_only_spaces(&v.value)
    })
  }
  if let Some(extra_args) = &mut config.extra_args {
    extra_args.retain(|v| !empty_or_only_spaces(v))
  }
  Ok(())
}

#[async_trait]
impl Resolve<CreateDeployment, User> for State {
  #[instrument(name = "CreateDeployment", skip(self, user))]
  async fn resolve(
    &self,
    CreateDeployment { name, mut config }: CreateDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    let name = to_monitor_name(&name);
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("valid ObjectIds cannot be used as names"));
    }
    validate_config(&mut config, &user).await?;
    let start_ts = monitor_timestamp();
    let deployment = Deployment {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: (),
    };
    let deployment_id = db_client()
      .await
      .deployments
      .insert_one(&deployment, None)
      .await
      .context("failed to add deployment to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let deployment = Deployment::get_resource(&deployment_id).await?;
    create_permission(&user, &deployment, PermissionLevel::Write)
      .await;

    let mut update =
      make_update(&deployment, Operation::CreateDeployment, &user);
    update.push_simple_log(
      "create deployment",
      format!(
        "created deployment\nid: {}\nname: {}",
        deployment.id, deployment.name
      ),
    );
    update
      .push_simple_log("config", format!("{:#?}", deployment.config));
    update.finalize();

    add_update(update).await?;

    Ok(deployment)
  }
}

#[async_trait]
impl Resolve<CopyDeployment, User> for State {
  #[instrument(name = "CopyDeployment", skip(self, user))]
  async fn resolve(
    &self,
    CopyDeployment { name, id }: CopyDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    let name = to_monitor_name(&name);
    let Deployment {
      config,
      description,
      tags,
      ..
    } = Deployment::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    if !config.server_id.is_empty() {
      Server::get_resource_check_permissions(&config.server_id, &user, PermissionLevel::Write)
        .await
        .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
    }
    if let DeploymentImage::Build { build_id, .. } = &config.image {
      if !build_id.is_empty() {
        Build::get_resource_check_permissions(build_id, &user, PermissionLevel::Read)
          .await
          .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
      }
    }
    let start_ts = monitor_timestamp();
    let deployment = Deployment {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description,
      tags,
      config,
      info: (),
    };
    let deployment_id = db_client()
      .await
      .deployments
      .insert_one(&deployment, None)
      .await
      .context("failed to add deployment to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let deployment = Deployment::get_resource(&deployment_id).await?;

    create_permission(&user, &deployment, PermissionLevel::Write)
      .await;

    let mut update =
      make_update(&deployment, Operation::CreateDeployment, &user);
    update.push_simple_log(
      "create deployment",
      format!(
        "created deployment\nid: {}\nname: {}",
        deployment.id, deployment.name
      ),
    );
    update
      .push_simple_log("config", format!("{:#?}", deployment.config));
    update.finalize();

    add_update(update).await?;

    Ok(deployment)
  }
}

#[async_trait]
impl Resolve<DeleteDeployment, User> for State {
  #[instrument(name = "DeleteDeployment", skip(self, user))]
  async fn resolve(
    &self,
    DeleteDeployment { id }: DeleteDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    if action_states().deployment.busy(&id).await {
      return Err(anyhow!("deployment busy"));
    }

    let deployment = Deployment::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    let inner = || async move {
      let state = get_deployment_state(&deployment)
        .await
        .context("failed to get container state")?;

      let mut update =
        make_update(&deployment, Operation::DeleteDeployment, &user);
      update.in_progress();

      update.id = add_update(update.clone()).await?;

      if !matches!(
        state,
        DockerContainerState::NotDeployed
          | DockerContainerState::Unknown
      ) {
        // container needs to be destroyed
        let server =
          Server::get_resource(&deployment.config.server_id).await;
        if let Err(e) = server {
          update.logs.push(Log::error(
            "remove container",
            format!(
              "failed to retrieve server at {} from db | {e:#?}",
              deployment.config.server_id
            ),
          ));
        } else if let Ok(server) = server {
          match periphery_client(&server) {
            Ok(periphery) => match periphery
              .request(api::container::RemoveContainer {
                name: deployment.name.clone(),
                signal: deployment.config.termination_signal.into(),
                time: deployment.config.termination_timeout.into(),
              })
              .await
            {
              Ok(log) => update.logs.push(log),
              Err(e) => update.push_error_log(
                "remove container",
                format!(
                  "failed to remove container on periphery | {e:#?}"
                ),
              ),
            },
            Err(e) => update.push_error_log(
              "remove container",
              format!(
                "failed to remove container on periphery | {e:#?}"
              ),
            ),
          };
        }
      }

      let res = delete_one_by_id(
        &db_client().await.deployments,
        &deployment.id,
        None,
      )
      .await
      .context("failed to delete deployment from mongo");

      let log = match res {
        Ok(_) => Log::simple(
          "delete deployment",
          format!("deleted deployment {}", deployment.name),
        ),
        Err(e) => Log::error(
          "delete deployment",
          format!("failed to delete deployment\n{e:#?}"),
        ),
      };

      delete_all_permissions_on_resource(&deployment).await;

      update.logs.push(log);
      update.end_ts = Some(monitor_timestamp());
      update.status = UpdateStatus::Complete;
      update.success = all_logs_success(&update.logs);

      update_update(update).await?;

      remove_from_recently_viewed(&deployment).await?;

      Ok(deployment)
    };

    action_states()
      .deployment
      .update_entry(id.clone(), |entry| {
        entry.deleting = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .deployment
      .update_entry(id, |entry| {
        entry.deleting = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<UpdateDeployment, User> for State {
  #[instrument(name = "UpdateDeployment", skip(self, user))]
  async fn resolve(
    &self,
    UpdateDeployment { id, mut config }: UpdateDeployment,
    user: User,
  ) -> anyhow::Result<Deployment> {
    if action_states().deployment.busy(&id).await {
      return Err(anyhow!("deployment busy"));
    }

    let deployment = Deployment::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    let inner = || async move {
      let start_ts = monitor_timestamp();

      validate_config(&mut config, &user).await?;

      update_one_by_id(
        &db_client().await.deployments,
        &id,
        mungos::update::Update::FlattenSet(
          doc! { "config": to_bson(&config)? },
        ),
        None,
      )
      .await
      .context("failed to update server on mongo")?;

      let update = Update {
        operation: Operation::UpdateDeployment,
        target: ResourceTarget::Deployment(id.clone()),
        start_ts,
        end_ts: Some(monitor_timestamp()),
        status: UpdateStatus::Complete,
        logs: vec![Log::simple(
          "deployment update",
          serde_json::to_string_pretty(&config).unwrap(),
        )],
        operator: user.id.clone(),
        success: true,
        ..Default::default()
      };

      add_update(update).await?;

      let deployment: Deployment =
        Deployment::get_resource(&id).await?;

      anyhow::Ok(deployment)
    };

    action_states()
      .deployment
      .update_entry(deployment.id.clone(), |entry| {
        entry.updating = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .deployment
      .update_entry(deployment.id, |entry| {
        entry.updating = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<RenameDeployment, User> for State {
  #[instrument(name = "RenameDeployment", skip(self, user))]
  async fn resolve(
    &self,
    RenameDeployment { id, name }: RenameDeployment,
    user: User,
  ) -> anyhow::Result<Update> {
    let name = to_monitor_name(&name);
    if action_states().deployment.busy(&id).await {
      return Err(anyhow!("deployment busy"));
    }

    let deployment = Deployment::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    let inner = || async {
      let name = to_monitor_name(&name);

      let container_state = get_deployment_state(&deployment).await?;

      if container_state == DockerContainerState::Unknown {
        return Err(anyhow!(
          "cannot rename deployment when container status is unknown"
        ));
      }

      let mut update =
        make_update(&deployment, Operation::RenameDeployment, &user);

      update_one_by_id(
        &db_client().await.deployments,
        &deployment.id,
        mungos::update::Update::Set(
          doc! { "name": &name, "updated_at": monitor_timestamp() },
        ),
        None,
      )
      .await
      .context("failed to update deployment name on db")?;

      if container_state != DockerContainerState::NotDeployed {
        let server =
          Server::get_resource(&deployment.config.server_id).await?;
        let log = periphery_client(&server)?
          .request(api::container::RenameContainer {
            curr_name: deployment.name.clone(),
            new_name: name.clone(),
          })
          .await
          .context("failed to rename container on server")?;
        update.logs.push(log);
      }

      update.push_simple_log(
        "rename deployment",
        format!(
          "renamed deployment from {} to {}",
          deployment.name, name
        ),
      );
      update.finalize();

      add_update(update.clone()).await?;

      Ok(update)
    };

    action_states()
      .deployment
      .update_entry(id.clone(), |entry| {
        entry.renaming = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .deployment
      .update_entry(id, |entry| {
        entry.renaming = false;
      })
      .await;

    res
  }
}
