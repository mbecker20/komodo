use std::time::Duration;

use anyhow::{anyhow, Context};
use diff::Diff;
use helpers::{
    all_logs_success,
    aws::{self, create_ec2_client, create_instance_with_ami, terminate_ec2_instance, Ec2Instance},
    to_monitor_name,
};
use mungos::{doc, to_bson};
use types::{
    monitor_timestamp,
    traits::{Busy, Permissioned},
    Build, Log, Operation, PermissionLevel, Update, UpdateStatus, UpdateTarget, Version,
};

use crate::{auth::RequestUser, state::State};

const BUILDER_POLL_RATE_SECS: u64 = 2;
const BUILDER_POLL_MAX_TRIES: usize = 30;

impl State {
    pub async fn get_build_check_permissions(
        &self,
        build_id: &str,
        user: &RequestUser,
        permission_level: PermissionLevel,
    ) -> anyhow::Result<Build> {
        let build = self.db.get_build(build_id).await?;
        let permissions = build.get_user_permissions(&user.id);
        if user.is_admin || permissions >= permission_level {
            Ok(build)
        } else {
            Err(anyhow!(
                "user does not have required permissions on this build"
            ))
        }
    }

    pub async fn build_busy(&self, id: &str) -> bool {
        match self.build_action_states.lock().await.get(id) {
            Some(a) => a.busy(),
            None => false,
        }
    }

    pub async fn create_build(&self, name: &str, user: &RequestUser) -> anyhow::Result<Build> {
        if !user.is_admin && !user.create_build_permissions {
            return Err(anyhow!("user does not have permission to create builds"));
        }
        let start_ts = monitor_timestamp();
        let build = Build {
            name: to_monitor_name(name),
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            last_built_at: "never".to_string(),
            created_at: start_ts.clone(),
            updated_at: start_ts.clone(),
            ..Default::default()
        };
        let build_id = self
            .db
            .builds
            .create_one(build)
            .await
            .context("failed at adding build to db")?;
        let build = self.db.get_build(&build_id).await?;
        let update = Update {
            target: UpdateTarget::Build(build_id),
            operation: Operation::CreateBuild,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(build)
    }

    pub async fn create_full_build(
        &self,
        mut build: Build,
        user: &RequestUser,
    ) -> anyhow::Result<Build> {
        build.id = self.create_build(&build.name, user).await?.id;
        let build = self.update_build(build, user).await?;
        Ok(build)
    }

    pub async fn copy_build(
        &self,
        target_id: &str,
        new_name: String,
        user: &RequestUser,
    ) -> anyhow::Result<Build> {
        let mut build = self
            .get_build_check_permissions(target_id, user, PermissionLevel::Update)
            .await?;
        build.name = new_name;
        build.version = Version::default();
        let build = self.create_full_build(build, user).await?;
        Ok(build)
    }

    pub async fn delete_build(&self, build_id: &str, user: &RequestUser) -> anyhow::Result<Build> {
        if self.build_busy(build_id).await {
            return Err(anyhow!("build busy"));
        }
        let build = self
            .get_build_check_permissions(build_id, user, PermissionLevel::Update)
            .await?;
        let start_ts = monitor_timestamp();
        self.db.builds.delete_one(build_id).await?;
        let update = Update {
            target: UpdateTarget::Build(build_id.to_string()),
            operation: Operation::DeleteBuild,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            logs: vec![Log::simple(
                "delete build",
                format!("deleted build {}", build.name),
            )],
            success: true,
            ..Default::default()
        };
        self.add_update(update).await?;
        Ok(build)
    }

    pub async fn update_build(
        &self,
        new_build: Build,
        user: &RequestUser,
    ) -> anyhow::Result<Build> {
        if self.build_busy(&new_build.id).await {
            return Err(anyhow!("build busy"));
        }
        let id = new_build.id.clone();
        {
            let mut lock = self.build_action_states.lock().await;
            let entry = lock.entry(id.clone()).or_default();
            entry.updating = true;
        }
        let res = self.update_build_inner(new_build, user).await;
        {
            let mut lock = self.build_action_states.lock().await;
            let entry = lock.entry(id).or_default();
            entry.updating = false;
        }
        res
    }

    async fn update_build_inner(
        &self,
        mut new_build: Build,
        user: &RequestUser,
    ) -> anyhow::Result<Build> {
        let start_ts = monitor_timestamp();
        let current_build = self
            .get_build_check_permissions(&new_build.id, user, PermissionLevel::Update)
            .await?;

        if let Some(new_server_id) = &new_build.server_id {
            if current_build.server_id.is_none()
                || new_server_id != current_build.server_id.as_ref().unwrap()
            {
                self.get_server_check_permissions(new_server_id, user, PermissionLevel::Update)
                    .await
                    .context("user does not have permission to attach build to this server")?;
            }
        }

        // none of these should be changed through this method
        new_build.name = current_build.name.clone();
        new_build.permissions = current_build.permissions.clone();
        new_build.last_built_at = current_build.last_built_at.clone();
        new_build.created_at = current_build.created_at.clone();
        new_build.updated_at = start_ts.clone();

        self.db
            .builds
            .update_one(&new_build.id, mungos::Update::Regular(new_build.clone()))
            .await
            .context("failed at update one build")?;

        let diff = current_build.diff(&new_build);

        let update = Update {
            operation: Operation::UpdateBuild,
            target: UpdateTarget::Build(new_build.id.clone()),
            start_ts,
            status: UpdateStatus::Complete,
            logs: vec![Log::simple(
                "build update",
                serde_json::to_string_pretty(&diff).unwrap(),
            )],
            operator: user.id.clone(),
            end_ts: Some(monitor_timestamp()),
            success: true,
            ..Default::default()
        };

        // update.id = self.add_update(update.clone()).await?;

        // if any_option_diff_is_some(&[&diff.repo, &diff.branch, &diff.github_account])
        //     || option_diff_is_some(&diff.on_clone)
        // {
        //     let server = self.db.get_server(&current_build.server_id).await?;
        //     match self.periphery.clone_repo(&server, &new_build).await {
        //         Ok(clone_logs) => {
        //             update.logs.extend(clone_logs);
        //         }
        //         Err(e) => update
        //             .logs
        //             .push(Log::error("cloning repo", format!("{e:#?}"))),
        //     }
        // }

        // update.end_ts = Some(monitor_timestamp());
        // update.success = all_logs_success(&update.logs);
        // update.status = UpdateStatus::Complete;

        self.add_update(update).await?;

        Ok(new_build)
    }

    pub async fn build(&self, build_id: &str, user: &RequestUser) -> anyhow::Result<Update> {
        if self.build_busy(build_id).await {
            return Err(anyhow!("build busy"));
        }
        {
            let mut lock = self.build_action_states.lock().await;
            let entry = lock.entry(build_id.to_string()).or_default();
            entry.building = true;
        }
        let res = self.build_inner(build_id, user).await;
        {
            let mut lock = self.build_action_states.lock().await;
            let entry = lock.entry(build_id.to_string()).or_default();
            entry.building = false;
        }
        res
    }

    async fn build_inner(&self, build_id: &str, user: &RequestUser) -> anyhow::Result<Update> {
        let mut build = self
            .get_build_check_permissions(build_id, user, PermissionLevel::Update)
            .await?;
        build.version.increment();
        let mut update = Update {
            target: UpdateTarget::Build(build_id.to_string()),
            operation: Operation::BuildBuild,
            start_ts: monitor_timestamp(),
            status: UpdateStatus::InProgress,
            operator: user.id.clone(),
            success: true,
            version: build.version.clone().into(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;

        let (server, aws_client) = if let Some(server_id) = &build.server_id {
            let server = self.db.get_server(server_id).await;
            if let Err(e) = server {
                update.status = UpdateStatus::Complete;
                update.end_ts = Some(monitor_timestamp());
                update.success = false;
                update
                    .logs
                    .push(Log::error("get build server", format!("{e:#?}")));
                self.update_update(update.clone()).await?;
                return Err(e);
            }
            let server = Ec2Instance {
                instance_id: String::new(),
                server: server.unwrap(),
            };
            (server, None)
        } else if build.aws_config.is_some() {
            let start_ts = monitor_timestamp();
            let res = self.create_ec2_instance_for_build(&build).await;
            if let Err(e) = res {
                update.status = UpdateStatus::Complete;
                update.end_ts = Some(monitor_timestamp());
                update.success = false;
                update.logs.push(Log {
                    stage: "start build server".to_string(),
                    stderr: format!("{e:#?}"),
                    success: false,
                    start_ts,
                    end_ts: monitor_timestamp(),
                    ..Default::default()
                });
                self.update_update(update).await?;
                return Err(e);
            }
            let (server, aws_client, logs) = res.unwrap();
            update.logs.extend(logs);
            self.update_update(update.clone()).await?;
            (server, aws_client)
        } else {
            update.status = UpdateStatus::Complete;
            update.end_ts = Some(monitor_timestamp());
            update.success = false;
            update.logs.push(Log::error(
                "start build",
                "build has neither server_id nor aws_config attached".to_string(),
            ));
            self.update_update(update).await?;
            return Err(anyhow!(
                "build has neither server_id or aws_config attached"
            ));
        };

        let clone_success = match self.periphery.clone_repo(&server.server, &build).await {
            Ok(clone_logs) => {
                update.logs.extend(clone_logs);
                true
            }
            Err(e) => {
                update
                    .logs
                    .push(Log::error("clone repo", format!("{e:#?}")));
                false
            }
        };

        if !clone_success {
            let _ = self
                .periphery
                .delete_repo(&server.server, &build.name)
                .await;
            if let Some(aws_client) = aws_client {
                self.terminate_ec2_instance(aws_client, &server, &mut update)
                    .await;
            }
            update.status = UpdateStatus::Complete;
            update.end_ts = Some(monitor_timestamp());
            update.success = false;
            self.update_update(update.clone()).await?;
            return Ok(update);
        }

        self.update_update(update.clone()).await?;

        let build_logs = match self
            .periphery
            .build(&server.server, &build)
            .await
            .context("failed at call to periphery to build")
        {
            Ok(logs) => logs,
            Err(e) => Some(vec![Log::error("build", format!("{e:#?}"))]),
        };

        match build_logs {
            Some(logs) => {
                let success = all_logs_success(&logs);
                update.logs.extend(logs);
                if success {
                    let _ = self
                        .db
                        .builds
                        .update_one::<Build>(
                            build_id,
                            mungos::Update::Set(doc! {
                                "version": to_bson(&build.version)
                                    .context("failed at converting version to bson")?,
                                "last_built_at": monitor_timestamp(),
                            }),
                        )
                        .await;
                }
            }
            None => {
                update
                    .logs
                    .push(Log::error("build", "builder busy".to_string()));
            }
        }

        let _ = self
            .periphery
            .delete_repo(&server.server, &build.name)
            .await;

        if let Some(aws_client) = aws_client {
            self.terminate_ec2_instance(aws_client, &server, &mut update)
                .await;
        }

        update.success = all_logs_success(&update.logs);
        update.status = UpdateStatus::Complete;
        update.end_ts = Some(monitor_timestamp());

        self.update_update(update.clone()).await?;

        Ok(update)
    }

    async fn create_ec2_instance_for_build(
        &self,
        build: &Build,
    ) -> anyhow::Result<(Ec2Instance, Option<aws::Client>, Vec<Log>)> {
        if build.aws_config.is_none() {
            return Err(anyhow!("build has no aws_config attached"));
        }
        let start_instance_ts = monitor_timestamp();
        let aws_config = build.aws_config.as_ref().unwrap();
        let region = aws_config
            .region
            .as_ref()
            .unwrap_or(&self.config.aws.default_region)
            .to_string();
        let aws_client = create_ec2_client(
            region,
            &self.config.aws.access_key_id,
            self.config.aws.secret_access_key.clone(),
        )
        .await;
        let ami_id = aws_config
            .ami_id
            .as_ref()
            .unwrap_or(&self.config.aws.default_ami_id);
        let instance_type = aws_config
            .instance_type
            .as_ref()
            .unwrap_or(&self.config.aws.default_instance_type);
        let subnet_id = aws_config
            .subnet_id
            .as_ref()
            .unwrap_or(&self.config.aws.default_subnet_id);
        let security_group_ids = aws_config
            .security_group_ids
            .as_ref()
            .unwrap_or(&self.config.aws.default_security_group_ids)
            .to_owned();
        let readable_sec_group_ids = security_group_ids.join(", ");
        let volume_size_gb = *aws_config
            .volume_gb
            .as_ref()
            .unwrap_or(&self.config.aws.default_volume_gb);
        let key_pair_name = aws_config
            .key_pair_name
            .as_ref()
            .unwrap_or(&self.config.aws.default_key_pair_name);
        let assign_public_ip = *aws_config
            .assign_public_ip
            .as_ref()
            .unwrap_or(&self.config.aws.default_assign_public_ip);
        let instance = create_instance_with_ami(
            &aws_client,
            &format!("BUILDER-{}-v{}", build.name, build.version.to_string()),
            ami_id,
            instance_type,
            subnet_id,
            security_group_ids,
            volume_size_gb,
            key_pair_name,
            assign_public_ip,
        )
        .await?;
        let instance_id = &instance.instance_id;
        let start_log = Log {
            stage: "start build instance".to_string(),
            success: true,
            stdout: format!("instance id: {instance_id}\nami id: {ami_id}\ninstance type: {instance_type}\nvolume size: {volume_size_gb} GB\nsubnet id: {subnet_id}\nsecurity groups: {readable_sec_group_ids}"),
            start_ts: start_instance_ts,
            end_ts: monitor_timestamp(),
            ..Default::default()
        };
        let start_connect_ts = monitor_timestamp();
        let mut res = Ok(String::new());
        for _ in 0..BUILDER_POLL_MAX_TRIES {
            let status = self.periphery.health_check(&instance.server).await;
            if let Ok(_) = status {
                let connect_log = Log {
                    stage: "build instance connected".to_string(),
                    success: true,
                    stdout: "established contact with periphery on builder".to_string(),
                    start_ts: start_connect_ts,
                    end_ts: monitor_timestamp(),
                    ..Default::default()
                };
                return Ok((instance, Some(aws_client), vec![start_log, connect_log]));
            }
            res = status;
            tokio::time::sleep(Duration::from_secs(BUILDER_POLL_RATE_SECS)).await;
        }
        let _ = terminate_ec2_instance(&aws_client, &instance.instance_id).await;
        Err(anyhow!(
            "unable to reach periphery agent on build server\n{res:#?}"
        ))
    }

    async fn terminate_ec2_instance(
        &self,
        aws_client: aws::Client,
        server: &Ec2Instance,
        update: &mut Update,
    ) {
        let res = terminate_ec2_instance(&aws_client, &server.instance_id).await;
        if let Err(e) = res {
            update
                .logs
                .push(Log::error("terminate instance", format!("{e:#?}")))
        } else {
            update.logs.push(Log::simple(
                "terminate instance",
                format!("terminate instance id {}", server.instance_id),
            ))
        }
    }

    // pub async fn reclone_build(
    //     &self,
    //     build_id: &str,
    //     user: &RequestUser,
    // ) -> anyhow::Result<Update> {
    //     if self.build_busy(build_id).await {
    //         return Err(anyhow!("build busy"));
    //     }
    //     {
    //         let mut lock = self.build_action_states.lock().await;
    //         let entry = lock.entry(build_id.to_string()).or_default();
    //         entry.recloning = true;
    //     }
    //     let res = self.reclone_build_inner(build_id, user).await;
    //     {
    //         let mut lock = self.build_action_states.lock().await;
    //         let entry = lock.entry(build_id.to_string()).or_default();
    //         entry.recloning = false;
    //     }
    //     res
    // }

    // async fn reclone_build_inner(
    //     &self,
    //     build_id: &str,
    //     user: &RequestUser,
    // ) -> anyhow::Result<Update> {
    //     let build = self
    //         .get_build_check_permissions(build_id, user, PermissionLevel::Update)
    //         .await?;
    //     let server = self.db.get_server(&build.server_id).await?;
    //     let mut update = Update {
    //         target: UpdateTarget::Build(build_id.to_string()),
    //         operation: Operation::RecloneBuild,
    //         start_ts: monitor_timestamp(),
    //         status: UpdateStatus::InProgress,
    //         operator: user.id.clone(),
    //         success: true,
    //         ..Default::default()
    //     };
    //     update.id = self.add_update(update.clone()).await?;

    //     update.success = match self.periphery.clone_repo(&server, &build).await {
    //         Ok(clone_logs) => {
    //             update.logs.extend(clone_logs);
    //             true
    //         }
    //         Err(e) => {
    //             update
    //                 .logs
    //                 .push(Log::error("clone repo", format!("{e:#?}")));
    //             false
    //         }
    //     };

    //     update.status = UpdateStatus::Complete;
    //     update.end_ts = Some(monitor_timestamp());

    //     self.update_update(update.clone()).await?;

    //     Ok(update)
    // }
}
