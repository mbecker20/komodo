use monitor_types::periphery_api::requests;
use periphery_client::PeripheryClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let periphery = PeripheryClient::new("http://localhost:9001", "monitor_passkey");

    let version = periphery.request(requests::GetVersion {}).await?;
    println!("{version:?}");

    Ok(())
}
