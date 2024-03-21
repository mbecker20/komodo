use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::I64;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
pub struct ApiKey {
  /// UNIQUE KEY ASSOCIATED WITH SECRET
  #[unique_index]
  pub key: String,

  /// HASH OF THE SECRET
  pub secret: String,

  /// USER ASSOCIATED WITH THE API KEY
  #[index]
  pub user_id: String,

  /// NAME ASSOCIATED WITH THE API KEY FOR MANAGEMENT
  pub name: String,

  /// TIMESTAMP OF KEY CREATION
  pub created_at: I64,

  /// EXPIRY OF KEY, OR 0 IF NEVER EXPIRES
  pub expires: I64,
}

impl ApiKey {
  pub fn sanitize(&mut self) {
    self.secret.clear()
  }
}
