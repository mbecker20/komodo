use typeshare::typeshare;

pub mod entities;
pub mod requests;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
