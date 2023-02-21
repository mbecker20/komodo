use std::time::Duration;

use anyhow::{anyhow, Context};
use aws_sdk_ec2::model::{
    BlockDeviceMapping, EbsBlockDevice, InstanceNetworkInterfaceSpecification, InstanceStateChange,
    InstanceStateName, InstanceStatus, ResourceType, Tag, TagSpecification,
};
pub use aws_sdk_ec2::{
    model::InstanceType,
    output::{DescribeInstanceStatusOutput, TerminateInstancesOutput},
    Client, Region,
};
use types::Server;

pub async fn create_ec2_client(
    region: String,
    access_key_id: &str,
    secret_access_key: String,
) -> Client {
    // There may be a better way to pass these keys to client
    std::env::set_var("AWS_ACCESS_KEY_ID", access_key_id);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", secret_access_key);
    let region = Region::new(region);
    let config = aws_config::from_env().region(region).load().await;
    let client = Client::new(&config);
    client
}

pub struct Ec2Instance {
    pub instance_id: String,
    pub server: Server,
}

const POLL_RATE_SECS: u64 = 2;
const MAX_POLL_TRIES: usize = 30;

/// this will only resolve after the instance is running
/// should still poll the periphery agent after creation
pub async fn create_instance_with_ami(
    client: &Client,
    instance_name: &str,
    ami_id: &str,
    instance_type: &str,
    subnet_id: &str,
    security_group_ids: Vec<String>,
    volume_size_gb: i32,
    key_pair_name: &str,
    assign_public_ip: bool,
) -> anyhow::Result<Ec2Instance> {
    let instance_type = InstanceType::from(instance_type);
    if let InstanceType::Unknown(t) = instance_type {
        return Err(anyhow!("unknown instance type {t:?}"));
    }
    let res = client
        .run_instances()
        .image_id(ami_id)
        .instance_type(instance_type)
        .block_device_mappings(
            BlockDeviceMapping::builder()
                .set_device_name(String::from("/dev/sda1").into())
                .set_ebs(
                    EbsBlockDevice::builder()
                        .volume_size(volume_size_gb)
                        .build()
                        .into(),
                )
                .build(),
        )
        .network_interfaces(
            InstanceNetworkInterfaceSpecification::builder()
                .subnet_id(subnet_id)
                .associate_public_ip_address(assign_public_ip)
                .set_groups(security_group_ids.into())
                .device_index(0)
                .build(),
        )
        .key_name(key_pair_name)
        .tag_specifications(
            TagSpecification::builder()
                .tags(Tag::builder().key("Name").value(instance_name).build())
                .resource_type(ResourceType::Instance)
                .build(),
        )
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .context("failed to start builder ec2 instance")?;
    let instance = res
        .instances()
        .ok_or(anyhow!("got None for created instances"))?
        .get(0)
        .ok_or(anyhow!("instances array is empty"))?;
    let instance_id = instance
        .instance_id()
        .ok_or(anyhow!("instance does not have instance_id"))?
        .to_string();
    for _ in 0..MAX_POLL_TRIES {
        let state_name = get_ec2_instance_state_name(&client, &instance_id).await?;
        if state_name == Some(InstanceStateName::Running) {
            let ip = if assign_public_ip {
                get_ec2_instance_public_ip(client, &instance_id).await?
            } else {
                instance
                    .private_ip_address()
                    .ok_or(anyhow!("instance does not have private ip"))?
                    .to_string()
            };
            let server = Server {
                address: format!("http://{ip}:8000"),
                ..Default::default()
            };
            return Ok(Ec2Instance {
                instance_id,
                server,
            });
        }
        tokio::time::sleep(Duration::from_secs(POLL_RATE_SECS)).await;
    }
    Err(anyhow!("instance not running after polling"))
}

pub async fn get_ec2_instance_status(
    client: &Client,
    instance_id: &str,
) -> anyhow::Result<Option<InstanceStatus>> {
    let status = client
        .describe_instance_status()
        .instance_ids(instance_id)
        .send()
        .await
        .context("failed to get instance status from aws")?
        .instance_statuses()
        .ok_or(anyhow!("instance statuses is None"))?
        .get(0)
        .map(|s| s.to_owned());
    Ok(status)
}

pub async fn get_ec2_instance_state_name(
    client: &Client,
    instance_id: &str,
) -> anyhow::Result<Option<InstanceStateName>> {
    let status = get_ec2_instance_status(client, instance_id).await?;
    if status.is_none() {
        return Ok(None);
    }
    let state = status
        .unwrap()
        .instance_state()
        .ok_or(anyhow!("instance state is None"))?
        .name()
        .ok_or(anyhow!("instance state name is None"))?
        .to_owned();
    Ok(Some(state))
}

pub async fn get_ec2_instance_public_ip(
    client: &Client,
    instance_id: &str,
) -> anyhow::Result<String> {
    let ip = client
        .describe_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .context("failed to get instance status from aws")?
        .reservations()
        .ok_or(anyhow!("instance reservations is None"))?
        .get(0)
        .ok_or(anyhow!("instance reservations is empty"))?
        .instances()
        .ok_or(anyhow!("instances is None"))?
        .get(0)
        .ok_or(anyhow!("instances is empty"))?
        .public_ip_address()
        .ok_or(anyhow!("instance has no public ip"))?
        .to_string();

    Ok(ip)
}

pub async fn terminate_ec2_instance(
    client: &Client,
    instance_id: &str,
) -> anyhow::Result<InstanceStateChange> {
    let res = client
        .terminate_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .context("failed to terminate instance from aws")?
        .terminating_instances()
        .ok_or(anyhow!("terminating instances is None"))?
        .get(0)
        .ok_or(anyhow!("terminating instances is empty"))?
        .to_owned();
    Ok(res)
}
