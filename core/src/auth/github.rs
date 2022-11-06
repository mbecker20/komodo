use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use types::CoreConfig;

pub fn github_oauth_client(
    secrets: &CoreConfig,
    redirect_url: String,
) -> anyhow::Result<BasicClient> {
    let github_client_id = ClientId::new(secrets.github_oauth.id.clone());
    let github_client_secret = ClientSecret::new(secrets.github_oauth.secret.clone());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
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
    Ok(client)
}
