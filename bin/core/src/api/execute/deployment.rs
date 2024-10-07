use std::collections::HashSet;

use anyhow::{anyhow, Context};
use formatting::format_serror;
use komodo_client::{
  api::execute::*,
  entities::{
    build::{Build, ImageRegistryConfig},
    deployment::{
      extract_registry_domain, Deployment, DeploymentImage,
    },
    get_image_name,
    permission::PermissionLevel,
    server::Server,
    update::{Log, Update},
    user::User,
    Version,
  },
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  helpers::{
    interpolate::{
      add_interp_update_log,
      interpolate_variables_secrets_into_extra_args,
      interpolate_variables_secrets_into_string,
    },
    periphery_client,
    query::get_variables_and_secrets,
    registry_token,
    update::update_update,
  },
  monitor::update_cache_for_server,
  resource,
  state::{action_states, State},
};

async fn setup_deployment_execution(
  deployment: &str,
  user: &User,
) -> anyhow::Result<(Deployment, Server)> {
  let deployment = resource::get_check_permissions::<Deployment>(
    deployment,
    user,
    PermissionLevel::Execute,
  )
  .await?;

  if deployment.config.server_id.is_empty() {
    return Err(anyhow!("deployment has no server configured"));
  }

  let server =
    resource::get::<Server>(&deployment.config.server_id).await?;

  Ok((deployment, server))
}

impl Resolve<Deploy, (User, Update)> for State {
  #[instrument(name = "Deploy", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    Deploy {
      deployment,
      stop_signal,
      stop_time,
    }: Deploy,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (mut deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.deploying = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    periphery
      .health_check()
      .await
      .context("Failed server health check, stopping run.")?;

    // This block resolves the attached Build to an actual versioned image
    let (version, registry_token) = match &deployment.config.image {
      DeploymentImage::Build { build_id, version } => {
        let build = resource::get::<Build>(build_id).await?;
        let image_name = get_image_name(&build)
          .context("failed to create image name")?;
        let version = if version.is_none() {
          build.config.version
        } else {
          *version
        };
        // Remove ending patch if it is 0, this means use latest patch.
        let version_str = if version.patch == 0 {
          format!("{}.{}", version.major, version.minor)
        } else {
          version.to_string()
        };
        // Potentially add the build image_tag postfix
        let version_str = if build.config.image_tag.is_empty() {
          version_str
        } else {
          format!("{version_str}-{}", build.config.image_tag)
        };
        // replace image with corresponding build image.
        deployment.config.image = DeploymentImage::Image {
          image: format!("{image_name}:{version_str}"),
        };
        if build.config.image_registry.domain.is_empty() {
          (version, None)
        } else {
          let ImageRegistryConfig {
            domain, account, ..
          } = build.config.image_registry;
          if deployment.config.image_registry_account.is_empty() {
            deployment.config.image_registry_account = account
          }
          let token = if !deployment
            .config
            .image_registry_account
            .is_empty()
          {
            registry_token(&domain, &deployment.config.image_registry_account).await.with_context(
              || format!("Failed to get git token in call to db. Stopping run. | {domain} | {}", deployment.config.image_registry_account),
            )?
          } else {
            None
          };
          (version, token)
        }
      }
      DeploymentImage::Image { image } => {
        let domain = extract_registry_domain(image)?;
        let token = if !deployment
          .config
          .image_registry_account
          .is_empty()
        {
          registry_token(&domain, &deployment.config.image_registry_account).await.with_context(
            || format!("Failed to get git token in call to db. Stopping run. | {domain} | {}", deployment.config.image_registry_account),
          )?
        } else {
          None
        };
        (Version::default(), token)
      }
    };

    // interpolate variables / secrets, returning the sanitizing replacers to send to
    // periphery so it may sanitize the final command for safe logging (avoids exposing secret values)
    let secret_replacers = if !deployment.config.skip_secret_interp {
      let vars_and_secrets = get_variables_and_secrets().await?;

      let mut global_replacers = HashSet::new();
      let mut secret_replacers = HashSet::new();

      interpolate_variables_secrets_into_string(
        &vars_and_secrets,
        &mut deployment.config.environment,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_string(
        &vars_and_secrets,
        &mut deployment.config.ports,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_string(
        &vars_and_secrets,
        &mut deployment.config.volumes,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_extra_args(
        &vars_and_secrets,
        &mut deployment.config.extra_args,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      interpolate_variables_secrets_into_string(
        &vars_and_secrets,
        &mut deployment.config.command,
        &mut global_replacers,
        &mut secret_replacers,
      )?;

      add_interp_update_log(
        &mut update,
        &global_replacers,
        &secret_replacers,
      );

      secret_replacers
    } else {
      Default::default()
    };

    update.version = version;
    update_update(update.clone()).await?;

    match periphery
      .request(api::container::Deploy {
        deployment,
        stop_signal,
        stop_time,
        registry_token,
        replacers: secret_replacers.into_iter().collect(),
      })
      .await
    {
      Ok(log) => update.logs.push(log),
      Err(e) => {
        update.push_error_log(
          "deploy container",
          format_serror(
            &e.context("failed to deploy container").into(),
          ),
        );
      }
    };

    update_cache_for_server(&server).await;

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StartDeployment, (User, Update)> for State {
  #[instrument(name = "StartDeployment", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StartDeployment { deployment }: StartDeployment,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.starting = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::StartContainer {
        name: deployment.name,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "start container",
        format_serror(&e.context("failed to start container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<RestartDeployment, (User, Update)> for State {
  #[instrument(name = "RestartDeployment", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RestartDeployment { deployment }: RestartDeployment,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.restarting = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::RestartContainer {
        name: deployment.name,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "restart container",
        format_serror(
          &e.context("failed to restart container").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<PauseDeployment, (User, Update)> for State {
  #[instrument(name = "PauseDeployment", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    PauseDeployment { deployment }: PauseDeployment,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.pausing = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::PauseContainer {
        name: deployment.name,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "pause container",
        format_serror(&e.context("failed to pause container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<UnpauseDeployment, (User, Update)> for State {
  #[instrument(name = "UnpauseDeployment", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    UnpauseDeployment { deployment }: UnpauseDeployment,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.unpausing = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::UnpauseContainer {
        name: deployment.name,
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "unpause container",
        format_serror(
          &e.context("failed to unpause container").into(),
        ),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<StopDeployment, (User, Update)> for State {
  #[instrument(name = "StopDeployment", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    StopDeployment {
      deployment,
      signal,
      time,
    }: StopDeployment,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.stopping = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::StopContainer {
        name: deployment.name,
        signal: signal
          .unwrap_or(deployment.config.termination_signal)
          .into(),
        time: time
          .unwrap_or(deployment.config.termination_timeout)
          .into(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "stop container",
        format_serror(&e.context("failed to stop container").into()),
      ),
    };

    update.logs.push(log);
    update_cache_for_server(&server).await;
    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

impl Resolve<DestroyDeployment, (User, Update)> for State {
  #[instrument(name = "DestroyDeployment", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    DestroyDeployment {
      deployment,
      signal,
      time,
    }: DestroyDeployment,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let (deployment, server) =
      setup_deployment_execution(&deployment, &user).await?;

    // get the action state for the deployment (or insert default).
    let action_state = action_states()
      .deployment
      .get_or_insert_default(&deployment.id)
      .await;

    // Will check to ensure deployment not already busy before updating, and return Err if so.
    // The returned guard will set the action state back to default when dropped.
    let _action_guard =
      action_state.update(|state| state.destroying = true)?;

    // Send update after setting action state, this way frontend gets correct state.
    update_update(update.clone()).await?;

    let periphery = periphery_client(&server)?;

    let log = match periphery
      .request(api::container::RemoveContainer {
        name: deployment.name,
        signal: signal
          .unwrap_or(deployment.config.termination_signal)
          .into(),
        time: time
          .unwrap_or(deployment.config.termination_timeout)
          .into(),
      })
      .await
    {
      Ok(log) => log,
      Err(e) => Log::error(
        "stop container",
        format_serror(&e.context("failed to stop container").into()),
      ),
    };

    update.logs.push(log);
    update.finalize();
    update_cache_for_server(&server).await;
    update_update(update.clone()).await?;

    Ok(update)
  }
}
