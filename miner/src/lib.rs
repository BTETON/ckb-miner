mod client;
mod config;
mod miner;
mod worker;

pub use crate::client::Client;
pub use crate::config::MinerConfig;
pub use crate::miner::Miner;

use ckb_jsonrpc_types::BlockTemplate;
use ckb_types::packed::Block;
use std::convert::From;

#[derive(Debug, Clone)]
pub struct Work {
    work_id: u64,
    block: Block,
}

impl From<BlockTemplate> for Work {
    fn from(block_template: BlockTemplate) -> Work {
        let work_id = block_template.work_id.clone();
        let block: Block = block_template.into();

        Work {
            work_id: work_id.into(),
            block,
        }
    }
}
