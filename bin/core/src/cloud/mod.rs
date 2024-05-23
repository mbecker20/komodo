pub mod aws;
pub mod hetzner;

#[derive(Debug)]
pub enum BuildCleanupData {
  Server { repo_name: String },
  Aws { instance_id: String, region: String },
}
