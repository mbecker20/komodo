use anyhow::{anyhow, Context};
use monitor_types::entities::{alert::Alert, alerter::*};
use reqwest::StatusCode;

pub async fn send_alert(alerter: &Alerter, alert: &Alert) -> anyhow::Result<()> {
    match &alerter.config {
        AlerterConfig::Slack(SlackAlerterConfig { url }) => send_slack_alert(url, alert).await,
        AlerterConfig::Custom(CustomAlerterConfig { url }) => send_custom_alert(url, alert).await,
    }
}

pub async fn send_slack_alert(url: &str, alert: &Alert) -> anyhow::Result<()> {
    #[allow(unused)]
    let message = match alert {
        Alert::ServerUnreachable { id, name, region } => String::new(),
        Alert::ServerCpu {
            id,
            name,
            region,
            state,
            percentage,
            top_procs,
        } => String::new(),
        Alert::ServerMem {
            id,
            name,
            region,
            state,
            used,
            total,
            top_procs,
        } => String::new(),
        Alert::ServerDisk {
            id,
            name,
            region,
            state,
            path,
            used,
            total,
        } => String::new(),
        Alert::ContainerStateChange {
            id,
            name,
            server,
            from,
            to,
        } => String::new(),
    };
    let res = reqwest::Client::new()
        .post(url)
        .send()
        .await
        .context("failed to make request to slack")?;
    let status = res.status();
    if status != StatusCode::OK {
        let text = res
            .text()
            .await
            .context("failed to get slack alert response as test")?;
        return Err(anyhow!("{status} | {text}"));
    }
    Ok(())
}

pub async fn send_custom_alert(url: &str, alert: &Alert) -> anyhow::Result<()> {
    Ok(())
}
