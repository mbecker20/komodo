use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use db::{DbClient, DbExtension};
use futures_util::{SinkExt, StreamExt};
use mungos::Serialize;
use serde_json::json;
use tokio::{
    select,
    sync::{
        watch::{self, Receiver, Sender},
        Mutex,
    },
};
use tokio_util::sync::CancellationToken;
use types::{EntityType, Update, User};

use crate::auth::{JwtClient, JwtExtension};

use super::user_permissions;

pub type WsSender = Arc<Mutex<Sender<Update>>>;
pub type WsSenderExtension = Extension<WsSender>;

pub type WsReciever = Receiver<Update>;
pub type WsRecieverExtension = Extension<WsReciever>;

#[derive(Serialize)]
struct UpdateMsg {
    #[serde(rename = "type")]
    msg_type: MsgType,
    update: Update,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum MsgType {
    Login,
    Close,
    Update,
}

impl UpdateMsg {
    fn from_update(update: Update) -> Message {
        let msg = UpdateMsg {
            msg_type: MsgType::Update,
            update,
        };
        Message::Text(serde_json::to_string(&msg).unwrap())
    }
}

pub fn make_update_ws_sender_reciver() -> (WsSenderExtension, WsRecieverExtension) {
    let (sender, reciever) = watch::channel(Default::default());
    (
        Extension(Arc::new(Mutex::new((sender)))),
        Extension(reciever),
    )
}

pub async fn ws_handler(
    Extension(jwt_client): JwtExtension,
    Extension(db_client): DbExtension,
    Extension(mut reciever): WsRecieverExtension,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        match login(socket, &jwt_client, &db_client).await {
            Some((mut socket, user_id)) => {
                let (ws_sender, mut ws_reciever) = socket.split();
                let ws_sender = Arc::new(Mutex::new(ws_sender));
                let ws_sender_clone = ws_sender.clone();
                let cancel = CancellationToken::new();
                let cancel_clone = cancel.clone();
                tokio::spawn(async move {
                    loop {
                        select! {
                            _ = cancel_clone.cancelled() => break,
                            _ = reciever.changed() => {}
                        };
                        let user = db_client.users.find_one_by_id(&user_id).await;
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
                        let update = reciever.borrow().to_owned();
                        match user_can_see_update(
                            &user,
                            &user_id,
                            update.entity_type,
                            &update.entity_id,
                            &db_client,
                        )
                        .await
                        {
                            Ok(_) => {
                                let _ = ws_sender
                                    .lock()
                                    .await
                                    .send(UpdateMsg::from_update(update))
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
            }
            None => {}
        }
    })
}

async fn login(
    mut socket: WebSocket,
    jwt_client: &JwtClient,
    db_client: &DbClient,
) -> Option<(WebSocket, String)> {
    if let Some(jwt) = socket.recv().await {
        match jwt {
            Ok(jwt) => match jwt {
                Message::Text(jwt) => match jwt_client.auth_jwt(&jwt, db_client).await {
                    Ok(user) => {
                        socket
                            .send(Message::Text(
                                json!({ "type": "LOGIN", "success": true }).to_string(),
                            ))
                            .await;
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
    entity_type: EntityType,
    entity_id: &Option<String>,
    db_client: &DbClient,
) -> anyhow::Result<()> {
    if user.admin {
        return Ok(());
    }
    match entity_type {
        EntityType::System => {
            if user.admin {
                Ok(())
            } else {
                Err(anyhow!("user not admin, can't recieve system updates"))
            }
        }
        EntityType::Server => {
            let server_id = entity_id
                .as_ref()
                .ok_or(anyhow!("must pass entity_id for {entity_type}"))?;
            let server = db_client
                .servers
                .find_one_by_id(server_id)
                .await
                .context(format!("failed at query to get server at {server_id}"))?
                .ok_or(anyhow!("did not server with id {server_id}"))?;
            user_permissions(user_id, &server.permissions)
        }
        EntityType::Deployment => {
            let deployment_id = entity_id
                .as_ref()
                .ok_or(anyhow!("must pass entity_id for {entity_type}"))?;
            let deployment = db_client
                .deployments
                .find_one_by_id(deployment_id)
                .await
                .context(format!(
                    "failed at query to get deployment at {deployment_id}"
                ))?
                .ok_or(anyhow!("did not deployment with id {deployment_id}"))?;
            user_permissions(user_id, &deployment.permissions)
        }
        EntityType::Build => {
            let build_id = entity_id
                .as_ref()
                .ok_or(anyhow!("must pass entity_id for {entity_type}"))?;
            let build = db_client
                .builds
                .find_one_by_id(build_id)
                .await
                .context(format!("failed at query to get build at {build_id}"))?
                .ok_or(anyhow!("did not build with id {build_id}"))?;
            user_permissions(user_id, &build.permissions)
        }
    }
}
