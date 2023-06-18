use monitor_client::MonitorClient;
use monitor_types::requests::api;

#[allow(unused)]
pub async fn tests() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let monitor = MonitorClient::new_from_env().await?;

    let secret = monitor
        .api(api::CreateLoginSecret {
            name: String::from("tests"),
            expires: None,
        })
        .await?;

    println!("{secret:#?}");

    Ok(())
}
