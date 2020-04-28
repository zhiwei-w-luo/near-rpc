use crate::types::CryptoHash;
use borsh::{BorshDeserialize, BorshSerialize};
use serde_derive::{Deserialize, Serialize};

/// Account identifier. Provides access to user's state.
pub type AccountId = String;
/// Balance is type for storing amounts of tokens.
pub type Balance = u128;
/// Hash used by a struct implementing the Merkle tree.
pub type MerkleHash = CryptoHash;
/// Nonce for transactions.
pub type Nonce = u64;
/// Index of the block.
pub type BlockHeight = u64;
/// StorageUsage is used to count the amount of storage used by a contract.
pub type StorageUsage = u64;
/// Gas is a type for storing amount of gas.
pub type Gas = u64;
/// Shard index, from 0 to NUM_SHARDS - 1.
pub type ShardId = u64;

/// Different types of finality.
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Finality {
    #[serde(rename = "optimistic")]
    None,
    #[serde(rename = "near-final")]
    DoomSlug,
    #[serde(rename = "final")]
    NFG,
}

impl Default for Finality {
    fn default() -> Self {
        Finality::NFG
    }
}


#[derive(Debug, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockId {
    Height(BlockHeight),
    Hash(CryptoHash),
}

#[derive(Debug, Clone, Eq, PartialEq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChunkId {
    BlockShardId(BlockId, ShardId),
    Hash(CryptoHash),
}

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockIdOrFinality {
    BlockId(BlockId),
    Finality(Finality),
}

impl BlockIdOrFinality {
    pub fn latest() -> Self {
        Self::Finality(Finality::None)
    }
}

#[derive(BorshSerialize, Serialize, Deserialize, BorshDeserialize, Clone, Debug)]
pub struct RpcAutoSendConfig {
    pub url: String,
    pub key_path: String,
    pub account: AccountId,
    pub start_nonce: u64,
    pub max_count: u64,
}

impl Default for RpcAutoSendConfig {
    fn default() -> Self {
        Self {
            url: "0.0.0.0:3030".to_owned(),
            key_path: "".to_owned(),
            account: "node0".to_owned(),
            start_nonce: 0,
            max_count: 1000,
        }
    }
}