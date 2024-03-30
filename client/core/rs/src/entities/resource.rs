use std::str::FromStr;

use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use mungos::mongodb::bson::{
  doc, oid::ObjectId, serde_helpers::hex_string_as_object_id,
  Document,
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
  pub ids: Vec<String>,
  #[serde(default)]
  pub names: Vec<String>,
  /// Pass Vec of tag ids or tag names
  #[serde(default)]
  pub tags: Vec<String>,
  #[serde(default)]
  pub tag_behavior: TagBehavior,
  #[serde(default)]
  pub specific: T,
}

#[typeshare]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum TagBehavior {
  /// Returns resources which have strictly all the tags
  #[default]
  All,
  /// Returns resources which have one or more of the tags
  Any,
}

pub trait AddFilters {
  fn add_filters(&self, _filters: &mut Document) {}
}

impl AddFilters for () {}

impl<T: AddFilters + Default> AddFilters for ResourceQuery<T> {
  fn add_filters(&self, filters: &mut Document) {
    if !self.ids.is_empty() {
      let ids = self
        .ids
        .iter()
        .flat_map(|id| ObjectId::from_str(id))
        .collect::<Vec<_>>();
      filters.insert("_id", doc! { "$in": &ids });
    }
    if !self.names.is_empty() {
      filters.insert("name", doc! { "$in": &self.names });
    }
    if !self.tags.is_empty() {
      match self.tag_behavior {
        TagBehavior::All => {
          filters.insert("tags", doc! { "$all": &self.tags });
        }
        TagBehavior::Any => {
          let ors = self
            .tags
            .iter()
            .map(|tag| doc! { "tags": tag })
            .collect::<Vec<_>>();
          filters.insert("$or", ors);
        }
      }
    }
    self.specific.add_filters(filters);
  }
}
