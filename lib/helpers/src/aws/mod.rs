use anyhow::{anyhow, Context};
use aws_sdk_ec2::{model::InstanceType, Client, Region};
use periphery_client::PeripheryClient;

/// must provide creds in env or with ~/.aws/credentials
pub async fn create_ec2_client(
    region: String,
    access_key_id: &str,
    secret_access_key: String,
) -> Client {
    std::env::set_var("AWS_ACCESS_KEY_ID", access_key_id);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", secret_access_key);
    let region = Region::new(region);
    let config = aws_config::from_env().region(region).load().await;
    let client = Client::new(&config);
    std::env::remove_var("AWS_ACCESS_KEY_ID");
    std::env::remove_var("AWS_SECRET_ACCESS_KEY");
    client
}

pub struct Ec2Instance {
    pub id: String,
    pub periphery_address: String,
}

const POLL_RATE_SECS: i32 = 1;
const MAX_POLL_TRIES: usize = 30;

/// should poll the periphery
pub async fn create_instance_with_ami(
    client: Client,
    ami_id: &str,
    instance_type: &str,
    security_groups_ids: &Vec<String>,
) -> anyhow::Result<Ec2Instance> {
    let instance_type = InstanceType::from(instance_type);
    if let InstanceType::Unknown(t) = instance_type {
        return Err(anyhow!("unknown instance type {t:?}"));
    }
    let mut req = client
        .run_instances()
        .image_id(ami_id)
        .instance_type(instance_type)
        .min_count(1)
        .max_count(1);

    for sg_id in security_groups_ids {
        req = req.security_group_ids(sg_id);
    }

    let res = req
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
    let ip = instance
        .private_ip_address()
        .ok_or(anyhow!("instance does not have private ip"))?;
    // client.describe_instances().
    let periphery_address = format!("http://{ip}:8000");
    Ok(Ec2Instance {
        id: instance_id,
        periphery_address,
    })
}
