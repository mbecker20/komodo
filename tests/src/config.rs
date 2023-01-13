use dotenv::dotenv;
use monitor_client::MonitorClient;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    pub url: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub secret: Option<String>,
}

pub async fn load() -> MonitorClient {
    dotenv().ok();
    let env: Env = envy::from_env().expect("failed to parse env");
    if let Some(token) = env.token {
        MonitorClient::new_with_token(&env.url, token)
    } else if let Some(username) = env.username {
        if let Some(password) = env.password {
            match MonitorClient::new_with_password(&env.url, &username, &password).await {
                Ok(monitor) => monitor,
                Err(e) => {
                    println!(
                        "could not login with username password\n{e:#?}\ntrying to make account..."
                    );
                    MonitorClient::new_with_new_account(&env.url, username, password)
                        .await
                        .expect("failed at logging in with new account")
                }
            }
        } else if let Some(secret) = env.secret {
            MonitorClient::new_with_secret(&env.url, username, secret)
                .await
                .expect("failed to log in with username / secret")
        } else {
            panic!("must provide either password or secret to login in along with username")
        }
    } else {
        panic!("must provide either token, username / password, or username / secret")
    }
}
