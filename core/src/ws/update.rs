use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use db::DbClient;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::{
    select,
    sync::{
        broadcast::{self, Receiver, Sender},
        Mutex,
    },
};
use tokio_util::sync::CancellationToken;
use types::{PermissionLevel, Update, UpdateTarget, User};

use crate::{
    auth::{JwtClient, JwtExtension},
    state::{State, StateExtension},
};

pub type UpdateWsSender = Mutex<Sender<Update>>;

pub type UpdateWsReciever = Receiver<Update>;
pub struct UpdateWsChannel {
    pub sender: UpdateWsSender,
    pub reciever: UpdateWsReciever,
}

impl UpdateWsChannel {
    pub fn new() -> UpdateWsChannel {
        let (sender, reciever) = broadcast::channel(16);
        UpdateWsChannel {
            sender: Mutex::new(sender),
            reciever,
        }
    }
}

pub async fn ws_handler(
    Extension(jwt_client): JwtExtension,
    Extension(state): StateExtension,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let mut reciever = state.update.reciever.resubscribe();
    ws.on_upgrade(|socket| async move {
        let login_res = login(socket, &jwt_client, &state).await;
        if login_res.is_none() {
            return;
        }
        let (socket, user_id) = login_res.unwrap();
        let (ws_sender, mut ws_reciever) = socket.split();
        let ws_sender = Arc::new(Mutex::new(ws_sender));
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            loop {
                let update = select! {
                    _ = cancel_clone.cancelled() => break,
                    update = reciever.recv() => {update.expect("failed to recv update msg")}
                };
                let user = state.db.users.find_one_by_id(&user_id).await;
                if user.is_err()
                    || user.as_ref().unwrap().is_none()
                    || !user.as_ref().unwrap().as_ref().unwrap().enabled
                {
                    let _ = ws_sender
                        .lock()
                        .await
                        .send(Message::Text(json!({ "type": "INVALID_USER" }).to_string()))
                        .await;
                    let _ = ws_sender.lock().await.close().await;
                    return;
                }
                let user = user.unwrap().unwrap(); // already handle cases where this panics in the above early return
                match user_can_see_update(&user, &user_id, &update.target, &state.db).await {
                    Ok(_) => {
                        let _ = ws_sender
                            .lock()
                            .await
                            .send(Message::Text(serde_json::to_string(&update).unwrap()))
                            .await;
                    }
                    Err(_) => {
                        // make these error visible in some way
                    }
                }
            }
        });

        while let Some(msg) = ws_reciever.next().await {
            match msg {
                Ok(msg) => match msg {
                    Message::Close(_) => {
                        cancel.cancel();
                        return;
                    }
                    _ => {}
                },
                Err(_) => {
                    cancel.cancel();
                    return;
                }
            }
        }
    })
}

async fn login(
    mut socket: WebSocket,
    jwt_client: &JwtClient,
    state: &State,
) -> Option<(WebSocket, String)> {
    if let Some(jwt) = socket.recv().await {
        match jwt {
            Ok(jwt) => match jwt {
                Message::Text(jwt) => match jwt_client.auth_jwt_check_enabled(&jwt, state).await {
                    Ok(user) => {
                        let _ = socket.send(Message::Text("LOGGED_IN".to_string())).await;
                        Some((socket, user.id))
                    }
                    Err(e) => {
                        let _ = socket
                            .send(Message::Text(format!(
                                "failed to authenticate user | {e:#?}"
                            )))
                            .await;
                        let _ = socket.close().await;
                        None
                    }
                },
                msg => {
                    let _ = socket
                        .send(Message::Text(format!("invalid login msg: {msg:#?}")))
                        .await;
                    let _ = socket.close().await;
                    None
                }
            },
            Err(e) => {
                let _ = socket
                    .send(Message::Text(format!("failed to get jwt message: {e:#?}")))
                    .await;
                let _ = socket.close().await;
                None
            }
        }
    } else {
        None
    }
}

async fn user_can_see_update(
    user: &User,
    user_id: &str,
    update_target: &UpdateTarget,
    db_client: &DbClient,
) -> anyhow::Result<()> {
    if user.admin {
        return Ok(());
    }
    let (permissions, target) = match update_target {
        UpdateTarget::Server(server_id) => {
            let permissions = db_client
                .get_user_permission_on_server(user_id, server_id)
                .await?;
            (permissions, "server")
        }
        UpdateTarget::Deployment(deployment_id) => {
            let permissions = db_client
                .get_user_permission_on_deployment(user_id, deployment_id)
                .await?;
            (permissions, "deployment")
        }
        UpdateTarget::Build(build_id) => {
            let permissions = db_client
                .get_user_permission_on_build(user_id, build_id)
                .await?;
            (permissions, "build")
        }
        UpdateTarget::Procedure(procedure_id) => {
            let permissions = db_client
                .get_user_permission_on_procedure(user_id, procedure_id)
                .await?;
            (permissions, "procedure")
        }
        UpdateTarget::System => {
            return Err(anyhow!("user not admin, can't recieve system updates"))
        }
    };
    if permissions != PermissionLevel::None {
        Ok(())
    } else {
        Err(anyhow!("user does not have permissions on {target}"))
    }
}
