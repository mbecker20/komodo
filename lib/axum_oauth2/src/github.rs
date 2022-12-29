use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context};
use axum::Extension;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::random_string;

pub type GithubOauthExtension = Extension<Arc<GithubOauthClient>>;

pub struct GithubOauthClient {
    http: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    scopes: String,
    states: Mutex<Vec<String>>,
    user_agent: String,
}

impl GithubOauthClient {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        scopes: &[&str],
        user_agent: String,
    ) -> GithubOauthClient {
        GithubOauthClient {
            http: reqwest::Client::new(),
            client_id,
            client_secret,
            redirect_uri,
            user_agent,
            scopes: urlencoding::encode(&scopes.join(" ")).to_string(),
            states: Default::default(),
        }
    }

    pub fn get_login_redirect_url(&self) -> String {
        let state = random_string(40);
        let redirect_url = format!(
            "https://github.com/login/oauth/authorize?state={state}&client_id={}&redirect_uri={}&scope={}",
            self.client_id, self.redirect_uri, self.scopes
        );
        {
            let mut states = self.states.lock().unwrap();
            states.push(state);
            // println!("{states:#?}");
        }
        redirect_url
    }

    pub fn check_state(&self, state: &str) -> bool {
        let mut contained = false;
        self.states.lock().unwrap().retain(|s| {
            if s.as_str() == state {
                contained = true;
                false
            } else {
                true
            }
        });
        contained
    }

    pub async fn get_access_token(&self, code: &str) -> anyhow::Result<AccessTokenResponse> {
        self.post::<(), _>(
            "https://github.com/login/oauth/access_token",
            &[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("code", code),
            ],
            None,
            None,
        )
        .await
        .context("failed to get github access token using code")
    }

    pub async fn get_github_user(&self, token: &str) -> anyhow::Result<GithubUserResponse> {
        self.get("https://api.github.com/user", &[], Some(token))
            .await
            .context("failed to get github user using access token")
    }

    async fn get<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &[(&str, &str)],
        bearer_token: Option<&str>,
    ) -> anyhow::Result<R> {
        let mut req = self
            .http
            .get(endpoint)
            .query(query)
            .header("User-Agent", &self.user_agent);

        if let Some(bearer_token) = bearer_token {
            req = req.header("Authorization", format!("Bearer {bearer_token}"));
        }

        let res = req.send().await.context("failed to reach github")?;

        let status = res.status();

        if status == StatusCode::OK {
            let body = res
                .json()
                .await
                .context("failed to parse body into expected type")?;
            Ok(body)
        } else {
            let text = res
                .text()
                .await
                .context(format!("status: {status} | failed to get response text"))?;
            Err(anyhow!("status: {status} | text: {text}"))
        }
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: &[(&str, &str)],
        body: Option<&B>,
        bearer_token: Option<&str>,
    ) -> anyhow::Result<R> {
        let mut req = self
            .http
            .post(endpoint)
            .query(query)
            .header("Accept", "application/json")
            .header("User-Agent", &self.user_agent);

        if let Some(body) = body {
            req = req.json(body);
        }

        if let Some(bearer_token) = bearer_token {
            req = req.header("Authorization", format!("Bearer {bearer_token}"));
        }

        let res = req.send().await.context("failed to reach github")?;

        let status = res.status();

        if status == StatusCode::OK {
            let body = res
                .json()
                .await
                .context("failed to parse POST body into expected type")?;
            Ok(body)
        } else {
            let text = res.text().await.context(format!(
                "method: POST | status: {status} | failed to get response text"
            ))?;
            Err(anyhow!("method: POST | status: {status} | text: {text}"))
        }
    }
}

#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct GithubUserResponse {
    pub login: String,
    pub id: u128,
    pub avatar_url: String,
    pub email: Option<String>,
}
