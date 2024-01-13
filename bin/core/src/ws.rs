use anyhow::anyhow;
use axum::{
  extract::{
    ws::{Message, WebSocket},
    WebSocketUpgrade,
  },
  response::IntoResponse,
  routing::get,
  Router,
};
use futures::{SinkExt, StreamExt};
use monitor_client::{
  entities::{
    alerter::Alerter,
    build::Build,
    builder::Builder,
    deployment::Deployment,
    procedure::Procedure,
    repo::Repo,
    server::Server,
    update::{ResourceTarget, ResourceTargetVariant},
    user::User,
    PermissionLevel,
  },
  permissioned::Permissioned,
  ws::WsLoginMessage,
};
use mungos::by_id::find_one_by_id;
use serde_json::json;
use tokio::select;
use tokio_util::sync::CancellationToken;

use crate::{
  auth::RequestUser,
  helpers::resource::StateResource,
  state::{State, StateExtension},
};

pub fn router() -> Router {
  Router::new().route("/update", get(ws_handler))
}

async fn ws_handler(
  state: StateExtension,
  ws: WebSocketUpgrade,
) -> impl IntoResponse {
  let mut receiver = state.update.receiver.resubscribe();
  ws.on_upgrade(|socket| async move {
    let login_res = state.ws_login(socket).await;
    if login_res.is_none() {
      return;
    }

    let (socket, user) = login_res.unwrap();
    let (mut ws_sender, mut ws_reciever) = socket.split();
    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    tokio::spawn(async move {
      loop {
        let update = select! {
          _ = cancel_clone.cancelled() => break,
          update = receiver.recv() => {update.expect("failed to recv update msg")}
        };
        let user = find_one_by_id(&state.db.users, &user.id).await;
        let user = match user {
          Err(e) => {
            let _ = ws_sender
              .send(Message::Text(json!({ "type": "INVALID_USER", "msg": format!("{e:#?}") }).to_string()))
              .await;
            let _ = ws_sender.close().await;
            return;
          },
          Ok(None) => {
            let _ = ws_sender
              .send(Message::Text(json!({ "type": "INVALID_USER", "msg": "user not found" }).to_string()))
              .await;
            let _ = ws_sender.close().await;
            return
          },
          Ok(Some(user)) if !user.enabled => {
            let _ = ws_sender
              .send(Message::Text(json!({ "type": "INVALID_USER", "msg": "user not enabled" }).to_string()))
              .await;
            let _ = ws_sender.close().await;
            return
          }
          Ok(Some(user)) => user

        };
        let res = state
          .user_can_see_update(&user, &update.target)
          .await;
        if res.is_ok() {
          let _ = ws_sender
            .send(Message::Text(serde_json::to_string(&update).unwrap()))
            .await;
        }
      }
    });

    while let Some(msg) = ws_reciever.next().await {
      match msg {
        Ok(msg) => {
          if let Message::Close(_) = msg {
            cancel.cancel();
            return;
          }
        }
        Err(_) => {
          cancel.cancel();
          return;
        }
      }
    }
    })
}

impl State {
  pub async fn ws_login(
    &self,
    mut socket: WebSocket,
  ) -> Option<(WebSocket, RequestUser)> {
    match socket.recv().await {
      Some(Ok(Message::Text(login_msg))) => {
        // login
        match WsLoginMessage::from_json_str(&login_msg) {
          Ok(WsLoginMessage::Jwt { jwt }) => {
            match self.auth_jwt_check_enabled(&jwt).await {
              Ok(user) => {
                let _ = socket
                  .send(Message::Text("LOGGED_IN".to_string()))
                  .await;
                Some((socket, user))
              }
              Err(e) => {
                let _ = socket
                  .send(Message::Text(format!(
                    "failed to authenticate user using jwt | {e:#?}"
                  )))
                  .await;
                let _ = socket.close().await;
                None
              }
            }
          }
          Ok(WsLoginMessage::ApiKeys { key, secret }) => {
            match self.auth_api_key_check_enabled(&key, &secret).await
            {
              Ok(user) => {
                let _ = socket
                  .send(Message::Text("LOGGED_IN".to_string()))
                  .await;
                Some((socket, user))
              }
              Err(e) => {
                let _ = socket
                  .send(Message::Text(format!(
                    "failed to authenticate user using api keys | {e:#?}"
                  )))
                  .await;
                let _ = socket.close().await;
                None
              }
            }
          }
          Err(e) => {
            let _ = socket
              .send(Message::Text(format!(
                "failed to parse login message: {e:#?}"
              )))
              .await;
            let _ = socket.close().await;
            None
          }
        }
      }
      Some(Ok(msg)) => {
        let _ = socket
          .send(Message::Text(format!(
            "invalid login message: {msg:#?}"
          )))
          .await;
        let _ = socket.close().await;
        None
      }
      Some(Err(e)) => {
        let _ = socket
          .send(Message::Text(format!(
            "failed to get login message: {e:#?}"
          )))
          .await;
        let _ = socket.close().await;
        None
      }
      None => {
        let _ = socket
          .send(Message::Text(String::from(
            "failed to get login message",
          )))
          .await;
        let _ = socket.close().await;
        None
      }
    }
  }

  async fn user_can_see_update(
    &self,
    user: &User,
    update_target: &ResourceTarget,
  ) -> anyhow::Result<()> {
    if user.admin {
      return Ok(());
    }
    let (permissions, target) = match update_target {
      ResourceTarget::Server(server_id) => {
        let resource: Server = self.get_resource(server_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Server,
        )
      }
      ResourceTarget::Deployment(deployment_id) => {
        let resource: Deployment =
          self.get_resource(deployment_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Deployment,
        )
      }
      ResourceTarget::Build(build_id) => {
        let resource: Build = self.get_resource(build_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Build,
        )
      }
      ResourceTarget::Builder(builder_id) => {
        let resource: Builder = self.get_resource(builder_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Builder,
        )
      }
      ResourceTarget::Repo(repo_id) => {
        let resource: Repo = self.get_resource(repo_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Repo,
        )
      }
      ResourceTarget::Alerter(alerter_id) => {
        let resource: Alerter = self.get_resource(alerter_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Alerter,
        )
      }
      ResourceTarget::Procedure(prodecure_id) => {
        let resource: Procedure =
          self.get_resource(prodecure_id).await?;
        (
          resource.get_user_permissions(&user.id),
          ResourceTargetVariant::Procedure,
        )
      }
      ResourceTarget::System(_) => {
        return Err(anyhow!(
          "user not admin, can't recieve system updates"
        ))
      }
    };
    if permissions != PermissionLevel::None {
      Ok(())
    } else {
      Err(anyhow!("user does not have permissions on {target}"))
    }
  }
}
