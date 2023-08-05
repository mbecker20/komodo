use anyhow::{anyhow, Context};
use async_timing_util::{get_timelength_in_ms, unix_timestamp_ms};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        deployment::ContainerSummary,
        server::{
            docker_image::ImageSummary, docker_network::DockerNetwork, stats::SystemInformation,
            Server, ServerActionState, ServerStatus,
        },
        PermissionLevel,
    },
    requests::read::*,
};
use mungos::mongodb::{bson::doc, options::FindOptions};
use periphery_client::requests;
use resolver_api::{Resolve, ResolveToString};

use crate::{auth::RequestUser, resource::Resource, state::State};

#[async_trait]
impl Resolve<GetServersSummary, RequestUser> for State {
    async fn resolve(
        &self,
        GetServersSummary {}: GetServersSummary,
        user: RequestUser,
    ) -> anyhow::Result<GetServersSummaryResponse> {
        let servers =
            <State as Resource<Server>>::list_resources_for_user(self, None, &user).await?;
        let mut res = GetServersSummaryResponse::default();
        for server in servers {
            res.total += 1;
            match server.status {
                ServerStatus::Ok => {
                    res.healthy += 1;
                }
                ServerStatus::NotOk => {
                    res.unhealthy += 1;
                }
                ServerStatus::Disabled => {
                    res.disabled += 1;
                }
            }
        }
        Ok(res)
    }
}

#[async_trait]
impl Resolve<GetPeripheryVersion, RequestUser> for State {
    async fn resolve(
        &self,
        req: GetPeripheryVersion,
        user: RequestUser,
    ) -> anyhow::Result<GetPeripheryVersionResponse> {
        let _: Server = self
            .get_resource_check_permissions(&req.server_id, &user, PermissionLevel::Read)
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
        self.get_resource_check_permissions(&req.id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListServers, RequestUser> for State {
    async fn resolve(
        &self,
        ListServers { query }: ListServers,
        user: RequestUser,
    ) -> anyhow::Result<Vec<ServerListItem>> {
        <State as Resource<Server>>::list_resources_for_user(self, query, &user).await
    }
}

#[async_trait]
impl Resolve<GetServerStatus, RequestUser> for State {
    async fn resolve(
        &self,
        GetServerStatus { id }: GetServerStatus,
        user: RequestUser,
    ) -> anyhow::Result<GetServerStatusResponse> {
        let _: Server = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .server_status_cache
            .get(&id)
            .await
            .ok_or(anyhow!("did not find cached status for server"))?;
        let response = GetServerStatusResponse {
            status: status.status,
        };
        Ok(response)
    }
}

#[async_trait]
impl Resolve<GetServerActionState, RequestUser> for State {
    async fn resolve(
        &self,
        GetServerActionState { id }: GetServerActionState,
        user: RequestUser,
    ) -> anyhow::Result<ServerActionState> {
        let _: Server = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Read)
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
        let server: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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

const STATS_PER_PAGE: i64 = 500;

#[async_trait]
impl Resolve<GetHistoricalServerStats, RequestUser> for State {
    async fn resolve(
        &self,
        GetHistoricalServerStats {
            server_id,
            interval,
            page,
        }: GetHistoricalServerStats,
        user: RequestUser,
    ) -> anyhow::Result<GetHistoricalServerStatsResponse> {
        let _: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        let interval = get_timelength_in_ms(interval.to_string().parse().unwrap()) as i64;
        let mut ts_vec = Vec::<i64>::new();
        let curr_ts = unix_timestamp_ms() as i64;
        let mut curr_ts = curr_ts - curr_ts % interval - interval * STATS_PER_PAGE * page as i64;
        for _ in 0..STATS_PER_PAGE {
            ts_vec.push(curr_ts);
            curr_ts -= interval;
        }
        let stats = self
            .db
            .stats
            .get_some(
                doc! {
                    "sid": server_id,
                    "ts": { "$in": ts_vec },
                },
                FindOptions::builder()
                    .sort(doc! { "ts": -1 })
                    .skip(page as u64 * STATS_PER_PAGE as u64)
                    .limit(STATS_PER_PAGE)
                    .build(),
            )
            .await
            .context("failed to pull stats from db")?;
        let next_page = if stats.len() == STATS_PER_PAGE as usize {
            Some(page + 1)
        } else {
            None
        };
        let res = GetHistoricalServerStatsResponse { stats, next_page };
        Ok(res)
    }
}

#[async_trait]
impl Resolve<GetDockerImages, RequestUser> for State {
    async fn resolve(
        &self,
        GetDockerImages { server_id }: GetDockerImages,
        user: RequestUser,
    ) -> anyhow::Result<Vec<ImageSummary>> {
        let server: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
        let server: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
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
    ) -> anyhow::Result<Vec<ContainerSummary>> {
        let server: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        self.periphery_client(&server)
            .request(requests::GetContainerList {})
            .await
    }
}
