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

    let block_id = BlockId::Height(170);
    let chunk_id = ChunkId::BlockShardId(block_id, 0);
    let bytes = chunk_id.try_to_vec().unwrap();
    let chunk_bytes = to_base64(&bytes);
    let chunk_value = serde_json::to_value(chunk_bytes).unwrap();

    let chunk = near.rpc().chunk(Some(chunk_value)).map(|chunk| {
        println!("chunk: {:?}", chunk);
    });

    event_loop.run(chunk).unwrap();

}