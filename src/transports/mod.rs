//! Supported Ethereum And Near Protocol JSON-RPC transports.

use crate::Error;

/// RPC Result.
pub type Result<T> = ::std::result::Result<T, Error>;


#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "http")]
pub use self::http::Http;
#[cfg(any(feature = "ipc", feature = "http", feature = "ws"))]
mod shared;

#[cfg(any(feature = "http"))]
extern crate tokio_core;
#[cfg(any(feature = "ipc"))]
extern crate tokio_io;
#[cfg(any(feature = "ipc", feature = "http", feature = "ws"))]
pub use self::shared::EventLoopHandle;
