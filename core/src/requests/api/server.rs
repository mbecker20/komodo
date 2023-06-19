use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        server::{stats::SystemInformation, Server},
        update::{Log, Update, UpdateStatus, UpdateTarget},
        Operation, PermissionLevel,
    },
    permissioned::Permissioned,
    requests::api::{
        CreateServer, DeleteServer, GetAllSystemStats, GetBasicSystemStats, GetCpuUsage,
        GetDiskUsage, GetNetworkUsage, GetPeripheryVersion, GetPeripheryVersionResponse, GetServer,
        GetSystemComponents, GetSystemInformation, GetSystemProcesses, ListServers, RenameServer,
        UpdateServer,
    },
};
use mungos::mongodb::bson::doc;
use periphery_client::requests;
use resolver_api::{Resolve, ResolveToString};

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetPeripheryVersion, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetPeripheryVersion,
        user: RequestUser,
    ) -> anyhow::Result<GetPeripheryVersionResponse> {
        self.get_server_check_permissions(&req.server_id, &user, PermissionLevel::Read)
            .await?;
        let version = self
            .server_status_cache
            .get(&req.server_id)
            .await
            .map(|s| s.version.clone())
            .unwrap_or(String::from("unknown"));
        Ok(GetPeripheryVersionResponse { version })
    }
}

#[async_trait]
impl Resolve<GetServer, RequestUser> for State {
    async fn resolve(&self, req: GetServer, user: RequestUser) -> anyhow::Result<Server> {
        self.get_server_check_permissions(&req.id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListServers, RequestUser> for State {
    async fn resolve(&self, _: ListServers, user: RequestUser) -> anyhow::Result<Vec<Server>> {
        let servers = self
            .db
            .servers
            .get_some(None, None)
            .await
            .context("failed to pull servers from mongo")?;

        let servers = if user.is_admin {
            servers
        } else {
            servers
                .into_iter()
                .filter(|server| server.get_user_permissions(&user.id) > PermissionLevel::None)
                .collect()
        };

        Ok(servers)
    }
}

#[async_trait]
impl Resolve<CreateServer, RequestUser> for State {
    async fn resolve(&self, req: CreateServer, user: RequestUser) -> anyhow::Result<Server> {
        if !user.is_admin && !user.create_server_permissions {
            return Err(anyhow!("user does not have create server permissions"));
        }
        let start_ts = unix_timestamp_ms() as i64;
        let server = Server {
            id: Default::default(),
            name: req.name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            tags: Default::default(),
            config: req.config.into(),
        };
        let server_id = self
            .db
            .servers
            .create_one(server)
            .await
            .context("failed to add server to db")?;
        let server = self.get_server(&server_id).await?;
        let update = Update {
            target: UpdateTarget::Server(server_id),
            operation: Operation::CreateServer,
            start_ts,
            end_ts: Some(unix_timestamp_ms() as i64),
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        self.add_update(update).await?;

        self.update_cache(&server).await;

        Ok(server)
    }
}

#[async_trait]
impl Resolve<DeleteServer, RequestUser> for State {
    async fn resolve(&self, req: DeleteServer, user: RequestUser) -> anyhow::Result<Server> {
        if self.action_states.server.busy(&req.id).await {
            return Err(anyhow!("server busy"));
        }

        let server = self
            .get_server_check_permissions(&req.id, &user, PermissionLevel::Update)
            .await?;

        let start_ts = unix_timestamp_ms() as i64;

        let mut update = Update {
            target: UpdateTarget::Server(req.id.clone()),
            operation: Operation::DeleteServer,
            start_ts,
            operator: user.id.clone(),
            success: true,
            status: UpdateStatus::InProgress,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        let res = self
            .db
            .servers
            .delete_one(&req.id)
            .await
            .context("failed to delete server from mongo");

        let log = match res {
            Ok(_) => Log::simple("delete server", format!("deleted server {}", server.name)),
            Err(e) => Log::error("delete server", format!("failed to delete server\n{e:#?}")),
        };

        update.end_ts = Some(unix_timestamp_ms() as i64);
        update.status = UpdateStatus::Complete;
        update.success = log.success;
        update.logs.push(log);

        self.update_update(update).await?;

        self.server_status_cache.remove(&req.id).await;

        Ok(server)
    }
}

#[async_trait]
impl Resolve<UpdateServer, RequestUser> for State {
    async fn resolve(&self, req: UpdateServer, user: RequestUser) -> anyhow::Result<Server> {
        todo!()
    }
}

#[async_trait]
impl Resolve<RenameServer, RequestUser> for State {
    async fn resolve(
        &self,
        RenameServer { id, name }: RenameServer,
        user: RequestUser,
    ) -> anyhow::Result<Server> {
        self.get_server_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        self.db
            .updates
            .update_one(
                &id,
                mungos::Update::<Server>::Set(
                    doc! { "name": name, "updated_at": unix_timestamp_ms() as i64 },
                ),
            )
            .await?;
        todo!()
    }
}

#[async_trait]
impl Resolve<GetSystemInformation, RequestUser> for State {
    async fn resolve(
        &self,
        GetSystemInformation { server_id }: GetSystemInformation,
        user: RequestUser,
    ) -> anyhow::Result<SystemInformation> {
        let server = self
            .get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        self.periphery_client(&server)
            .request(requests::GetSystemInformation {})
            .await
    }
}

#[async_trait]
impl ResolveToString<GetAllSystemStats, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetAllSystemStats { server_id }: GetAllSystemStats,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats)?;
        Ok(stats)
    }
}

#[async_trait]
impl ResolveToString<GetBasicSystemStats, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetBasicSystemStats { server_id }: GetBasicSystemStats,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats.basic)?;
        Ok(stats)
    }
}

#[async_trait]
impl ResolveToString<GetCpuUsage, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetCpuUsage { server_id }: GetCpuUsage,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats.cpu)?;
        Ok(stats)
    }
}

#[async_trait]
impl ResolveToString<GetDiskUsage, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetDiskUsage { server_id }: GetDiskUsage,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats.disk)?;
        Ok(stats)
    }
}

#[async_trait]
impl ResolveToString<GetNetworkUsage, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetNetworkUsage { server_id }: GetNetworkUsage,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats.network)?;
        Ok(stats)
    }
}

#[async_trait]
impl ResolveToString<GetSystemProcesses, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetSystemProcesses { server_id }: GetSystemProcesses,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats.processes)?;
        Ok(stats)
    }
}

#[async_trait]
impl ResolveToString<GetSystemComponents, RequestUser> for State {
    async fn resolve_to_string(
        &self,
        GetSystemComponents { server_id }: GetSystemComponents,
        user: RequestUser,
    ) -> anyhow::Result<String> {
        self.get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&server_id)
            .await
            .ok_or(anyhow!("did not find status for server at {server_id}"))?;
        let stats = status
            .stats
            .as_ref()
            .ok_or(anyhow!("server not reachable"))?;
        let stats = serde_json::to_string(&stats.components)?;
        Ok(stats)
    }
}
