use typeshare::typeshare;

pub mod entities;
pub mod requests;
pub mod permissioned;
pub mod busy;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
#[typeshare(serialized_as = "any")]
pub type MongoDocument = mungos::mongodb::bson::Document;

fn i64_is_zero(n: &I64) -> bool {
    *n == 0
}
