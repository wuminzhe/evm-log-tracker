use evm_log_tracker::{
    LogsHandler, Result, EvmLogTracker, EvmClient, DefaultLogsHandler, Ethereum
};
use array_bytes::hex2bytes_unchecked as bytes;
use web3::{
    Web3,
    types::{H160, Log, H256, Address, U256, Bytes},
    transports::Http,
};

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

pub struct Erc721LogsHandler;

#[async_trait]
impl LogsHandler for Erc721LogsHandler {
    async fn handle(
        &self,
        client: &EvmClient,
        _topics_list: &Vec<(Option<H160>, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        for log in logs {
            if client.is_erc721(log.address).await? {
                info!("{:?}", log.address);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let web3 = Web3::new(
        Http::new("https://mainnet.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap(),
    );

    // let contract_address = H160::from_slice(&bytes(contract_address));
    // let result = is_erc721(web3, contract_address).await?;
    // println!("result: {}", result);

    // let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
    // let contract_address = H160::from_slice(&bytes(contract_address));

    // OwnershipTransferred
    let topics = &vec!["0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0"];
    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect();

    let client = EvmClient::new(web3);
    let mut tracker = EvmLogTracker::<Ethereum, _>::new(
        client,
        vec![(None, topics)],
        Erc721LogsHandler {},
        10000000,
        100000,
        10,
    );

    tracker.start().await;
    Ok(())
}

