use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        deployment::BasicContainerInfo,
        server::{
            docker_image::ImageSummary, docker_network::DockerNetwork, stats::SystemInformation,
            Server, ServerActionState,
        },
        PermissionLevel,
    },
    permissioned::Permissioned,
    requests::read::*,
};
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
    async fn resolve(
        &self,
        ListServers { query }: ListServers,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Server>> {
        let servers = self
            .db
            .servers
            .get_some(query, None)
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
impl Resolve<GetServerActionState, RequestUser> for State {
    async fn resolve(
        &self,
        GetServerActionState { id }: GetServerActionState,
        user: RequestUser,
    ) -> anyhow::Result<ServerActionState> {
        self.get_server_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self.action_states.server.get(&id).await.unwrap_or_default();
        Ok(action_state)
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

#[async_trait]
impl Resolve<GetDockerImages, RequestUser> for State {
    async fn resolve(
        &self,
        GetDockerImages { server_id }: GetDockerImages,
        user: RequestUser,
    ) -> anyhow::Result<Vec<ImageSummary>> {
        let server = self
            .get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        self.periphery_client(&server)
            .request(requests::GetImageList {})
            .await
    }
}

#[async_trait]
impl Resolve<GetDockerNetworks, RequestUser> for State {
    async fn resolve(
        &self,
        GetDockerNetworks { server_id }: GetDockerNetworks,
        user: RequestUser,
    ) -> anyhow::Result<Vec<DockerNetwork>> {
        let server = self
            .get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        self.periphery_client(&server)
            .request(requests::GetNetworkList {})
            .await
    }
}

#[async_trait]
impl Resolve<GetDockerContainers, RequestUser> for State {
    async fn resolve(
        &self,
        GetDockerContainers { server_id }: GetDockerContainers,
        user: RequestUser,
    ) -> anyhow::Result<Vec<BasicContainerInfo>> {
        let server = self
            .get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        self.periphery_client(&server)
            .request(requests::GetContainerList {})
            .await
    }
}
