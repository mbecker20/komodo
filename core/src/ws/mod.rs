use axum::{
    extract::ws::{Message, WebSocket},
    routing::get,
    Router,
};

use crate::{
    auth::{JwtClient, RequestUser},
    state::State,
};

mod stats;
pub mod update;

pub fn router() -> Router {
    Router::new()
        .route("/update", get(update::ws_handler))
        .route("/stats/:id", get(stats::ws_handler))
}

impl State {
    pub async fn ws_login(
        &self,
        mut socket: WebSocket,
        jwt_client: &JwtClient,
    ) -> Option<(WebSocket, RequestUser)> {
        if let Some(jwt) = socket.recv().await {
            match jwt {
                Ok(jwt) => match jwt {
                    Message::Text(jwt) => {
                        match jwt_client.auth_jwt_check_enabled(&jwt, self).await {
                            Ok(user) => {
                                let _ = socket.send(Message::Text("LOGGED_IN".to_string())).await;
                                Some((socket, user))
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
                        }
                    }
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
            let _ = socket
                .send(Message::Text(format!("failed to get jwt message")))
                .await;
            let _ = socket.close().await;
            None
        }
    }
}
