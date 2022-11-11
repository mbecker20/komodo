use axum::{extract::Path, Extension, Json};
use mungos::Deserialize;
use types::AccountType;

use crate::BuilderSecretsExtension;

#[derive(Deserialize, Debug)]
pub struct GetAccountsPath {
    account_type: AccountType,
}

pub async fn get_accounts(
    Extension(secrets): BuilderSecretsExtension,
    Path(path): Path<GetAccountsPath>,
) -> Json<Vec<String>> {
    match path.account_type {
        AccountType::Github => {
            let mut accounts: Vec<String> =
                secrets.github_accounts.keys().map(|k| k.clone()).collect();
            accounts.sort();
            Json(accounts)
        }
        AccountType::Docker => {
            let mut accounts: Vec<String> =
                secrets.docker_accounts.keys().map(|k| k.clone()).collect();
            accounts.sort();
            Json(accounts)
        }
    }
}
