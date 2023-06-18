use monitor_client::MonitorClient;

#[allow(unused)]
pub async fn tests() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let monitor =
        MonitorClient::new_with_new_account("http://localhost:9001", "defi moses", "jah guide")
            .await?;

    Ok(())
}
