use anyhow::anyhow;
use axum::{
    extract::{ws::Message, WebSocketUpgrade},
    response::IntoResponse,
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

use crate::{auth::JwtExtension, state::StateExtension};

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
    jwt_client: JwtExtension,
    state: StateExtension,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let mut reciever = state.update.reciever.resubscribe();
    ws.on_upgrade(|socket| async move {
        let login_res = state.ws_login(socket, &jwt_client).await;
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
                    update = reciever.recv() => {update.expect("failed to recv update msg")}
                };
                let user = state.db.users.find_one_by_id(&user.id).await;
                if user.is_err()
                    || user.as_ref().unwrap().is_none()
                    || !user.as_ref().unwrap().as_ref().unwrap().enabled
                {
                    let _ = ws_sender
                        .send(Message::Text(json!({ "type": "INVALID_USER" }).to_string()))
                        .await;
                    let _ = ws_sender.close().await;
                    return;
                }
                let user = user.unwrap().unwrap(); // already handle cases where this panics in the above early return
                match user_can_see_update(&user, &user.id, &update.target, &state.db).await {
                    Ok(_) => {
                        let _ = ws_sender
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
        UpdateTarget::Group(group_id) => {
            let permissions = db_client
                .get_user_permission_on_group(user_id, group_id)
                .await?;
            (permissions, "group")
        }
        UpdateTarget::Command(command_id) => {
            let permissions = db_client
                .get_user_permission_on_command(user_id, command_id)
                .await?;
            (permissions, "command")
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
