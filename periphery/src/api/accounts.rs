use axum::{extract::Path, Extension, Json};
use serde::Deserialize;
use types::AccountType;

use crate::PeripheryConfigExtension;

#[derive(Deserialize, Debug)]
pub struct GetAccountsPath {
    account_type: AccountType,
}

pub async fn get_accounts(
    Extension(config): PeripheryConfigExtension,
    Path(path): Path<GetAccountsPath>,
) -> Json<Vec<String>> {
    match path.account_type {
        AccountType::Github => {
            let mut accounts: Vec<String> =
                config.github_accounts.keys().map(|k| k.clone()).collect();
            accounts.sort();
            Json(accounts)
        }
        AccountType::Docker => {
            let mut accounts: Vec<String> =
                config.docker_accounts.keys().map(|k| k.clone()).collect();
            accounts.sort();
            Json(accounts)
        }
    }
}
