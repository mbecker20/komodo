use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::user::ApiSecret,
    monitor_timestamp,
    requests::write::*,
};
use mungos::{
    mongodb::bson::{doc, to_bson},
    Update,
};
use resolver_api::Resolve;

use crate::{
    auth::{random_string, RequestUser},
    state::State,
};

const SECRET_LENGTH: usize = 40;
const BCRYPT_COST: u32 = 10;

#[async_trait]
impl Resolve<CreateLoginSecret, RequestUser> for State {
    async fn resolve(
        &self,
        secret: CreateLoginSecret,
        user: RequestUser,
    ) -> anyhow::Result<CreateLoginSecretResponse> {
        let user = self.get_user(&user.id).await?;
        for s in &user.secrets {
            if s.name == secret.name {
                return Err(anyhow!("secret with name {} already exists", secret.name));
            }
        }
        let secret_str = random_string(SECRET_LENGTH);
        let api_secret = ApiSecret {
            name: secret.name,
            created_at: monitor_timestamp(),
            expires: secret.expires,
            hash: bcrypt::hash(&secret_str, BCRYPT_COST)
                .context("failed at hashing secret string")?,
        };
        self.db
            .users
            .update_one(
                &user.id,
                Update::Custom(doc! {
                    "$push": {
                        "secrets": to_bson(&api_secret).context("failed at converting secret to bson")?
                    }
                }),
            )
            .await
            .context("failed at mongo update query")?;
        Ok(CreateLoginSecretResponse { secret: secret_str })
    }
}

#[async_trait]
impl Resolve<DeleteLoginSecret, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteLoginSecret { name }: DeleteLoginSecret,
        user: RequestUser,
    ) -> anyhow::Result<()> {
        self.db
            .users
            .update_one(
                &user.id,
                Update::Custom(doc! {
                    "$pull": {
                        "secrets": {
                            "name": name
                        }
                    }
                }),
            )
            .await
            .context("failed at mongo update query")?;
        Ok(())
    }
}
