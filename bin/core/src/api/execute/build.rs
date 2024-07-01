use std::{collections::HashSet, future::IntoFuture, time::Duration};

use anyhow::{anyhow, Context};
use formatting::{format_serror, muted};
use futures::future::join_all;
use monitor_client::{
  api::execute::{
    CancelBuild, CancelBuildResponse, Deploy, RunBuild,
  },
  entities::{
    alert::{Alert, AlertData},
    all_logs_success,
    build::{Build, CloudRegistryConfig, ImageRegistry},
    builder::{AwsBuilderConfig, Builder, BuilderConfig},
    config::core::{AwsEcrConfig, AwsEcrConfigWithCredentials},
    deployment::DeploymentState,
    monitor_timestamp,
    permission::PermissionLevel,
    server::{stats::SeverityLevel, Server},
    server_template::aws::AwsServerTemplateConfig,
    to_monitor_name,
    update::{Log, Update},
    user::{auto_redeploy_user, User},
  },
};
use mungos::{
  by_id::update_one_by_id,
  find::find_collect,
  mongodb::{
    bson::{doc, to_bson, to_document},
    options::FindOneOptions,
  },
};
use periphery_client::{
  api::{self, GetVersionResponse},
  PeripheryClient,
};
use resolver_api::Resolve;
use tokio_util::sync::CancellationToken;

use crate::{
  cloud::{
    aws::{
      ec2::{
        launch_ec2_instance, terminate_ec2_instance_with_retry,
        Ec2Instance,
      },
      ecr,
    },
    BuildCleanupData,
  },
  config::core_config,
  helpers::{
    alert::send_alerts,
    channel::build_cancel_channel,
    periphery_client,
    query::{get_deployment_state, get_global_variables},
    update::update_update,
  },
  resource::{self, refresh_build_state_cache},
  state::{action_states, db_client, State},
};

use crate::helpers::update::init_execution_update;

use super::ExecuteRequest;

impl Resolve<RunBuild, (User, Update)> for State {
  #[instrument(name = "RunBuild", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    RunBuild { build }: RunBuild,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<Update> {
    let mut build = resource::get_check_permissions::<Build>(
      &build,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    let (registry_token, aws_ecr) =
      validate_account_extract_registry_token_aws_ecr(&build).await?;

    // get the action state for the build (or insert default).
    let action_state =
      action_states().build.get_or_insert_default(&build.id).await;

    // This will set action state back to default when dropped.
    // Will also check to ensure build not already busy before updating.
    let _action_guard =
      action_state.update(|state| state.building = true)?;

    build.config.version.increment();
    update.version = build.config.version;
    update_update(update.clone()).await?;

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let mut cancel_recv =
      build_cancel_channel().receiver.resubscribe();
    let build_id = build.id.clone();

    tokio::spawn(async move {
      let poll = async {
        loop {
          let (incoming_build_id, mut update) = tokio::select! {
            _ = cancel_clone.cancelled() => return Ok(()),
            id = cancel_recv.recv() => id?
          };
          if incoming_build_id == build_id {
            update.push_simple_log(
              "cancel acknowledged",
              "the build cancellation has been queued, it may still take some time",
            );
            update.finalize();
            let id = update.id.clone();
            if let Err(e) = update_update(update).await {
              warn!("failed to update Update {id} | {e:#}");
            }
            cancel_clone.cancel();
            return Ok(());
          }
        }
        #[allow(unreachable_code)]
        anyhow::Ok(())
      };
      tokio::select! {
        _ = cancel_clone.cancelled() => {}
        _ = poll => {}
      }
    });

    // GET BUILDER PERIPHERY

    let (periphery, cleanup_data) =
      match get_build_builder(&build, &mut update).await {
        Ok(builder) => {
          info!("got builder for build");
          builder
        }
        Err(e) => {
          warn!("failed to get builder | {e:#}");
          update.logs.push(Log::error(
            "get builder",
            format_serror(&e.context("failed to get builder").into()),
          ));
          return handle_early_return(
            update, build.id, build.name, false,
          )
          .await;
        }
      };

    let core_config = core_config();
    let variables = get_global_variables().await?;

    // CLONE REPO

    let github_token = core_config
      .github_accounts
      .get(&build.config.github_account)
      .cloned();

    let res = tokio::select! {
      res = periphery
        .request(api::git::CloneRepo {
          args: (&build).into(),
          github_token,
        }) => res,
      _ = cancel.cancelled() => {
        info!("build cancelled during clone, cleaning up builder");
        update.push_error_log("build cancelled", String::from("user cancelled build during repo clone"));
        cleanup_builder_instance(periphery, cleanup_data, &mut update)
          .await;
        info!("builder cleaned up");
        return handle_early_return(update, build.id, build.name, true).await
      },
    };

    match res {
      Ok(clone_logs) => {
        info!("finished repo clone");
        update.logs.extend(clone_logs);
      }
      Err(e) => {
        warn!("failed build at clone repo | {e:#}");
        update.push_error_log(
          "clone repo",
          format_serror(&e.context("failed to clone repo").into()),
        );
      }
    }

    update_update(update.clone()).await?;

    if all_logs_success(&update.logs) {
      // Interpolate variables / secrets into build args
      let mut global_replacers = HashSet::new();
      let mut secret_replacers = HashSet::new();
      let mut secret_replacers_for_log = HashSet::new();

      // Interpolate into build args
      for arg in &mut build.config.build_args {
        // first pass - global variables
        let (res, more_replacers) = svi::interpolate_variables(
          &arg.value,
          &variables,
          svi::Interpolator::DoubleBrackets,
          false,
        )
        .context("failed to interpolate global variables")?;
        global_replacers.extend(more_replacers);
        // second pass - core secrets
        let (res, more_replacers) = svi::interpolate_variables(
          &res,
          &core_config.secrets,
          svi::Interpolator::DoubleBrackets,
          false,
        )
        .context("failed to interpolate core secrets")?;
        secret_replacers_for_log.extend(
          more_replacers.iter().map(|(_, variable)| variable.clone()),
        );
        secret_replacers.extend(more_replacers);
        arg.value = res;
      }

      // Interpolate into secret args
      for arg in &mut build.config.secret_args {
        // first pass - global variables
        let (res, more_replacers) = svi::interpolate_variables(
          &arg.value,
          &variables,
          svi::Interpolator::DoubleBrackets,
          false,
        )
        .context("failed to interpolate global variables")?;
        global_replacers.extend(more_replacers);
        // second pass - core secrets
        let (res, more_replacers) = svi::interpolate_variables(
          &res,
          &core_config.secrets,
          svi::Interpolator::DoubleBrackets,
          false,
        )
        .context("failed to interpolate core secrets")?;
        secret_replacers_for_log.extend(
          more_replacers.into_iter().map(|(_, variable)| variable),
        );
        // Secret args don't need to be in replacers sent to periphery.
        // The secret args don't end up in the command like build args do.
        arg.value = res;
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
          secret_replacers_for_log
            .into_iter()
            .map(|variable| format!("<span class=\"text-muted-foreground\">replaced:</span> {variable}"))
            .collect::<Vec<_>>()
            .join("\n"),
        );
      }

      let res = tokio::select! {
        res = periphery
          .request(api::build::Build {
            build: build.clone(),
            registry_token,
            aws_ecr,
            replacers: secret_replacers.into_iter().collect(),
          }) => res.context("failed at call to periphery to build"),
        _ = cancel.cancelled() => {
          info!("build cancelled during build, cleaning up builder");
          update.push_error_log("build cancelled", String::from("user cancelled build during docker build"));
          cleanup_builder_instance(periphery, cleanup_data, &mut update)
            .await;
          return handle_early_return(update, build.id, build.name, true).await
        },
      };

      match res {
        Ok(logs) => {
          info!("finished build");
          update.logs.extend(logs);
        }
        Err(e) => {
          warn!("error in build | {e:#}");
          update.push_error_log(
            "build",
            format_serror(&e.context("failed to build").into()),
          )
        }
      };
    }

    update.finalize();

    let db = db_client().await;

    if update.success {
      let _ = db
        .builds
        .update_one(
          doc! { "name": &build.name },
          doc! {
            "$set": {
              "config.version": to_bson(&build.config.version)
                .context("failed at converting version to bson")?,
              "info.last_built_at": monitor_timestamp(),
            }
          },
        )
        .await;
    }

    // stop the cancel listening task from going forever
    cancel.cancel();

    cleanup_builder_instance(periphery, cleanup_data, &mut update)
      .await;

    // Need to manually update the update before cache refresh,
    // and before broadcast with add_update.
    // The Err case of to_document should be unreachable,
    // but will fail to update cache in that case.
    if let Ok(update_doc) = to_document(&update) {
      let _ = update_one_by_id(
        &db.updates,
        &update.id,
        mungos::update::Update::Set(update_doc),
        None,
      )
      .await;
      refresh_build_state_cache().await;
    }

    update_update(update.clone()).await?;

    if update.success {
      // don't hold response up for user
      tokio::spawn(async move {
        handle_post_build_redeploy(&build.id).await;
        info!("post build redeploy handled");
      });
    } else {
      let target = update.target.clone();
      let version = update.version;
      let err = update.logs.iter().find(|l| !l.success).cloned();
      tokio::spawn(async move {
        let alert = Alert {
          id: Default::default(),
          target,
          ts: monitor_timestamp(),
          resolved_ts: Some(monitor_timestamp()),
          resolved: true,
          level: SeverityLevel::Warning,
          data: AlertData::BuildFailed {
            id: build.id,
            name: build.name,
            err,
            version,
          },
        };
        send_alerts(&[alert]).await
      });
    }

    Ok(update)
  }
}

#[instrument(skip(update))]
async fn handle_early_return(
  mut update: Update,
  build_id: String,
  build_name: String,
  is_cancel: bool,
) -> anyhow::Result<Update> {
  update.finalize();
  // Need to manually update the update before cache refresh,
  // and before broadcast with add_update.
  // The Err case of to_document should be unreachable,
  // but will fail to update cache in that case.
  if let Ok(update_doc) = to_document(&update) {
    let _ = update_one_by_id(
      &db_client().await.updates,
      &update.id,
      mungos::update::Update::Set(update_doc),
      None,
    )
    .await;
    refresh_build_state_cache().await;
  }
  update_update(update.clone()).await?;
  if !update.success && !is_cancel {
    let target = update.target.clone();
    let version = update.version;
    let err = update.logs.iter().find(|l| !l.success).cloned();
    tokio::spawn(async move {
      let alert = Alert {
        id: Default::default(),
        target,
        ts: monitor_timestamp(),
        resolved_ts: Some(monitor_timestamp()),
        resolved: true,
        level: SeverityLevel::Warning,
        data: AlertData::BuildFailed {
          id: build_id,
          name: build_name,
          version,
          err,
        },
      };
      send_alerts(&[alert]).await
    });
  }
  Ok(update)
}

#[instrument(skip_all)]
pub async fn validate_cancel_build(
  request: &ExecuteRequest,
) -> anyhow::Result<()> {
  if let ExecuteRequest::CancelBuild(req) = request {
    let build = resource::get::<Build>(&req.build).await?;

    let db = db_client().await;

    let (latest_build, latest_cancel) = tokio::try_join!(
      db.updates
        .find_one(doc! {
          "operation": "RunBuild",
          "target.id": &build.id,
        },)
        .with_options(
          FindOneOptions::builder()
            .sort(doc! { "start_ts": -1 })
            .build()
        )
        .into_future(),
      db.updates
        .find_one(doc! {
          "operation": "CancelBuild",
          "target.id": &build.id,
        },)
        .with_options(
          FindOneOptions::builder()
            .sort(doc! { "start_ts": -1 })
            .build()
        )
        .into_future()
    )?;

    match (latest_build, latest_cancel) {
      (Some(build), Some(cancel)) => {
        if cancel.start_ts > build.start_ts {
          return Err(anyhow!("Build has already been cancelled"));
        }
      }
      (None, _) => return Err(anyhow!("No build in progress")),
      _ => {}
    };
  }
  Ok(())
}

impl Resolve<CancelBuild, (User, Update)> for State {
  #[instrument(name = "CancelBuild", skip(self, user, update), fields(user_id = user.id, update_id = update.id))]
  async fn resolve(
    &self,
    CancelBuild { build }: CancelBuild,
    (user, mut update): (User, Update),
  ) -> anyhow::Result<CancelBuildResponse> {
    let build = resource::get_check_permissions::<Build>(
      &build,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // make sure the build is building
    if !action_states()
      .build
      .get(&build.id)
      .await
      .and_then(|s| s.get().ok().map(|s| s.building))
      .unwrap_or_default()
    {
      return Err(anyhow!("Build is not building."));
    }

    update.push_simple_log(
      "cancel triggered",
      "the build cancel has been triggered",
    );
    update_update(update.clone()).await?;

    let update_id = update.id.clone();

    build_cancel_channel()
      .sender
      .lock()
      .await
      .send((build.id, update))?;

    // Make sure cancel is set to complete after some time in case
    // no reciever is there to do it. Prevents update stuck in InProgress.
    tokio::spawn(async move {
      tokio::time::sleep(Duration::from_secs(60)).await;
      if let Err(e) = update_one_by_id(
        &db_client().await.updates,
        &update_id,
        doc! { "$set": { "status": "Complete" } },
        None,
      )
      .await
      {
        warn!("failed to set BuildCancel Update status Complete after timeout | {e:#}")
      }
    });

    Ok(CancelBuildResponse {})
  }
}

const BUILDER_POLL_RATE_SECS: u64 = 2;
const BUILDER_POLL_MAX_TRIES: usize = 30;

#[instrument(skip_all, fields(build_id = build.id, update_id = update.id))]
async fn get_build_builder(
  build: &Build,
  update: &mut Update,
) -> anyhow::Result<(PeripheryClient, BuildCleanupData)> {
  if build.config.builder_id.is_empty() {
    return Err(anyhow!("build has not configured a builder"));
  }
  let builder =
    resource::get::<Builder>(&build.config.builder_id).await?;
  match builder.config {
    BuilderConfig::Server(config) => {
      if config.server_id.is_empty() {
        return Err(anyhow!("builder has not configured a server"));
      }
      let server = resource::get::<Server>(&config.server_id).await?;
      let periphery = periphery_client(&server)?;
      Ok((
        periphery,
        BuildCleanupData::Server {
          repo_name: build.name.clone(),
        },
      ))
    }
    BuilderConfig::Aws(config) => {
      get_aws_builder(build, config, update).await
    }
  }
}

#[instrument(skip_all, fields(build_id = build.id, update_id = update.id))]
async fn get_aws_builder(
  build: &Build,
  config: AwsBuilderConfig,
  update: &mut Update,
) -> anyhow::Result<(PeripheryClient, BuildCleanupData)> {
  let start_create_ts = monitor_timestamp();

  let instance_name =
    format!("BUILDER-{}-v{}", build.name, build.config.version);
  let Ec2Instance { instance_id, ip } = launch_ec2_instance(
    &instance_name,
    AwsServerTemplateConfig::from_builder_config(&config),
  )
  .await?;

  info!("ec2 instance launched");

  let log = Log {
    stage: "start build instance".to_string(),
    success: true,
    stdout: start_aws_builder_log(&instance_id, &ip, &config),
    start_ts: start_create_ts,
    end_ts: monitor_timestamp(),
    ..Default::default()
  };

  update.logs.push(log);

  update_update(update.clone()).await?;

  let periphery_address = format!("http://{ip}:{}", config.port);
  let periphery =
    PeripheryClient::new(&periphery_address, &core_config().passkey);

  let start_connect_ts = monitor_timestamp();
  let mut res = Ok(GetVersionResponse {
    version: String::new(),
  });
  for _ in 0..BUILDER_POLL_MAX_TRIES {
    let version = periphery
      .request(api::GetVersion {})
      .await
      .context("failed to reach periphery client on builder");
    if let Ok(GetVersionResponse { version }) = &version {
      let connect_log = Log {
        stage: "build instance connected".to_string(),
        success: true,
        stdout: format!(
          "established contact with periphery on builder\nperiphery version: v{}",
          version
        ),
        start_ts: start_connect_ts,
        end_ts: monitor_timestamp(),
        ..Default::default()
      };
      update.logs.push(connect_log);
      update_update(update.clone()).await?;
      return Ok((
        periphery,
        BuildCleanupData::Aws {
          instance_id,
          region: config.region,
        },
      ));
    }
    res = version;
    tokio::time::sleep(Duration::from_secs(BUILDER_POLL_RATE_SECS))
      .await;
  }

  // Spawn terminate task in failure case (if loop is passed without return)
  tokio::spawn(async move {
    let _ =
      terminate_ec2_instance_with_retry(config.region, &instance_id)
        .await;
  });

  // Unwrap is safe, only way to get here is after check Ok / early return, so it must be err
  Err(
    res.err().unwrap().context(
      "failed to start usable builder. terminating instance.",
    ),
  )
}

#[instrument(skip(periphery, update))]
async fn cleanup_builder_instance(
  periphery: PeripheryClient,
  cleanup_data: BuildCleanupData,
  update: &mut Update,
) {
  match cleanup_data {
    BuildCleanupData::Server { repo_name } => {
      let _ = periphery
        .request(api::git::DeleteRepo { name: repo_name })
        .await;
    }
    BuildCleanupData::Aws {
      instance_id,
      region,
    } => {
      let _instance_id = instance_id.clone();
      tokio::spawn(async move {
        let _ =
          terminate_ec2_instance_with_retry(region, &_instance_id)
            .await;
      });
      update.push_simple_log(
        "terminate instance",
        format!("termination queued for instance id {instance_id}"),
      );
    }
  }
}

#[instrument]
async fn handle_post_build_redeploy(build_id: &str) {
  let Ok(redeploy_deployments) = find_collect(
    &db_client().await.deployments,
    doc! {
      "config.image.params.build_id": build_id,
      "config.redeploy_on_build": true
    },
    None,
  )
  .await
  else {
    return;
  };

  let futures =
    redeploy_deployments
      .into_iter()
      .map(|deployment| async move {
        let state =
          get_deployment_state(&deployment).await.unwrap_or_default();
        if state == DeploymentState::Running {
          let req = super::ExecuteRequest::Deploy(Deploy {
            deployment: deployment.id.clone(),
            stop_signal: None,
            stop_time: None,
          });
          let user = auto_redeploy_user().to_owned();
          let res = async {
            let update = init_execution_update(&req, &user).await?;
            State
              .resolve(
                Deploy {
                  deployment: deployment.id.clone(),
                  stop_signal: None,
                  stop_time: None,
                },
                (user, update),
              )
              .await
          }
          .await;
          Some((deployment.id.clone(), res))
        } else {
          None
        }
      });

  for res in join_all(futures).await {
    let Some((id, res)) = res else {
      continue;
    };
    if let Err(e) = res {
      warn!("failed post build redeploy for deployment {id}: {e:#}");
    }
  }
}

fn start_aws_builder_log(
  instance_id: &str,
  ip: &str,
  config: &AwsBuilderConfig,
) -> String {
  let AwsBuilderConfig {
    ami_id,
    instance_type,
    volume_gb,
    subnet_id,
    assign_public_ip,
    security_group_ids,
    use_public_ip,
    ..
  } = config;

  let readable_sec_group_ids = security_group_ids.join(", ");

  [
    format!("{}: {instance_id}", muted("instance id")),
    format!("{}: {ip}", muted("ip")),
    format!("{}: {ami_id}", muted("ami id")),
    format!("{}: {instance_type}", muted("instance type")),
    format!("{}: {volume_gb} GB", muted("volume size")),
    format!("{}: {subnet_id}", muted("subnet id")),
    format!("{}: {readable_sec_group_ids}", muted("security groups")),
    format!("{}: {assign_public_ip}", muted("assign public ip")),
    format!("{}: {use_public_ip}", muted("use public ip")),
  ]
  .join("\n")
}

/// This will make sure that a build with non-none image registry has an account attached,
/// and will check the core config for a token / aws ecr config matching requirements.
/// Otherwise it is left to periphery.
async fn validate_account_extract_registry_token_aws_ecr(
  build: &Build,
) -> anyhow::Result<(Option<String>, Option<AwsEcrConfig>)> {
  match &build.config.image_registry {
    ImageRegistry::None(_) => Ok((None, None)),
    ImageRegistry::DockerHub(CloudRegistryConfig {
      account, ..
    }) => {
      if account.is_empty() {
        return Err(anyhow!(
          "Must attach account to use DockerHub image registry"
        ));
      }
      Ok((core_config().docker_accounts.get(account).cloned(), None))
    }
    ImageRegistry::Ghcr(CloudRegistryConfig { account, .. }) => {
      if account.is_empty() {
        return Err(anyhow!(
          "Must attach account to use GithubContainerRegistry"
        ));
      }
      Ok((core_config().github_accounts.get(account).cloned(), None))
    }
    ImageRegistry::AwsEcr(label) => {
      let config = core_config().aws_ecr_registries.get(label);
      let token = match config {
        Some(AwsEcrConfigWithCredentials {
          region,
          access_key_id,
          secret_access_key,
          ..
        }) => {
          let token = ecr::get_ecr_token(
            region,
            access_key_id,
            secret_access_key,
          )
          .await
          .context("failed to get aws ecr token")?;
          ecr::maybe_create_repo(
            &to_monitor_name(&build.name),
            region.to_string(),
            access_key_id,
            secret_access_key,
          )
          .await
          .context("failed to create aws ecr repo")?;
          Some(token)
        }
        None => None,
      };
      Ok((token, config.map(AwsEcrConfig::from)))
    }
    ImageRegistry::Custom(_) => {
      Err(anyhow!("Custom image registry is not implemented"))
    }
  }
}
