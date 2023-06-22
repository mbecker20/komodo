//

use monitor_macros::derive_crud_requests;
use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::{MongoDocument, entities::{deployment::{Deployment, PartialDeploymentConfig}, update::Update}};

//

derive_crud_requests!(Deployment);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RenameDeployment {
	pub id: String,
	pub name: String,
}