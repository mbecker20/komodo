use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::KomodoExecuteRequest;

//

/// Clones the target repo. Response: [Update].
///
/// Note. Repo must have server attached at `server_id`.
///
/// 1. Clones the repo on the target server using `git clone https://{$token?}@github.com/${repo} -b ${branch}`.
/// The token will only be used if a github account is specified,
/// and must be declared in the periphery configuration on the target server.
/// 2. If `on_clone` and `on_pull` are specified, they will be executed.
/// `on_clone` will be executed before `on_pull`.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
pub struct CloneRepo {
  /// Id or name
  pub repo: String,
}

//

/// Pulls the target repo. Response: [Update].
///
/// Note. Repo must have server attached at `server_id`.
///
/// 1. Pulls the repo on the target server using `git pull`.
/// 2. If `on_pull` is specified, it will be executed after the pull is complete.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
pub struct PullRepo {
  /// Id or name
  pub repo: String,
}

//

/// Builds the target repo, using the attached builder. Response: [Update].
///
/// Note. Repo must have builder attached at `builder_id`.
///
/// 1. Spawns the target builder instance (For AWS type. For Server type, just use CloneRepo).
/// 2. Clones the repo on the builder using `git clone https://{$token?}@github.com/${repo} -b ${branch}`.
/// The token will only be used if a github account is specified,
/// and must be declared in the periphery configuration on the builder instance.
/// 3. If `on_clone` and `on_pull` are specified, they will be executed.
/// `on_clone` will be executed before `on_pull`.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
pub struct BuildRepo {
  /// Id or name
  pub repo: String,
}

//

/// Cancels the target repo build.
/// Only does anything if the repo build is `building` when called.
/// Response: [Update]
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
pub struct CancelRepoBuild {
  /// Can be id or name
  pub repo: String,
}
