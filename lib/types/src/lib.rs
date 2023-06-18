use typeshare::typeshare;

pub mod requests;
pub mod entities;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
