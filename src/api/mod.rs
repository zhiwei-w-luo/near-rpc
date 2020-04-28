//! Near RPC implementation

mod rpc;
mod near_subscribe;

use crate::Transport;
pub use self::near_subscribe::{NearSubscribe, SubscriptionId, SubscriptionResult, SubscriptionStream};

/// Common API for all namespaces
pub trait Namespace<T: Transport>: Clone {
    /// Creates new API namespace
    fn new(transport: T) -> Self;

    /// Borrows a transports.
    fn transport(&self) -> &T;
}

/// `NearApi` wrapper for all namespaces
#[derive(Debug, Clone)]
pub struct NearApi<T: Transport> {
    transport: T,
}

impl<T: Transport> NearApi<T> {
    /// Create new `NearApi` with given transports
    pub fn new(transport: T) -> Self {
        NearApi { transport }
    }
    /// Borrows a transports.
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Access methods from custom namespace
    pub fn api<A: Namespace<T>>(&self) -> A {
        A::new(self.transport.clone())
    }
    /// Access methods from custom namespace
    pub fn rpc(&self) -> rpc::NearRpc<T> {
        self.api()
    }
}