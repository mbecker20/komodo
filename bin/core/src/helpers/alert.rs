use anyhow::{anyhow, Context};
use monitor_types::entities::{alert::Alert, alerter::*, server::stats::SystemProcess};
use reqwest::StatusCode;
use slack::types::Block;

pub async fn send_alert(alerter: &Alerter, alert: &Alert) -> anyhow::Result<()> {
    match &alerter.config {
        AlerterConfig::Slack(SlackAlerterConfig { url }) => send_slack_alert(url, alert).await,
        AlerterConfig::Custom(CustomAlerterConfig { url }) => send_custom_alert(url, alert).await,
    }
}

pub async fn send_slack_alert(url: &str, alert: &Alert) -> anyhow::Result<()> {
    let (text, blocks) = match alert {
        Alert::ServerUnreachable { id, name, region } => {
            let region = fmt_region(region);
            let text = format!("CRITICAL 🚨 | *{name}*{region} is *unreachable* ❌");
            let blocks = vec![
                Block::header("CRITICAL 🚨"),
                Block::section(format!("*{name}*{region} is *unreachable* ❌")),
            ];
            (text, blocks.into())
        }
        Alert::ServerCpu {
            id,
            name,
            region,
            state,
            percentage,
            top_procs,
        } => {
            let region = fmt_region(region);
            let text =
                format!("{state} 🚨 | *{name}*{region} cpu usage at *{percentage:.1}%* 📈 🚨");
            let blocks = vec![
                Block::header(format!("{state} 🚨")),
                Block::section(format!(
                    "*{name}*{region} cpu usage at *{percentage:.1}%* 📈 🚨"
                )),
                Block::section(format!("*top cpu processes*{}", fmt_top_procs(top_procs))),
            ];
            (text, blocks.into())
        }
        Alert::ServerMem {
            id,
            name,
            region,
            state,
            used_gb,
            total_gb,
            top_procs,
        } => {
            let region = fmt_region(region);
            let percentage = 100.0 * used_gb / total_gb;
            let text =
                format!("{state} 🚨 | *{name}*{region} memory usage at *{percentage:.1}%* 💾 🚨");
            let blocks = vec![
                Block::header(format!("{state} 🚨")),
                Block::section(format!(
                    "*{name}*{region} memory usage at *{percentage:.1}%* 💾 🚨"
                )),
                Block::section(format!("using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*")),
                Block::section(format!("*top cpu processes*{}", fmt_top_procs(top_procs))),
            ];
            (text, blocks.into())
        }
        Alert::ServerDisk {
            id,
            name,
            region,
            state,
            path,
            used_gb,
            total_gb,
        } => {
            let region = fmt_region(region);
            let percentage = 100.0 * used_gb / total_gb;
            let text =
                format!("{state} 🚨 | *{name}*{region} disk usage at *{percentage:.1}%* | mount point: *{path}* 💿 🚨");
            let blocks = vec![
                Block::header(format!("{state} 🚨")),
                Block::section(format!(
                    "*{name}*{region} disk usage at *{percentage:.1}%* 💿 🚨"
                )),
                Block::section(format!(
                    "mount point: {path} | using *{used_gb:.1} GiB* / *{total_gb:.1} GiB*"
                )),
            ];
            (text, blocks.into())
        }
        Alert::ServerTemp {
            id,
            name,
            region,
            state,
            temp,
            max,
        } => {
            let region = fmt_region(region);
            let header = format!("");
            let info = format!("");
            (String::new(), None)
        }
        Alert::ContainerStateChange {
            id,
            name,
            server,
            from,
            to,
        } => {
            let header = format!("");
            let info = format!("");
            (String::new(), None)
        }
    };
    let slack = slack::Client::new(url);
    slack.send_message(text, blocks).await?;
    Ok(())
}

pub async fn send_custom_alert(url: &str, alert: &Alert) -> anyhow::Result<()> {
    let res = reqwest::Client::new()
        .post(url)
        .json(alert)
        .send()
        .await
        .context("failed at post request to alerter")?;
    let status = res.status();
    if status != StatusCode::OK {
        let text = res
            .text()
            .await
            .context("failed to get response text on alerter response")?;
        return Err(anyhow!("post to alerter failed | {status} | {text}"));
    }
    Ok(())
}

fn fmt_region(region: &Option<String>) -> String {
    match region {
        Some(region) => format!(" ({region})"),
        None => String::new(),
    }
}

fn fmt_top_procs(top_procs: &[SystemProcess]) -> String {
    top_procs
        .iter()
        .enumerate()
        .map(|(i, p)| {
            format!(
                "\n{}. *{}* | *{:.1}%* CPU | *{:.1} GiB* MEM",
                i + 1,
                p.name,
                p.cpu_perc,
                p.mem_mb / 1024.0,
            )
        })
        .collect::<Vec<_>>()
        .join("")
}
