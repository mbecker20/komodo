use termination_signal::tokio::immediate_term_handle;

mod config;
mod state;

async fn app() -> anyhow::Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let term_signal = immediate_term_handle()?;

    let app = tokio::spawn(app());

    tokio::select! {
        res = app => return res?,
        _ = term_signal => {},
    }

    Ok(())
}
