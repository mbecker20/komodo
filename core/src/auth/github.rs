use std::sync::Arc;

use axum::{Extension, Router};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use types::CoreConfig;

pub type GithubOauthExtension = Extension<Arc<BasicClient>>;

pub fn router(config: &CoreConfig) -> Router {
    Router::new().layer(github_oauth_extension(
        config,
        format!("http://localhost:9000/auth/github/callback"),
    ))
}

fn github_oauth_extension(config: &CoreConfig, redirect_url: String) -> GithubOauthExtension {
    let github_client_id = ClientId::new(config.github_oauth.id.clone());
    let github_client_secret = ClientSecret::new(config.github_oauth.secret.clone());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("invalid auth url");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"));
    Extension(Arc::new(client))
}
