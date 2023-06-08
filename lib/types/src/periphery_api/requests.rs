use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::impl_has_response;

// GET HEALTH

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealth {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

impl_has_response!(GetHealth, GetHealthResponse);

// GET VERSION

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

impl_has_response!(GetVersion, GetVersionResponse);

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemInformation {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemInformationResponse {
    pub version: String,
}

impl_has_response!(GetSystemInformation, GetSystemInformationResponse);
