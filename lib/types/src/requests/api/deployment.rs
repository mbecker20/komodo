//

use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::{MongoDocument, entities::{deployment::{Deployment, PartialDeploymentConfig}, update::Update}};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct GetDeployment {
	pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Deployment>)]
pub struct ListDeployments {
	pub query: Option<MongoDocument>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct CreateDeployment {
	pub name: String,
	pub config: PartialDeploymentConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct DeleteDeployment {
	pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct UpdateDeployment {
	pub id: String,
	pub config: PartialDeploymentConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RenameDeployment {
	pub id: String,
	pub name: String,
}