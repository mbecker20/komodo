pub mod aws;

pub enum BuildCleanupData {
  Server { repo_name: String },
  Aws { instance_id: String, region: String },
}
