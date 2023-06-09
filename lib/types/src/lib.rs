use typeshare::typeshare;

pub mod api;
pub mod entities;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
