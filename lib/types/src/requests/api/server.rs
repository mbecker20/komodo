use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::{entities::server::{Server, PartialServerConfig}, MongoDocument};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetPeripheryVersionResponse)]
pub struct GetPeripheryVersion {
	pub server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPeripheryVersionResponse {
	pub version: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Server)]
pub struct GetServer {
	pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Server>)]
pub struct ListServers {
	pub query: Option<MongoDocument>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Server)]
pub struct CreateServer {
	pub name: String,
	pub config: PartialServerConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(())]
pub struct DeleteServer {
	pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Server)]
pub struct UpdateServer {
	pub config: PartialServerConfig,
}

