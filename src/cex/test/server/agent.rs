use std::net::SocketAddr;
use std::time::Duration;

use log::error;

use super::client::handle_clients;

pub async fn start(bind: SocketAddr) -> anyhow::Result<()> {
    while let Err(err) = handle_clients(bind).await {
        error!("Publish messages to clients error: {}", err);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
