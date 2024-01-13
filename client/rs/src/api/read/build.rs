use std::cmp::Ordering;

use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  build::{Build, BuildActionState, BuildListItem, BuildQuery},
  Version, I64,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildResponse)]
pub struct GetBuild {
  pub id: String,
}

#[typeshare]
pub type GetBuildResponse = Build;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListBuildsResponse)]
pub struct ListBuilds {
  #[serde(default)]
  pub query: BuildQuery,
}

#[typeshare]
pub type ListBuildsResponse = Vec<BuildListItem>;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildActionStateResponse)]
pub struct GetBuildActionState {
  pub id: String,
}

#[typeshare]
pub type GetBuildActionStateResponse = BuildActionState;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildsSummaryResponse)]
pub struct GetBuildsSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildsSummaryResponse {
  pub total: u32,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildMonthlyStatsResponse)]
pub struct GetBuildMonthlyStats {
  #[serde(default)]
  pub page: u32,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetBuildMonthlyStatsResponse {
  pub total_time: f64,  // in hours
  pub total_count: f64, // number of builds
  pub days: Vec<BuildStatsDay>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildStatsDay {
  pub time: f64,
  pub count: f64,
  pub ts: f64,
}

impl GetBuildMonthlyStatsResponse {
  pub fn new(
    mut days: Vec<BuildStatsDay>,
  ) -> GetBuildMonthlyStatsResponse {
    days.sort_by(|a, b| {
      if a.ts < b.ts {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    });
    let mut total_time = 0.0;
    let mut total_count = 0.0;
    for day in &days {
      total_time += day.time;
      total_count += day.count;
    }
    GetBuildMonthlyStatsResponse {
      total_time,
      total_count,
      days,
    }
  }
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildVersionsResponse)]
pub struct GetBuildVersions {
  pub id: String,
  #[serde(default)]
  pub page: u32,
  pub major: Option<i32>,
  pub minor: Option<i32>,
  pub patch: Option<i32>,
}

#[typeshare]
pub type GetBuildVersionsResponse = Vec<BuildVersionResponseItem>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildVersionResponseItem {
  pub version: Version,
  pub ts: I64,
}
