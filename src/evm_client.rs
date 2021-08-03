use crate::Result;
use web3::{
    transports::http::Http,
    types::{BlockNumber, FilterBuilder, Log, SyncState, H160, H256, U64},
    Web3,
    contract::{Contract, Options},
};

pub struct EvmClient {
    web3: Web3<Http>,
}

impl EvmClient {
    pub fn new(web3: Web3<Http>) -> EvmClient {
        EvmClient { web3 }
    }
}

impl EvmClient {
    pub async fn get_logs(
        &self,
        contract_address: Option<H160>,
        topics: Vec<H256>,
        from: u64,
        to: u64,
    ) -> Result<Vec<Log>> {
        // build filter
        let filter_builder = if let Some(contract) = contract_address {
            FilterBuilder::default()
                .address(vec![contract])
                .topics(Some(topics.clone()), None, None, None)
        } else {
            FilterBuilder::default()
                .topics(Some(topics.clone()), None, None, None)
        };

        let filter = filter_builder
            .clone()
            .from_block(BlockNumber::Number(U64::from(from)))
            .to_block(BlockNumber::Number(U64::from(to)))
            .build();

        Ok(self.web3.eth().logs(filter).await?)
    }

    pub async fn get_latest_block_number(&self) -> Result<u64> {
        let eth = self.web3.eth();
        let sync_state = eth.syncing().await?;

        let latest_block_number = match sync_state {
            // TOOD: what the difference between eth_blockNumber and eth_getBlockByNumber("latest", false)
            SyncState::NotSyncing => eth.block_number().await?.as_u64(),
            SyncState::Syncing(info) => info.current_block.as_u64(),
        };
        Ok(latest_block_number)
    }

    pub async fn is_erc721(&self, contract_address: H160) -> Result<bool> {
        let contract = Contract::from_json(
            self.web3.eth(),
            contract_address,
            include_bytes!("./contracts/erc721.json"),
        )?;
        let interface_id = [
            128u8, 172, 88, 205
        ];
        match contract.query("supportsInterface", (interface_id,), None, Options::default(), None).await {
            Ok(result) => Ok(result),
            Err(err) => {
                error!("{}, {:?}", contract_address.to_string(), err);
                Ok(false)
            }
        }
    }
}
