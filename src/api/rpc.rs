//! `NearRpc` namespace

use crate::api::Namespace;
use crate::helpers::{self, CallFuture};

use crate::Transport;
use serde_json::Value;

use crate::types::{BlockId, ChunkId, BlockIdOrFinality, to_base64};

/// `NearRpc` namespace
#[derive(Debug, Clone)]
pub struct NearRpc<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for NearRpc<T> {
    fn new(transport: T) -> Self
        where
            Self: Sized,
    {
        NearRpc { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> NearRpc<T> {
    /// Returns Chunk information of Near protocol
    pub fn chunk(&self, params: Option<Value>) -> CallFuture<Value, T::Out> {
        let chunk = helpers::serialize(&params);
        CallFuture::new(self.transport.execute("chunk", vec![chunk]))
    }

    /// Get list of available validators.
    pub fn validators(&self, params: Option<Value>) -> CallFuture<Option<Value>, T::Out> {
        let args = helpers::serialize(&params);
        CallFuture::new(self.transport.execute("validators", vec![args]))
    }

    /// Get Network status of Near protocol.
    pub fn status(&self) -> CallFuture<Value, T::Out> {
        CallFuture::new(self.transport.execute("status", vec![]))
    }

    /// Get Information of Account of Near Protocol.
    pub fn query(&self, params: Option<Value>) -> CallFuture<Value, T::Out> {
        let args = helpers::serialize(&params);
        CallFuture::new(self.transport.execute("query", vec![]))
    }

    /// Get Block information of Near protocol.
    pub fn block(&self) -> CallFuture<Value, T::Out> {
        // let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));
        CallFuture::new(self.transport.execute("block", vec![]))
    }

    /// Send Transaction to Near Network with params
    pub fn send_transaction(&self, bytes: Option<Value>) -> CallFuture<Value, T::Out> {
        let args = helpers::serialize(&bytes);
        CallFuture::new(self.transport.execute("broadcast_tx_commit", vec![args]))
    }

    /// Get last block_hash to Near Network
    pub fn block_hash(&self) -> CallFuture<Value, T::Out> {
        CallFuture::new(self.transport.execute("block_hash", vec![]))
    }

    pub fn network_info(&self) -> CallFuture<Value, T::Out> {
        CallFuture::new(self.transport.execute("network_info", vec![]))
    }

    pub fn get_tx_status(&self, params: Option<Value>) -> CallFuture<Value, T::Out> {
        let args = helpers::serialize(&params);
        CallFuture::new(self.transport.execute("tx", vec![args]))
    }
}

#[cfg(test)]
mod tests {
    use futures::Future;

    use crate::api::Namespace;
    use crate::rpc::Value;
    use crate::types::{Bytes, H256};

    use super::NearRpc;

    rpc_test!(
        NearRpc:status => "status";
        Value::String("Test123".into()) => "Test123"
        );
}
