extern crate tokio_core;
extern crate near_rpc;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::convert::TryFrom;
use std::path::Path;
use serde_json::{to_value, Result as JsonResult, Value, json};

use near_rpc::futures::Future;

use crate::near_rpc::types::{to_base64, CryptoHash, RpcAutoSendConfig, SignedTransaction, Balance};
use near_crypto::{InMemorySigner, KeyType};
use borsh::BorshSerialize;

const MAX_PARALLEL_REQUESTS: usize = 64;


fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();

    let near = near_rpc::NearApi::new(
        near_rpc::transports::Http::with_event_loop("http://localhost:3030", &event_loop.handle(), MAX_PARALLEL_REQUESTS)
            .unwrap(),
    );
    let params:(String,String) =("access_key/node0".to_base(),"".to_base());
    let bytes = params.try_to_vec().unwrap();
    let config_bytes = to_base64(&bytes);
    let config_value = serde_json::to_value(config_bytes).unwrap();

    let account = near.rpc().query(Some(config_value)).map(|account| {
        println!("status: {:?}", account);
    });
}