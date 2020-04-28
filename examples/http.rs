extern crate tokio_core;
extern crate near_rpc;

use near_rpc::futures::Future;
use serde_json::json;
use serde_json::{to_value, Result as JsonResult, Value};
use near_rpc::types::{BlockId, ChunkId};
use hyper::Chunk;

const MAX_PARALLEL_REQUESTS: usize = 64;

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();

    let near = near_rpc::NearApi::new(
        near_rpc::transports::Http::with_event_loop("http://localhost:3030", &event_loop.handle(), MAX_PARALLEL_REQUESTS)
            .unwrap(),
    );
    let validators = near.rpc().networkInfo().map(|validators| {
        println!("validators: {:?}", validators);
    });

    let status = near.rpc().status().map(|status| {
        println!("status: {:?}", status);
    });

    event_loop.run(status).unwrap();
}
