use std::time::Duration;

use anyhow::Context;
use futures::{SinkExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use serror::serialize_error;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;
use tracing::{info, info_span, warn};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{entities::update::UpdateListItem, MonitorClient};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum WsLoginMessage {
  Jwt { jwt: String },
  ApiKeys { key: String, secret: String },
}

impl WsLoginMessage {
  pub fn from_json_str(json: &str) -> anyhow::Result<WsLoginMessage> {
    serde_json::from_str(json)
      .context("failed to parse json as WsLoginMessage")
  }

  pub fn to_json_string(&self) -> anyhow::Result<String> {
    serde_json::to_string(self)
      .context("failed to serialize WsLoginMessage to json string")
  }
}

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
  ) -> anyhow::Result<(
    broadcast::Receiver<UpdateWsMessage>,
    CancellationToken,
  )> {
    let (tx, rx) = broadcast::channel(capacity);
    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();
    let address =
      format!("{}/ws/update", self.address.replacen("http", "ws", 1));
    let login_msg = WsLoginMessage::ApiKeys {
      key: self.key.clone(),
      secret: self.secret.clone(),
    }
    .to_json_string()?;

    tokio::spawn(async move {
      let master_uuid = Uuid::new_v4();
      loop {
        // OUTER LOOP (LONG RECONNECT)
        if cancel.is_cancelled() {
          break;
        }

        let outer_uuid = Uuid::new_v4();
        let span = info_span!("Outer Loop");
        let _ = span.enter();

        info!("Entering inner (connection) loop | outer uuid {outer_uuid} | master uuid {master_uuid}");

        loop {
          // INNER LOOP (SHORT RECONNECT)
          if cancel.is_cancelled() {
            break;
          }

          let inner_uuid = Uuid::new_v4();
          let span = info_span!("Inner Loop");
          let _ = span.enter();

          info!("Connecting to websocket | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");

          let mut ws =
            match connect_async(&address).await.with_context(|| {
              format!(
                "failed to connect to monitor websocket at {address}"
              )
            }) {
              Ok((ws, _)) => ws,
              Err(e) => {
                let _ = tx.send(UpdateWsMessage::Error(
                  UpdateWsError::ConnectionError(serialize_error(&e)),
                ));
                warn!("{e:#}");
                break;
              }
            };

          info!("Connected to websocket | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");

          // ==================
          // SEND LOGIN MSG
          // ==================
          let login_send_res = ws
            .send(Message::Text(login_msg.clone()))
            .await
            .context("failed to send login message");

          if let Err(e) = login_send_res {
            let _ = tx.send(UpdateWsMessage::Error(
              UpdateWsError::LoginError(serialize_error(&e)),
            ));
            warn!("breaking inner loop | {e:#} | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");
            break;
          }

          // ==================
          // HANDLE LOGIN RES
          // ==================
          match ws.try_next().await {
            Ok(Some(Message::Text(msg))) => {
              if msg != "LOGGED_IN" {
                let _ = tx.send(UpdateWsMessage::Error(
                  UpdateWsError::LoginError(msg.clone()),
                ));
                let _ = ws.close(None).await;
                warn!("breaking inner loop | got msg {msg} instead of 'LOGGED_IN' | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");
                break;
              }
            }
            Ok(Some(msg)) => {
              let _ = tx.send(UpdateWsMessage::Error(
                UpdateWsError::LoginError(format!("{msg:#?}")),
              ));
              let _ = ws.close(None).await;
              warn!("breaking inner loop | got msg {msg} instead of Message::Text 'LOGGED_IN' | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");
              break;
            }
            Ok(None) => {
              let _ = tx.send(UpdateWsMessage::Error(
                UpdateWsError::LoginError(String::from(
                  "got None message after login message",
                )),
              ));
              let _ = ws.close(None).await;
              warn!("breaking inner loop | got None instead of 'LOGGED_IN' | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");
              break;
            }
            Err(e) => {
              let _ = tx.send(UpdateWsMessage::Error(
                UpdateWsError::LoginError(format!(
                  "failed to recieve message | {e:#?}"
                )),
              ));
              let _ = ws.close(None).await;
              warn!("breaking inner loop | got error msg | {e:?} | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");
              break;
            }
          }

          let _ = tx.send(UpdateWsMessage::Reconnected);

          info!("logged into websocket | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}");

          // ==================
          // HANLDE MSGS
          // ==================
          loop {
            match ws
              .try_next()
              .await
              .context("failed to recieve message")
            {
              Ok(Some(Message::Text(msg))) => {
                match serde_json::from_str::<UpdateListItem>(&msg) {
                  Ok(msg) => {
                    tracing::debug!(
                      "got recognized message: {msg:?} | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}"
                    );
                    let _ = tx.send(UpdateWsMessage::Update(msg));
                  }
                  Err(_) => {
                    tracing::warn!(
                      "got unrecognized message: {msg:?} | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}"
                    );
                    let _ = tx.send(UpdateWsMessage::Error(
                      UpdateWsError::MessageUnrecognized(msg),
                    ));
                  }
                }
              }
              Ok(Some(Message::Close(_))) => {
                let _ = tx.send(UpdateWsMessage::Disconnected);
                let _ = ws.close(None).await;
                tracing::warn!(
                  "breaking inner loop | got close message | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}"
                );
                break;
              }
              Err(e) => {
                let _ = tx.send(UpdateWsMessage::Error(
                  UpdateWsError::MessageError(serialize_error(&e)),
                ));
                let _ = tx.send(UpdateWsMessage::Disconnected);
                let _ = ws.close(None).await;
                tracing::warn!(
                  "breaking inner loop | got error message | {e:#} | inner uuid {inner_uuid} | outer uuid {outer_uuid} | master uuid {master_uuid}"
                );
                break;
              }
              Ok(_) => {
                // ignore
              }
            }
          }
        }
        tokio::time::sleep(Duration::from_secs(retry_cooldown_secs))
          .await;
      }
    });

    Ok((rx, cancel_clone))
  }
}
