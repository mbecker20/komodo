use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id, Document,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::update::ResourceTargetVariant;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct Resource<Config, Info: Default = ()> {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  #[builder(setter(skip))]
  pub id: MongoId,

  pub name: String,

  #[serde(default)]
  #[builder(default)]
  pub description: String,

  #[serde(default)]
  #[builder(setter(skip))]
  pub updated_at: I64,

  /// Tag Ids
  #[serde(default)]
  #[builder(default)]
  pub tags: Vec<String>,

  #[serde(default)]
  #[builder(setter(skip))]
  pub info: Info,

  pub config: Config,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceListItem<Info> {
  pub id: String,
  #[serde(rename = "type")]
  pub resource_type: ResourceTargetVariant,
  pub name: String,
  pub created_at: I64,
  /// Tag Ids
  pub tags: Vec<String>,
  pub info: Info,
}

/// Passing empty Vec is the same as not filtering by that field
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct ResourceQuery<T: Default> {
  #[serde(default)]
  pub names: Vec<String>,
  /// Pass Vec of tag ids
  #[serde(default)]
  pub tags: Vec<String>,
  #[serde(default)]
  pub specific: T,
}

pub trait AddFilters {
  fn add_filters(&self, _filters: &mut Document) {}
}

impl AddFilters for () {}

impl<T: AddFilters + Default> AddFilters for ResourceQuery<T> {
  fn add_filters(&self, filters: &mut Document) {
    if !self.names.is_empty() {
      filters.insert("name", doc! { "$in": &self.names });
    }
    if !self.tags.is_empty() {
      filters.insert("tags", doc! { "$all": &self.tags });
    }
    self.specific.add_filters(filters);
  }
}
