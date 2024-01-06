use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::async_trait;
use monitor_types::requests::auth::{
  LoginWithSecret, LoginWithSecretResponse,
};
use mungos::mongodb::bson::{doc, oid::ObjectId};
use resolver_api::Resolve;

use crate::state::State;

#[async_trait]
impl Resolve<LoginWithSecret> for State {
  async fn resolve(
    &self,
    LoginWithSecret { username, secret }: LoginWithSecret,
    _: (),
  ) -> anyhow::Result<LoginWithSecretResponse> {
    let user = self
      .db
      .users
      .find_one(doc! { "username": &username }, None)
      .await
      .context("failed at mongo query")?
      .ok_or(anyhow!("did not find user with username {username}"))?;
    let ts = unix_timestamp_ms() as i64;
    for s in user.secrets {
      if let Some(expires) = s.expires {
        if expires < ts {
          self
            .db
            .users
            .update_one(
              doc! { "_id": ObjectId::from_str(&user.id).context("user id is not valid ObjectId")? },
              doc! { "$pull": { "secrets": { "name": s.name } } },
              None,
            )
            .await
            .context("failed to remove expired secret")?;
          continue;
        }
      }
      if bcrypt::verify(&secret, &s.hash)
        .context("failed at verifying hash")?
      {
        let jwt = self
          .jwt
          .generate(user.id)
          .context("failed at generating jwt for user")?;
        return Ok(LoginWithSecretResponse { jwt });
      }
    }
    Err(anyhow!("invalid secret"))
  }
}

// pub fn router() -> Router {
//     Router::new().route(
//         "/login",
//         post(|db, jwt, body| async { login(db, jwt, body).await.map_err(|e| ()) }),
//     )
// }

// pub async fn login(
//     state: StateExtension,
//     Json(SecretLoginBody { username, secret }): Json<SecretLoginBody>,
// ) -> anyhow::Result<String> {
//     let user = state
//         .db
//         .users
//         .find_one(doc! { "username": &username }, None)
//         .await
//         .context("failed at mongo query")?
//         .ok_or(anyhow!("did not find user with username {username}"))?;
//     let ts = unix_timestamp_ms() as i64;
//     for s in user.secrets {
//         if let Some(expires) = s.expires {
//             let expires = unix_from_monitor_ts(&expires)?;
//             if expires < ts {
//                 state
//                     .db
//                     .users
//                     .update_one::<Document>(
//                         &user.id,
//                         Update::Custom(doc! { "$pull": { "secrets": { "name": s.name } } }),
//                     )
//                     .await
//                     .context("failed to remove expired secret")?;
//                 continue;
//             }
//         }
//         if bcrypt::verify(&secret, &s.hash).context("failed at verifying hash")? {
//             let jwt = jwt
//                 .generate(user.id)
//                 .context("failed at generating jwt for user")?;
//             return Ok(jwt);
//         }
//     }
//     Err(anyhow!("invalid secret"))
// }
