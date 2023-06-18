mod core;
mod periphery;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // periphery::tests().await?;
    core::tests().await?;

    Ok(())
}
