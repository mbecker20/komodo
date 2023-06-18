use async_trait::async_trait;
use monitor_types::{
    entities::{server::Server, PermissionLevel},
    requests::api::{
        CreateServer, DeleteServer, GetPeripheryVersion, GetPeripheryVersionResponse, GetServer,
        ListServers, UpdateServer,
    },
};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

#[async_trait]
impl Resolve<GetPeripheryVersion, RequestUser> for State {
    async fn resolve(
        &self,
        GetPeripheryVersion { server_id }: GetPeripheryVersion,
        user: RequestUser,
    ) -> anyhow::Result<GetPeripheryVersionResponse> {
        let server = self
            .get_server_check_permissions(&server_id, &user, PermissionLevel::Read)
            .await?;
        
        todo!()
    }
}

#[async_trait]
impl Resolve<GetServer, RequestUser> for State {
    async fn resolve(&self, req: GetServer, user: RequestUser) -> anyhow::Result<Server> {
        todo!()
    }
}

#[async_trait]
impl Resolve<ListServers, RequestUser> for State {
    async fn resolve(&self, req: ListServers, user: RequestUser) -> anyhow::Result<Vec<Server>> {
        todo!()
    }
}

#[async_trait]
impl Resolve<CreateServer, RequestUser> for State {
    async fn resolve(&self, req: CreateServer, user: RequestUser) -> anyhow::Result<Server> {
        todo!()
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
