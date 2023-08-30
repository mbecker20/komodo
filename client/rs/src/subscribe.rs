use std::time::Duration;

use anyhow::Context;
use futures::{SinkExt, TryStreamExt};
use monitor_types::entities::update::UpdateListItem;
use serror::serialize_error;
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
        retry_cooldown_secs: u64,
    ) -> (broadcast::Receiver<UpdateWsMessage>, CancellationToken) {
        let (tx, rx) = broadcast::channel(capacity);
        let cancel = CancellationToken::new();
        let cancel_clone = cancel.clone();
        let address = format!("{}/ws/update", self.address.replacen("http", "ws", 1));
        let mut client = self.clone();

        tokio::spawn(async move {
            loop {
                // OUTER LOOP (LONG RECONNECT)
                if cancel.is_cancelled() {
                    break;
                }
                loop {
                    // INNER LOOP (SHORT RECONNECT)
                    if cancel.is_cancelled() {
                        break;
                    }

                    if client.creds.is_some() {
                        let res = client.refresh_jwt().await;
                        if let Err(e) = res {
                            let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                                serialize_error(e),
                            )));
                            break;
                        }
                    }

                    let res = connect_async(&address)
                        .await
                        .context("failed to connect to websocket endpoint");

                    if let Err(e) = res {
                        let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::ConnectionError(
                            serialize_error(e),
                        )));
                        break;
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
                            serialize_error(e),
                        )));
                        break;
                    }

                    // ==================
                    // HANDLE LOGIN RES
                    // ==================
                    match ws.try_next().await {
                        Ok(Some(Message::Text(msg))) => {
                            if msg != "LOGGED_IN" {
                                let _ =
                                    tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(msg)));
                                let _ = ws.close(None).await;
                                break;
                            }
                        }
                        Ok(Some(msg)) => {
                            let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                                format!("{msg:#?}"),
                            )));
                            let _ = ws.close(None).await;
                            break;
                        }
                        Ok(None) => {
                            let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                                String::from("got None message after login message"),
                            )));
                            let _ = ws.close(None).await;
                            break;
                        }
                        Err(e) => {
                            let _ = tx.send(UpdateWsMessage::Error(UpdateWsError::LoginError(
                                format!("failed to recieve message | {e:#?}"),
                            )));
                            let _ = ws.close(None).await;
                            break;
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
                                let _ = ws.close(None).await;
                                break;
                            }
                            Err(e) => {
                                let _ = tx.send(UpdateWsMessage::Error(
                                    UpdateWsError::MessageError(serialize_error(e)),
                                ));
                                let _ = tx.send(UpdateWsMessage::Disconnected);
                                let _ = ws.close(None).await;
                                break;
                            }
                            Ok(_) => {
                                // ignore
                            }
                        }
                    }
                }
                tokio::time::sleep(Duration::from_secs(retry_cooldown_secs)).await;
            }
        });

        (rx, cancel_clone)
    }
}
