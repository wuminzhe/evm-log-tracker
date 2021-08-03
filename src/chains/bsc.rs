use crate::{EvmChain, EvmClient, Result};
use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;

/// Binance Smart Chain
pub struct Bsc;

#[async_trait]
impl EvmChain for Bsc {
    const NAME: &'static str = "Bsc";

    async fn next_range(from: u64, step: u64, client: &EvmClient) -> Result<(u64, u64)> {
        // if step > 5000 {
        //     Err()
        // }
        let latest = client.get_latest_block_number().await?;
        let to = from + step;
        if to > latest { // 走的太快了
            sleep(Duration::from_secs(30)).await;
            Bsc::next_range(from, step, client).await
        } else {
            Ok((from, to))
        }
    }
}
