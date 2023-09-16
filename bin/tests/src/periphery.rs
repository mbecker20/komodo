use periphery_client::{requests, PeripheryClient};

#[allow(unused)]
pub async fn tests() -> anyhow::Result<()> {
    let periphery = PeripheryClient::new(
        "http://localhost:9001",
        "monitor_passkey",
    );

    let version = periphery.request(requests::GetVersion {}).await?;
    println!("{version:?}");

    let system_info =
        periphery.request(requests::GetSystemInformation {}).await?;
    println!("{system_info:#?}");

    let processes =
        periphery.request(requests::GetSystemProcesses {}).await?;
    // println!("{system_stats:#?}");

    let periphery_process =
        processes.into_iter().find(|p| p.name.contains("periphery"));
    println!("{periphery_process:#?}");

    let accounts =
        periphery.request(requests::GetAccounts {}).await?;
    println!("{accounts:#?}");

    let secrets = periphery.request(requests::GetSecrets {}).await?;
    println!("{secrets:#?}");

    let container_stats = periphery
        .request(requests::GetContainerStatsList {})
        .await?;
    println!("{container_stats:#?}");

    let res = periphery.request(requests::GetNetworkList {}).await?;
    println!("{res:#?}");

    let res = periphery
        .request(requests::GetContainerStats {
            name: "monitor-mongo".into(),
        })
        .await?;
    println!("{res:#?}");

    let res = periphery
        .request(requests::GetContainerLog {
            name: "monitor-mongo".into(),
            tail: 50,
        })
        .await?;
    println!("{res:#?}");

    Ok(())
}
