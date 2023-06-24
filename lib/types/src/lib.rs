use typeshare::typeshare;

pub mod busy;
pub mod entities;
pub mod permissioned;
pub mod requests;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
#[typeshare(serialized_as = "any")]
pub type MongoDocument = mungos::mongodb::bson::Document;

fn i64_is_zero(n: &I64) -> bool {
    *n == 0
}

pub fn all_logs_success(logs: &Vec<entities::update::Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}

pub fn optional_string(string: &str) -> Option<String> {
    if string.is_empty() {
        None
    } else {
        Some(string.to_string())
    }
}