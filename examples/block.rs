extern crate tokio_core;
extern crate near_rpc;

use near_rpc::futures::Future;
use serde_json::json;
use serde_json::{to_value, Result as JsonResult, Value};
use near_rpc::types::{to_base64, BlockId, ChunkId, BlockIdOrFinality};

use borsh::BorshSerialize;

const MAX_PARALLEL_REQUESTS: usize = 64;

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();

    let near = near_rpc::NearApi::new(
        near_rpc::transports::Http::with_event_loop("http://localhost:3030", &event_loop.handle(), MAX_PARALLEL_REQUESTS)
            .unwrap(),
    );

    let block = near.rpc().block().map(|block_info| {
        println!("blockInfo: {:?}", block_info);
    });

    event_loop.run(block).unwrap();
}
