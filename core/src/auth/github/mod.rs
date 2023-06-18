use axum::Router;

use crate::config::CoreConfig;

pub mod client;

// pub fn router(config: &CoreConfig) -> Router {
//     let client = GithubOauthClient::new(
//         config.github_oauth.id.clone(),
//         config.github_oauth.secret.clone(),
//         format!("{}/auth/github/callback", config.host),
//         &[],
//         "monitor".to_string(),
//     );
//     Router::new()
//         .route(
//             "/login",
//             get(|Extension(client): GithubOauthExtension| async move {
//                 Redirect::to(&client.get_login_redirect_url())
//             }),
//         )
//         .route(
//             "/callback",
//             get(|client, jwt, state, query| async {
//                 let redirect = callback(client, jwt, state, query)
//                     .await
//                     .map_err(handle_anyhow_error)?;
//                 response!(redirect)
//             }),
//         )
//         .layer(Extension(Arc::new(client)))
// }

// #[derive(Deserialize)]
// struct CallbackQuery {
//     state: String,
//     code: String,
// }

// async fn callback(
//     Extension(client): GithubOauthExtension,
//     Extension(jwt_client): JwtExtension,
//     Extension(state): StateExtension,
//     Query(query): Query<CallbackQuery>,
// ) -> anyhow::Result<Redirect> {
//     if !client.check_state(&query.state) {
//         return Err(anyhow!("state mismatch"));
//     }
//     let token = client.get_access_token(&query.code).await?;
//     let github_user = client.get_github_user(&token.access_token).await?;
//     let github_id = github_user.id.to_string();
//     let user = state
//         .db
//         .users
//         .find_one(doc! { "github_id": &github_id }, None)
//         .await
//         .context("failed at find user query from mongo")?;
//     let jwt = match user {
//         Some(user) => jwt_client
//             .generate(user.id)
//             .context("failed to generate jwt")?,
//         None => {
//             let ts = monitor_timestamp();
//             let no_users_exist = state.db.users.find_one(None, None).await?.is_none();
//             let user = User {
//                 username: github_user.login,
//                 avatar: github_user.avatar_url.into(),
//                 github_id: github_id.into(),
//                 enabled: no_users_exist,
//                 admin: no_users_exist,
//                 create_server_permissions: no_users_exist,
//                 create_build_permissions: no_users_exist,
//                 created_at: ts.clone(),
//                 updated_at: ts,
//                 ..Default::default()
//             };
//             let user_id = state
//                 .db
//                 .users
//                 .create_one(user)
//                 .await
//                 .context("failed to create user on mongo")?;
//             jwt_client
//                 .generate(user_id)
//                 .context("failed to generate jwt")?
//         }
//     };
//     let exchange_token = jwt_client.create_exchange_token(jwt);
//     Ok(Redirect::to(&format!(
//         "{}?token={exchange_token}",
//         state.config.host
//     )))
// }