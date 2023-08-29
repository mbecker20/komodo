use anyhow::Context;
use futures::{SinkExt, TryStreamExt};
use monitor_types::entities::update::UpdateListItem;
use serror::serialize_error_pretty;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

use crate::MonitorClient;

#[derive(Debug, Clone)]
pub enum UpdateWsMessage {
    Update(UpdateListItem),
    Error(UpdateWsError),
    Disconnected,
    Reconnected,
}

#[derive(Error, Debug, Clone)]
pub enum UpdateWsError {
    #[error("failed to connect | {0}")]
    ConnectionError(String),
    #[error("failed to login | {0}")]
    LoginError(String),
    #[error("failed to recieve message | {0}")]
    MessageError(String),
    #[error("did not recognize message | {0}")]
    MessageUnrecognized(String),
}

impl MonitorClient {
    pub fn subscribe_to_updates(
        &self,
        capacity: usize,
    ) -> (broadcast::Receiver<UpdateWsMessage>, CancellationToken) {
        let (tx, rx) = broadcast::channel(capacity);
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        let address = format!("{}/ws/update", self.address.replacen("http", "ws", 1));
        let mut client = self.clone();

        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    break;
                }

                if client.creds.is_some() {
                    let res = client.refresh_jwt().await;
                    if let Err(e) = res {
                        let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                            serialize_error_pretty(e),
                        )));
                    }
                }

                let res = connect_async(&address)
                    .await
                    .context("failed to connect to websocket endpoint");
                if let Err(e) = res {
                    let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::ConnectionError(
                        serialize_error_pretty(e),
                    )));
                    return;
                }
                let (mut ws, _) = res.unwrap();

                // ==================
                // SEND LOGIN MSG
                // ==================
                let login_send_res = ws
                    .send(Message::Text(client.jwt.clone()))
                    .await
                    .context("failed to send login message");

                if let Err(e) = login_send_res {
                    let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                        serialize_error_pretty(e),
                    )));
                    return;
                }

                // ==================
                // HANDLE LOGIN RES
                // ==================
                match ws.try_next().await {
                    Ok(Some(Message::Text(msg))) => {
                        if msg != "LOGGED_IN" {
                            let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(msg)));
                            return;
                        }
                    }
                    Ok(Some(msg)) => {
                        let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                            format!("{msg:#?}"),
                        )));
                        return;
                    }
                    Ok(None) => {
                        let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                            String::from("got None message after login message"),
                        )));
                        return;
                    }
                    Err(e) => {
                        let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                            format!("failed to recieve message | {e:#?}"),
                        )));
                        return;
                    }
                }

                let _ = tx.send(UpdateWsMessage::Reconnected);

                // ==================
                // HANLDE MSGS
                // ==================
                loop {
                    match ws.try_next().await.context("failed to recieve message") {
                        Ok(Some(Message::Text(msg))) => {
                            match serde_json::from_str::<UpdateListItem>(&msg) {
                                Ok(msg) => {
                                    let _ = tx.send(UpdateWsMessage::Update(msg));
                                }
                                Err(_) => {
                                    let _ = tx.send(UpdateWsMessage::Error(
                                        UpdateWsError::MessageUnrecognized(msg),
                                    ));
                                }
                            }
                        }
                        Ok(Some(Message::Close(_))) => {
                            let _ = tx.send(UpdateWsMessage::Disconnected);
                            break;
                        }
                        Err(e) => {
                            let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::MessageError(
                                serialize_error_pretty(e),
                            )));
                            let _ = tx.send(UpdateWsMessage::Disconnected);
                            break;
                        }
                        Ok(_) => {
                            // ignore
                        }
                    }
                }
            }
        });

        (rx, cancel_clone)
    }
}
