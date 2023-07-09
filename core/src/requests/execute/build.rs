use std::time::Duration;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_types::{
    all_logs_success,
    entities::{
        build::{Build, BuildBuilderConfig},
        builder::{AwsBuilderConfig, BuilderConfig},
        deployment::DockerContainerState,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    requests::execute::{Deploy, RunBuild},
};
use mungos::mongodb::bson::{doc, to_bson};
use periphery_client::{
    requests::{self, GetVersionResponse},
    PeripheryClient,
};
use resolver_api::Resolve;

use crate::{
    auth::{InnerRequestUser, RequestUser},
    cloud::{aws::Ec2Instance, BuildCleanupData},
    state::State,
};

#[async_trait]
impl Resolve<RunBuild, RequestUser> for State {
    async fn resolve(
        &self,
        RunBuild { build_id }: RunBuild,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if self.action_states.build.busy(&build_id).await {
            return Err(anyhow!("build busy"));
        }

        let mut build = self
            .get_build_check_permissions(&build_id, &user, PermissionLevel::Execute)
            .await?;

        let inner = || async move {
            build.config.version.increment();
            let mut update = Update {
                target: ResourceTarget::Build(build.id.clone()),
                operation: Operation::RunBuild,
                start_ts: monitor_timestamp(),
                status: UpdateStatus::InProgress,
                success: true,
                operator: user.id.clone(),
                version: build.config.version.clone(),
                ..Default::default()
            };
            update.id = self.add_update(update.clone()).await?;

            // GET BUILDER PERIPHERY

            let builder = self.get_build_builder(&build, &mut update).await;

            if let Err(e) = &builder {
                update
                    .logs
                    .push(Log::error("get builder", format!("{e:#?}")));
                update.finalize();
                self.update_update(update.clone()).await?;
                return Ok(update);
            }

            let (periphery, cleanup_data) = builder.unwrap();

            // CLONE REPO

            let clone_success = match periphery
                .request(requests::CloneRepo {
                    args: (&build).into(),
                })
                .await
            {
                Ok(clone_logs) => {
                    let success = all_logs_success(&clone_logs);
                    update.logs.extend(clone_logs);
                    success
                }
                Err(e) => {
                    update
                        .logs
                        .push(Log::error("clone repo", format!("{e:#?}")));
                    false
                }
            };

            if clone_success {
                match periphery
                    .request(requests::Build {
                        build: build.clone(),
                    })
                    .await
                    .context("failed at call to periphery to build")
                {
                    Ok(logs) => update.logs.extend(logs),
                    Err(e) => update.logs.push(Log::error("build", format!("{e:#?}"))),
                };
            }

            if all_logs_success(&update.logs) {
                let _ = self
                    .db
                    .builds
                    .update_one(
                        &build.id,
                        mungos::Update::Set(doc! {
                            "version": to_bson(&build.config.version)
                                .context("failed at converting version to bson")?,
                            "last_built_at": monitor_timestamp(),
                        }),
                    )
                    .await;
            }

            // CLEANUP AND FINALIZE UPDATE

            self.cleanup_builder_instance(periphery, cleanup_data, &mut update)
                .await;

            self.handle_post_build_redeploy(&build.id, &mut update)
                .await;

            update.finalize();

            self.update_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .build
            .update_entry(build_id.clone(), |entry| {
                entry.building = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .build
            .update_entry(build_id, |entry| {
                entry.building = false;
            })
            .await;

        res
    }
}

const BUILDER_POLL_RATE_SECS: u64 = 2;
const BUILDER_POLL_MAX_TRIES: usize = 30;

impl State {
    async fn get_build_builder(
        &self,
        build: &Build,
        update: &mut Update,
    ) -> anyhow::Result<(PeripheryClient, BuildCleanupData)> {
        match &build.config.builder {
            BuildBuilderConfig::Server { server_id } => {
                if server_id.is_empty() {
                    return Err(anyhow!("build has not configured a builder"));
                }
                let server = self.get_server(server_id).await?;
                let periphery = self.periphery_client(&server);
                Ok((
                    periphery,
                    BuildCleanupData::Server {
                        repo_name: build.name.clone(),
                    },
                ))
            }
            BuildBuilderConfig::Builder { builder_id } => {
                if builder_id.is_empty() {
                    return Err(anyhow!("build has not configured a builder"));
                }
                let builder = self.get_builder(builder_id).await?;
                match builder.config {
                    BuilderConfig::Aws(config) => self.get_aws_builder(build, config, update).await,
                }
            }
        }
    }

    async fn get_aws_builder(
        &self,
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
            self.create_ec2_instance(&instance_name, &config).await?;

        let readable_sec_group_ids = config.security_group_ids.join(", ");
        let AwsBuilderConfig {
            ami_id,
            instance_type,
            volume_gb,
            subnet_id,
            ..
        } = config;

        let log = Log {
            stage: "start build instance".to_string(),
            success: true,
            stdout: format!("instance id: {instance_id}\nami id: {ami_id}\ninstance type: {instance_type}\nvolume size: {volume_gb} GB\nsubnet id: {subnet_id}\nsecurity groups: {readable_sec_group_ids}"),
            start_ts: start_create_ts,
            end_ts: monitor_timestamp(),
            ..Default::default()
        };

        update.logs.push(log);

        self.update_update(update.clone()).await?;

        let periphery = PeripheryClient::new(format!("http://{ip}:8000"), &self.config.passkey);

        let start_connect_ts = monitor_timestamp();
        let mut res = Ok(GetVersionResponse {
            version: String::new(),
        });
        for _ in 0..BUILDER_POLL_MAX_TRIES {
            let version = periphery
                .request(requests::GetVersion {})
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
                self.update_update(update.clone()).await?;
                return Ok((
                    periphery,
                    BuildCleanupData::Aws {
                        instance_id,
                        region: config.region,
                    },
                ));
            }
            res = version;
            tokio::time::sleep(Duration::from_secs(BUILDER_POLL_RATE_SECS)).await;
        }
        Err(anyhow!("{:#?}", res.err().unwrap()))
    }

    async fn cleanup_builder_instance(
        &self,
        periphery: PeripheryClient,
        cleanup_data: BuildCleanupData,
        update: &mut Update,
    ) {
        match cleanup_data {
            BuildCleanupData::Server { repo_name } => {
                let _ = periphery
                    .request(requests::DeleteRepo { name: repo_name })
                    .await;
            }
            BuildCleanupData::Aws {
                instance_id,
                region,
            } => {
                let res = self
                    .terminate_ec2_instance(region, &instance_id)
                    .await
                    .context("failed to terminate ec2 instance");
                let log = match res {
                    Ok(_) => Log::simple(
                        "terminate instance",
                        format!("terminate instance id {}", instance_id),
                    ),
                    Err(e) => Log::error("terminate instance", format!("{e:#?}")),
                };
                update.logs.push(log);
            }
        }
    }

    async fn handle_post_build_redeploy(&self, build_id: &str, update: &mut Update) {
        let redeploy_deployments = self
            .db
            .deployments
            .get_some(
                doc! { "build_id": build_id, "redeploy_on_build": true },
                None,
            )
            .await;

        if let Ok(deployments) = redeploy_deployments {
            let futures = deployments.into_iter().map(|deployment| async move {
                let request_user: RequestUser = InnerRequestUser {
                    id: "auto redeploy".to_string(),
                    is_admin: true,
                    ..Default::default()
                }
                .into();
                let state = self
                    .get_deployment_state(&deployment)
                    .await
                    .unwrap_or_default();
                if state == DockerContainerState::Running {
                    let res = self
                        .resolve(
                            Deploy {
                                deployment_id: deployment.id.clone(),
                                stop_signal: None,
                                stop_time: None,
                            },
                            request_user,
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

            if !redeploys.is_empty() {
                update.logs.push(Log::simple(
                    "redeploy",
                    format!("redeployed deployments: {}", redeploys.join(", ")),
                ))
            }

            if !redeploy_failures.is_empty() {
                update.logs.push(Log::simple(
                    "redeploy failures",
                    redeploy_failures.join("\n"),
                ))
            }
        } else if let Err(e) = redeploy_deployments {
            update.logs.push(Log::simple(
                "redeploys failed",
                format!("failed to get deployments to redeploy: {e:#?}"),
            ))
        }
    }
}
