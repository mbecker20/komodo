use derive_builder::Builder;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id, Document,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::{update::ResourceTargetVariant, PermissionsMap};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct Resource<Config, Info: Default> {
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
  pub permissions: PermissionsMap,

  #[serde(default)]
  #[builder(setter(skip))]
  pub updated_at: I64,

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
  pub tags: Vec<String>,
  pub info: Info,
}

/// Passing empty Vec is the same as not filtering by that field
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResourceQuery<T> {
  #[serde(default)]
  pub names: Vec<String>,
  #[serde(default)]
  pub tags: Vec<String>,
  #[serde(default)]
  pub specific: T,
}

pub trait AddFilters {
  fn add_filters(&self, _filters: &mut Document) {}
}

impl AddFilters for () {}

impl<T: AddFilters> AddFilters for ResourceQuery<T> {
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

#[derive(Default)]
pub struct ResourceQueryBuilder<T> {
  pub names: Option<Vec<String>>,
  pub tags: Option<Vec<String>>,
  pub specific: Option<T>,
}

impl<T: Default> ResourceQueryBuilder<T> {
  pub fn build(self) -> ResourceQuery<T> {
    ResourceQuery {
      names: self.names.unwrap_or_default(),
      tags: self.tags.unwrap_or_default(),
      specific: self.specific.unwrap_or_default(),
    }
  }

  pub fn names(
    mut self,
    names: impl Into<Vec<String>>,
  ) -> ResourceQueryBuilder<T> {
    self.names = Some(names.into());
    self
  }

  pub fn tags(
    mut self,
    tags: impl Into<Vec<String>>,
  ) -> ResourceQueryBuilder<T> {
    self.tags = Some(tags.into());
    self
  }

  pub fn specific(mut self, specific: T) -> ResourceQueryBuilder<T> {
    self.specific = Some(specific);
    self
  }
}
