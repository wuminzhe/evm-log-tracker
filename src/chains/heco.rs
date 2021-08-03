use crate::{EvmChain, EvmClient, Result};
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

/// Huobi ECO Chain
pub struct Heco;

#[async_trait]
impl EvmChain for Heco {
    const NAME: &'static str = "Heco";

    async fn next_range(from: u64, step: u64, client: &EvmClient) -> Result<(u64, u64)> {
        let latest = client.get_latest_block_number().await?;
        let to = from + step;
        if to > latest { // 走的太快了
            sleep(Duration::from_secs(30)).await;
            Heco::next_range(from, step, client).await
        } else {
            Ok((from, to))
        }
    }
}
