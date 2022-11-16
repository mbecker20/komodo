use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use db::{DbClient, DbExtension};
use serde_json::Value;

use crate::auth::{JwtClient, JwtExtension};
use tokio::sync::watch::{self, error::SendError, Receiver, Sender};

pub type WsSender = Arc<Sender<String>>;
pub type WsSenderExtension = Extension<WsSender>;

pub type WsReciever = Receiver<String>;
pub type WsRecieverExtension = Extension<WsReciever>;

pub fn make_ws_sender_reciver() -> (WsSenderExtension, WsRecieverExtension) {
    let (sender, reciever) = watch::channel(String::new());
    (Extension(Arc::new(sender)), Extension(reciever))
}

pub async fn ws_handler(
    Extension(jwt_client): JwtExtension,
    Extension(db_client): DbExtension,
    Extension(mut reciever): WsRecieverExtension,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        match login(socket, &jwt_client, &db_client).await {
            Some((ws, user_id)) => {
				loop {
					reciever.changed().await;
					let msg = serde_json::from_str::<Value>(&reciever.borrow()).unwrap();

					todo!()
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
                    Ok(user) => Some((socket, user.id)),
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
                _ => None,
            },
            Err(e) => {
                let _ = socket
                    .send(Message::Text(format!("failed to get message: {e:#?}")))
                    .await;
                let _ = socket.close().await;
                None
            }
        }
    } else {
        None
    }
}
