use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        server::{
            stats::{
                AllSystemStats, BasicSystemStats, CpuUsage, DiskUsage, NetworkUsage,
                SystemComponent, SystemInformation, SystemProcess,
            },
            Server, ServerBuilder,
        },
        update::{Update, UpdateTarget},
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
use resolver_api::Resolve;

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
    async fn resolve(&self, req: DeleteServer, user: RequestUser) -> anyhow::Result<()> {
        todo!()
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
    async fn resolve(&self, req: RenameServer, args: RequestUser) -> anyhow::Result<Server> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetSystemInformation, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetSystemInformation,
        args: RequestUser,
    ) -> anyhow::Result<SystemInformation> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetAllSystemStats, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetAllSystemStats,
        args: RequestUser,
    ) -> anyhow::Result<AllSystemStats> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetBasicSystemStats, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetBasicSystemStats,
        args: RequestUser,
    ) -> anyhow::Result<BasicSystemStats> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetCpuUsage, RequestUser> for State {
    async fn resolve(&self, req: GetCpuUsage, args: RequestUser) -> anyhow::Result<CpuUsage> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetDiskUsage, RequestUser> for State {
    async fn resolve(&self, req: GetDiskUsage, args: RequestUser) -> anyhow::Result<DiskUsage> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetNetworkUsage, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetNetworkUsage,
        args: RequestUser,
    ) -> anyhow::Result<NetworkUsage> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetSystemProcesses, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetSystemProcesses,
        args: RequestUser,
    ) -> anyhow::Result<Vec<SystemProcess>> {
        todo!()
    }
}

#[async_trait]
impl Resolve<GetSystemComponents, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetSystemComponents,
        args: RequestUser,
    ) -> anyhow::Result<Vec<SystemComponent>> {
        todo!()
    }
}
