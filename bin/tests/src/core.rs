use monitor_client::MonitorClient;
use monitor_types::{
    entities::{repo::PartialRepoConfig, server::PartialServerConfig},
    requests::{read, write},
};

#[allow(unused)]
pub async fn tests() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let monitor = MonitorClient::new_from_env().await?;

    let mut res = monitor.read(read::ListServers { query: None }).await?;
    let server_id = res.pop().unwrap().id;

    

    Ok(())
}

#[allow(unused)]
async fn create_repo(monitor: &MonitorClient) -> anyhow::Result<()> {
    let mut res = monitor.read(read::ListServers { query: None }).await?;
    let server_id = res.pop().unwrap().id;

    let repo = monitor
        .write(write::CreateRepo {
            name: String::from("monitor"),
            config: PartialRepoConfig {
                server_id: server_id.into(),
                repo: "mbecker20/monitor".to_string().into(),
                branch: "next".to_string().into(),
                ..Default::default()
            },
        })
        .await?;

    println!("{repo:#?}");

    Ok(())
}

#[allow(unused)]
async fn create_server(monitor: &MonitorClient) -> anyhow::Result<()> {
    let res = monitor
        .write(write::CreateServer {
            name: String::from("max-apt"),
            config: PartialServerConfig {
                address: "http://localhost:8001".to_string().into(),
                ..Default::default()
            },
        })
        .await?;

    println!("{res:#?}");

    Ok(())
}

#[allow(unused)]
async fn create_secret() -> anyhow::Result<()> {
    let monitor =
        MonitorClient::new_with_new_account("http://localhost:9001", "defi moses", "jah guide")
            .await?;

    let secret = monitor
        .write(write::CreateLoginSecret {
            name: "tests".to_string(),
            expires: None,
        })
        .await?;

    println!("{secret:#?}");

    Ok(())
}
