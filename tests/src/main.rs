use periphery_client::{requests, PeripheryClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let periphery = PeripheryClient::new("http://localhost:9001", "monitor_passkey");

    let version = periphery.request(requests::GetVersion {}).await?;
    println!("{version:?}");

    let system_info = periphery.request(requests::GetSystemInformation {}).await?;
    println!("{system_info:#?}");

    let processes = periphery.request(requests::GetSystemProcesses {}).await?;
    // println!("{system_stats:#?}");

    let periphery_process = processes.into_iter().find(|p| p.name.contains("periphery"));
    println!("{periphery_process:#?}");

    let accounts = periphery.request(requests::GetAccounts {}).await?;
    println!("{accounts:#?}");

    let secrets = periphery.request(requests::GetSecrets {}).await?;
    println!("{secrets:#?}");

    Ok(())
}
