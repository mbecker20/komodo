use anyhow::{anyhow, Context};
use monitor_client::{
  api::read::{
    GetUsername, GetUsernameResponse, ListApiKeys,
    ListApiKeysForServiceUser, ListApiKeysForServiceUserResponse,
    ListApiKeysResponse, ListUsers, ListUsersResponse,
  },
  entities::user::{User, UserConfig},
};
use mungos::{
  by_id::find_one_by_id, find::find_collect, mongodb::bson::doc,
};
use resolver_api::Resolve;

use crate::state::{db_client, State};

impl Resolve<GetUsername, User> for State {
  async fn resolve(
    &self,
    GetUsername { user_id }: GetUsername,
    _: User,
  ) -> anyhow::Result<GetUsernameResponse> {
    let user = find_one_by_id(&db_client().await.users, &user_id)
      .await
      .context("failed at mongo query for user")?
      .context("no user found with id")?;

    let avatar = match user.config {
      UserConfig::Github { avatar, .. } => Some(avatar),
      UserConfig::Google { avatar, .. } => Some(avatar),
      _ => None,
    };

    Ok(GetUsernameResponse {
      username: user.username,
      avatar,
    })
  }
}

impl Resolve<ListUsers, User> for State {
  async fn resolve(
    &self,
    ListUsers {}: ListUsers,
    user: User,
  ) -> anyhow::Result<ListUsersResponse> {
    if !user.admin {
      return Err(anyhow!("this route is only accessable by admins"));
    }
    let mut users =
      find_collect(&db_client().await.users, None, None)
        .await
        .context("failed to pull users from db")?;
    users.iter_mut().for_each(|user| user.sanitize());
    Ok(users)
  }
}

impl Resolve<ListApiKeys, User> for State {
  async fn resolve(
    &self,
    ListApiKeys {}: ListApiKeys,
    user: User,
  ) -> anyhow::Result<ListApiKeysResponse> {
    let api_keys = find_collect(
      &db_client().await.api_keys,
      doc! { "user_id": &user.id },
      None,
    )
    .await
    .context("failed to query db for api keys")?
    .into_iter()
    .map(|mut api_keys| {
      api_keys.sanitize();
      api_keys
    })
    .collect();
    Ok(api_keys)
  }
}

impl Resolve<ListApiKeysForServiceUser, User> for State {
  async fn resolve(
    &self,
    ListApiKeysForServiceUser { user_id }: ListApiKeysForServiceUser,
    admin: User,
  ) -> anyhow::Result<ListApiKeysForServiceUserResponse> {
    if !admin.admin {
      return Err(anyhow!("This method is admin only."));
    }
    let user = find_one_by_id(&db_client().await.users, &user_id)
      .await
      .context("failed to query db for users")?
      .context("user at id not found")?;
    let UserConfig::Service { .. } = user.config else {
      return Err(anyhow!("Given user is not service user"));
    };
    let api_keys = find_collect(
      &db_client().await.api_keys,
      doc! { "user_id": user_id },
      None,
    )
    .await
    .context("failed to query db for api keys")?
    .into_iter()
    .map(|mut api_keys| {
      api_keys.sanitize();
      api_keys
    })
    .collect();
    Ok(api_keys)
  }
}
