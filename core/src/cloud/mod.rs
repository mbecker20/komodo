pub mod aws;

pub enum InstanceCleanupData {
	Aws { instance_id: String, region: String, }
}