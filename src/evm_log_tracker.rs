use crate::{EvmChain, EvmClient, LogsHandler, Result};
use std::marker::PhantomData;
use std::time::Duration;
use tokio::time::sleep;
use web3::types::{Log, H160, H256};

pub struct EvmLogTracker<C: EvmChain, H: LogsHandler> {
    client: EvmClient,
    topics_list: Vec<(Option<H160>, Vec<H256>)>,
    logs_handler: H,
    from: u64,
    step: u64,
    step_in_secs: u64,
    running: bool,
    phantom: PhantomData<C>,
}

impl<C: EvmChain, H: LogsHandler> EvmLogTracker<C, H> {
    pub fn new(
        client: EvmClient,
        topics_list: Vec<(Option<H160>, Vec<H256>)>,
        logs_handler: H,
        from: u64,
        step: u64,
        step_in_secs: u64,
    ) -> Self {
        EvmLogTracker {
            client,
            topics_list,
            logs_handler,
            from,
            step,
            step_in_secs,
            running: false,
            phantom: PhantomData,
        }
    }

    pub async fn start(&mut self) {
        self.running = true;
        loop {
            match self.next().await {
                Err(err) => {
                    error!("{:?}", err);
                    sleep(Duration::from_secs(30)).await;
                }
                Ok(logs) => {
                    if let Err(err2) = self.handle(logs).await {
                        error!("{:?}", err2);
                    }
                }
            }

            if self.running == false {
                break;
            }

            sleep(Duration::from_secs(self.step_in_secs)).await;
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub async fn next(&mut self) -> Result<Vec<Log>> {
        let mut result = vec![];
        let (from, to) = self.next_range().await?;
        info!(
            "Heartbeat>>> Scanning on {} for new cross-chain transactions from {} to {} ...",
            C::NAME,
            from,
            to
        );
        for topics in &self.topics_list {
            let logs = self.client.get_logs(topics.0.clone(), topics.1.clone(), from, to).await?;
            result.extend_from_slice(&logs);
        }
        Ok(result)
    }

    async fn next_range(&mut self) -> Result<(u64, u64)> {
        let range = C::next_range(self.from, self.step, &self.client).await?;
        self.from = range.1;
        Ok(range)
    }

    async fn handle(&self, logs: Vec<Log>) -> Result<()> {
        self.logs_handler
            .handle(&self.client, &self.topics_list, logs)
            .await?;
        Ok(())
    }
}

#[tokio::test]
async fn test_ethereum() {
    use crate::DefaultLogsHandler;
    use crate::Ethereum;
    use array_bytes::hex2bytes_unchecked as bytes;
    let web3 = Web3::new(
        Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
    );

    let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
    let contract_address = H160::from_slice(&bytes(contract_address));

    let topics = &vec!["0x96635f5f1b0b05ed7e2265d4e13634378280f038e5a958227d4f383f825c2771"];
    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect();

    let client = EvmClient::new(web3);
    let mut tracker = EvmLogTracker::<Ethereum, DefaultLogsHandler>::new(
        client,
        vec![(contract_address, topics)],
        DefaultLogsHandler {},
        100,
    );

    tracker.start().await;
}

#[tokio::test]
async fn test_heco() {
    use crate::DefaultLogsHandler;
    use crate::Heco;
    use array_bytes::hex2bytes_unchecked as bytes;
    let web3 = Web3::new(Http::new("https://http-mainnet-node.huobichain.com").unwrap());

    let contract_address = "0x0981F3C078856E2491F11A5F86d26274Bb4009d2";
    let contract_address = H160::from_slice(&bytes(contract_address));

    let topics = &vec!["0x2709918445f306d3e94d280907c62c5d2525ac3192d2e544774c7f181d65af3e"];
    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect();

    let client = EvmClient::new(web3);
    let mut tracker = EvmLogTracker::<Heco, DefaultLogsHandler>::new(
        client,
        vec![(contract_address, topics)],
        DefaultLogsHandler {},
        4006177,
    );

    tracker.start().await;
}
