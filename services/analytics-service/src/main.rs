use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!(
        service = env!("CARGO_PKG_NAME"),
        "starting service scaffold"
    );
    Ok(())
}
