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
    let (header, info) = match alert {
        Alert::ServerUnreachable { id, name, region } => (String::new(), String::new()),
        Alert::ServerCpu {
            id,
            name,
            region,
            state,
            percentage,
            top_procs,
        } => (String::new(), String::new()),
        Alert::ServerMem {
            id,
            name,
            region,
            state,
            used,
            total,
            top_procs,
        } => (String::new(), String::new()),
        Alert::ServerDisk {
            id,
            name,
            region,
            state,
            path,
            used,
            total,
        } => (String::new(), String::new()),
        Alert::ContainerStateChange {
            id,
            name,
            server,
            from,
            to,
        } => (String::new(), String::new()),
    };
    let slack = slack::Client::new(url);
    slack.send_message_with_header(header, info).await?;
    Ok(())
}

pub async fn send_custom_alert(url: &str, alert: &Alert) -> anyhow::Result<()> {
    Ok(())
}
