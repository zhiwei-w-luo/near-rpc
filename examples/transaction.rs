extern crate tokio_core;
extern crate near_rpc;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::convert::TryFrom;
use std::path::Path;

use near_rpc::futures::Future;

use crate::near_rpc::types::{to_base64, CryptoHash, SignedTransaction, Balance};
use near_crypto::{InMemorySigner, KeyType};
use borsh::BorshSerialize;

const MAX_PARALLEL_REQUESTS: usize = 64;

fn generate_transactions(
    signer_id: &str,
    signer_seed: &str,
    starting_nonce: u64,
    block_hash: CryptoHash,
    end_nonce: u64,
) -> Vec<SignedTransaction> {
    let key_path = Path::new("/Users/Roger/.near/node0/validator_key.json");

    let signer =
        Arc::new(InMemorySigner::from_file(key_path));
    (starting_nonce..=end_nonce)
        .map(|i| {
            SignedTransaction::send_money(
                end_nonce,
                signer_id.to_string(),
                "node1".to_string(),
                &*signer,
                i as Balance,
                block_hash,
            )
        })
        .collect()
}

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();

    let n = 10000;
    let near = near_rpc::NearApi::new(
        near_rpc::transports::Http::with_event_loop("http://localhost:3030", &event_loop.handle(), MAX_PARALLEL_REQUESTS)
            .unwrap(),
    );

    let mut i = 1;
    while i < 10000 {
        let mut hash_string = String::from("");
        let get_hash = near.rpc().block_hash().map(|value| {
            hash_string = String::from(value.as_str().unwrap());
        });
        event_loop.run(get_hash).unwrap();
        let seconds = Duration::from_secs(0.01 as u64);
        thread::sleep(seconds);

        let block_hash = CryptoHash::try_from(hash_string).unwrap();
        println!("send transaction hash: {:?},i:{:?}", block_hash, i);

        let txs = generate_transactions("node0", "node0", i, block_hash, i);
        let bytes = txs[0].try_to_vec().unwrap();
        let tx_bytes = to_base64(&bytes);
        let tx_value = serde_json::to_value(tx_bytes).unwrap();
        let tx = near.web3().send_transaction(Some(tx_value)).map(|value| {
            println!("send_transaction: {:?}", value);
        });
        event_loop.run(tx);
        i += 1;
    }
}