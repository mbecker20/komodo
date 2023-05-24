use std::sync::Arc;

use axum::{
    extract::{ws::Message as AxumMessage, Path, Query, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use helpers::handle_anyhow_error;
use serde::Deserialize;
use tokio::select;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use tokio_util::sync::CancellationToken;
use types::{traits::Permissioned, PermissionLevel, SystemStatsQuery};

use crate::{auth::JwtExtension, state::StateExtension, ResponseResult};

#[derive(Deserialize)]
pub struct ServerId {
    id: String,
}

pub async fn ws_handler(
    state: StateExtension,
    jwt_client: JwtExtension,
    path: Path<ServerId>,
    query: Query<SystemStatsQuery>,
    ws: WebSocketUpgrade,
) -> ResponseResult<impl IntoResponse> {
    let server = state
        .db
        .get_server(&path.id)
        .await
        .map_err(handle_anyhow_error)?;
    let query = Arc::new(query);
    let upgrade = ws.on_upgrade(|socket| async move {
        let login_res = state.ws_login(socket, &jwt_client).await;
        if login_res.is_none() {
            return;
        }
        let (mut socket, user) = login_res.unwrap();
        if !user.is_admin && server.get_user_permissions(&user.id) < PermissionLevel::Read {
            let _ = socket
                .send(AxumMessage::Text(
                    "permission denied. user must have at least read permissions on this server"
                        .to_string(),
                ))
                .await;
            return;
        }
        let (mut ws_sender, mut ws_reciever) = socket.split();
        let res = state.periphery.subscribe_to_stats_ws(&server, &query).await;
        if let Err(e) = &res {
            let _ = ws_sender
                .send(AxumMessage::Text(format!("ERROR: {e}")))
                .await;
            return;
        }
        let mut stats_recv = res.unwrap();
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        tokio::spawn(async move {
            loop {
                let stats = select! {
                    _ = cancel_clone.cancelled() => {
                        let _ = stats_recv.close(None).await;
                        break
                    },
                    stats = stats_recv.next() => stats,
                };
                if let Some(Ok(TungsteniteMessage::Text(msg))) = stats {
                    let _ = ws_sender.send(AxumMessage::Text(msg)).await;
                } else {
                    let _ = stats_recv.close(None).await;
                    break;
                }
            }
        });
        while let Some(msg) = ws_reciever.next().await {
            match msg {
                Ok(msg) => match msg {
                    AxumMessage::Close(_) => {
                        // println!("CLOSE");
                        cancel.cancel();
                        return;
                    }
                    _ => {}
                },
                Err(_) => {
                    // println!("CLOSE FROM ERR");
                    cancel.cancel();
                    return;
                }
            }
        }
    });
    Ok(upgrade)
}
