//! A etcdv3 client written in Rust-lang.
//!
//! ```rust,no_run
//! use etcdv3client::{EtcdClient, EtcdClientError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), EtcdClientError> {
//!     let endpoint = "http://localhost:2379";
//!     let auth: Option<(String, String)> = None;
//!     let mut client = EtcdClient::new(vec![endpoint], auth).await?;
//!
//!     let key = "/hello";
//!     // use convenience api under EtcdClient.
//!     match client.get(key).await {
//!         Ok(v) => {
//!             println!("got `{}` => {:?}", key, String::from_utf8_lossy(&v));
//!         }
//!         Err(EtcdClientError::KeyNotFound) => {
//!             eprintln!("can not find `{}`", key);
//!         }
//!         Err(e) => {
//!             eprintln!("etcd get error: `{:?}`", e);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
pub mod pb;
mod service;
mod utils;

mod auth;
mod kv;
mod lease;
mod watch;

pub use client::EtcdClient;
pub use error::EtcdClientError;
pub use kv::KvClient;
pub use lease::{LeaseClient, LeaseKeepAliver};
pub use watch::{WatchClient, Watcher};
