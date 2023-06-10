use monitor_types::api::periphery::requests;
use periphery_client::PeripheryClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let periphery = PeripheryClient::new("http://localhost:9001", "monitor_passkey");

    let version = periphery.request(requests::GetVersion {}).await?;
    println!("{version:?}");

    let system_info = periphery.request(requests::GetSystemInformation {}).await?;
    println!("{system_info:#?}");

    let system_stats = periphery.request(requests::GetAllSystemStats {}).await?;
    // println!("{system_stats:#?}");

    let periphery_process = system_stats.processes.into_iter().find(|p| p.name.contains("periphery"));
    println!("{periphery_process:#?}");

    Ok(())
}
