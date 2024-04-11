use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_client::{
  api::execute::{
    CancelBuild, CancelBuildResponse, Deploy, RunBuild,
  },
  entities::{
    all_logs_success,
    build::Build,
    builder::{AwsBuilderConfig, Builder, BuilderConfig},
    deployment::DockerContainerState,
    monitor_timestamp,
    permission::PermissionLevel,
    server::Server,
    update::{Log, Update},
    user::{auto_redeploy_user, User},
    Operation,
  },
};
use mungos::{
  find::find_collect,
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use periphery_client::{
  api::{self, GetVersionResponse},
  PeripheryClient,
};
use resolver_api::Resolve;
use serror::{serialize_error, serialize_error_pretty};
use tokio_util::sync::CancellationToken;

use crate::{
  cloud::{
    aws::{launch_ec2_instance, terminate_ec2_instance, Ec2Instance},
    BuildCleanupData,
  },
  config::core_config,
  db::db_client,
  helpers::{
    add_update, channel::build_cancel_channel, get_deployment_state,
    make_update, periphery_client, resource::StateResource,
    update_update,
  },
  state::{action_states, State},
};

#[async_trait]
impl Resolve<RunBuild, User> for State {
  #[instrument(name = "RunBuild", skip(self, user))]
  async fn resolve(
    &self,
    RunBuild { build }: RunBuild,
    user: User,
  ) -> anyhow::Result<Update> {
    let mut build = Build::get_resource_check_permissions(
      &build,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    if action_states().build.busy(&build.id).await {
      return Err(anyhow!("build busy"));
    }

    build.config.version.increment();

    let mut update = make_update(&build, Operation::RunBuild, &user);
    update.in_progress();
    update.version = build.config.version.clone();

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
            info!("build cancel acknowledged");
            update.push_simple_log(
              "cancel acknowledged",
              "the build cancellation has been queud, it may still take some time",
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

    let build_id = build.id.clone();

    let inner = || async move {
      update.id = add_update(update.clone()).await?;

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
              serialize_error_pretty(&e),
            ));
            update.finalize();
            update_update(update.clone()).await?;
            return Ok(update);
          }
        };

      // CLONE REPO

      let res = tokio::select! {
        res = periphery
          .request(api::git::CloneRepo {
            args: (&build).into(),
          }) => res,
        _ = cancel.cancelled() => {
          info!("build cancelled during clone, cleaning up builder");
          update.push_error_log("build cancelled", String::from("user cancelled build during repo clone"));
          cleanup_builder_instance(periphery, cleanup_data, &mut update)
            .await;
          info!("builder cleaned up");
          update.finalize();
          update_update(update.clone()).await?;
          return Ok(update)
        },
      };

      match res {
        Ok(clone_logs) => {
          info!("finished repo clone");
          update.logs.extend(clone_logs);
        }
        Err(e) => {
          warn!("failed build at clone repo | {e:#}");
          update.push_error_log("clone repo", serialize_error(&e));
        }
      }

      update_update(update.clone()).await?;

      if all_logs_success(&update.logs) {
        let res = tokio::select! {
          res = periphery
            .request(api::build::Build {
              build: build.clone(),
            }) => res.context("failed at call to periphery to build"),
          _ = cancel.cancelled() => {
            info!("build cancelled during build, cleaning up builder");
            update.push_error_log("build cancelled", String::from("user cancelled build during docker build"));
            cleanup_builder_instance(periphery, cleanup_data, &mut update)
              .await;
            update.finalize();
            update_update(update.clone()).await?;
            return Ok(update)
          },
        };

        match res {
          Ok(logs) => {
            info!("finished build");
            update.logs.extend(logs);
          }
          Err(e) => {
            warn!("error in build | {e:#}");
            update.push_error_log("build", serialize_error(&e))
          }
        };
      }

      update.finalize();

      if update.success {
        let _ = db_client()
          .await
          .builds
          .update_one(
            doc! { "_id": ObjectId::from_str(&build.id)? },
            doc! {
              "$set": {
                "config.version": to_bson(&build.config.version)
                  .context("failed at converting version to bson")?,
                "info.last_built_at": monitor_timestamp(),
              }
            },
            None,
          )
          .await;
      }

      cancel.cancel();

      cleanup_builder_instance(periphery, cleanup_data, &mut update)
        .await;

      info!("builder instance cleaned up");

      update_update(update.clone()).await?;

      if update.success {
        handle_post_build_redeploy(&build.id).await;
        info!("post build redeploy handled");
      }

      Ok(update)
    };

    action_states()
      .build
      .update_entry(&build_id, |entry| {
        entry.building = true;
      })
      .await;

    let res = inner().await;

    action_states()
      .build
      .update_entry(build_id, |entry| {
        entry.building = false;
      })
      .await;

    res
  }
}

#[async_trait]
impl Resolve<CancelBuild, User> for State {
  #[instrument(name = "CancelBuild", skip(self, user))]
  async fn resolve(
    &self,
    CancelBuild { build }: CancelBuild,
    user: User,
  ) -> anyhow::Result<CancelBuildResponse> {
    let build = Build::get_resource_check_permissions(
      &build,
      &user,
      PermissionLevel::Execute,
    )
    .await?;

    // check if theres already an open cancel build update
    if db_client()
      .await
      .updates
      .find_one(
        doc! {
          "operation": "CancelBuild",
          "status": "InProgress",
          "target.id": &build.id,
        },
        None,
      )
      .await
      .context("failed to query updates")?
      .is_some()
    {
      return Err(anyhow!("Build cancel is already in progress"));
    }

    let mut update =
      make_update(&build, Operation::CancelBuild, &user);

    update.push_simple_log(
      "cancel triggered",
      "the build cancel has been triggered",
    );
    update.in_progress();

    update.id =
      add_update(make_update(&build, Operation::CancelBuild, &user))
        .await?;

    build_cancel_channel()
      .sender
      .lock()
      .await
      .send((build.id, update))?;

    Ok(CancelBuildResponse {})
  }
}

const BUILDER_POLL_RATE_SECS: u64 = 2;
const BUILDER_POLL_MAX_TRIES: usize = 30;

#[instrument]
async fn get_build_builder(
  build: &Build,
  update: &mut Update,
) -> anyhow::Result<(PeripheryClient, BuildCleanupData)> {
  if build.config.builder_id.is_empty() {
    return Err(anyhow!("build has not configured a builder"));
  }
  let builder =
    Builder::get_resource(&build.config.builder_id).await?;
  match builder.config {
    BuilderConfig::Server(config) => {
      if config.server_id.is_empty() {
        return Err(anyhow!("builder has not configured a server"));
      }
      let server = Server::get_resource(&config.server_id).await?;
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

#[instrument]
async fn get_aws_builder(
  build: &Build,
  config: AwsBuilderConfig,
  update: &mut Update,
) -> anyhow::Result<(PeripheryClient, BuildCleanupData)> {
  let start_create_ts = monitor_timestamp();

  let instance_name = format!(
    "BUILDER-{}-v{}",
    build.name,
    build.config.version.to_string()
  );
  let Ec2Instance { instance_id, ip } =
    launch_ec2_instance(&instance_name, &config).await?;

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
  let _ = terminate_ec2_instance(config.region, &instance_id).await;

  // Unwrap is safe, only way to get here is after check Ok / early return, so it must be err
  Err(res.err().unwrap())
}

#[instrument(skip(periphery))]
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
      let res = terminate_ec2_instance(region, &instance_id)
        .await
        .context("failed to terminate ec2 instance");
      let log = match res {
        Ok(_) => Log::simple(
          "terminate instance",
          format!("terminate instance id {}", instance_id),
        ),
        Err(e) => {
          Log::error("terminate instance", serialize_error_pretty(&e))
        }
      };
      update.logs.push(log);
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
        if state == DockerContainerState::Running {
          let res = State
            .resolve(
              Deploy {
                deployment: deployment.id.clone(),
                stop_signal: None,
                stop_time: None,
              },
              auto_redeploy_user().to_owned(),
            )
            .await;
          Some((deployment.id.clone(), res))
        } else {
          None
        }
      });

  let redeploy_results = join_all(futures).await;

  let mut redeploys = Vec::<String>::new();
  let mut redeploy_failures = Vec::<String>::new();

  for res in redeploy_results {
    if res.is_none() {
      continue;
    }
    let (id, res) = res.unwrap();
    match res {
      Ok(_) => redeploys.push(id),
      Err(e) => redeploy_failures.push(format!("{id}: {e:#?}")),
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

  format!("instance id: {instance_id}\nip: {ip}\nami id: {ami_id}\ninstance type: {instance_type}\nvolume size: {volume_gb} GB\nsubnet id: {subnet_id}\nsecurity groups: {readable_sec_group_ids}\nassign public ip: {assign_public_ip}\nuse public ip: {use_public_ip}")
}
