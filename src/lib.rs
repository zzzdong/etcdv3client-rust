//! A etcdv3 client written in Rust-lang.
//!
//! ```rust,no_run
//! use etcdv3client::{Client, Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let endpoint = "http://localhost:2379";
//!     let auth: Option<(String, String)> = None;
//!     let mut client = Client::new(vec![endpoint], auth).await?;
//!
//!     let key = "/hello";
//!     // use convenience api under Client.
//!     match client.get(key).await {
//!         Ok(v) => {
//!             println!("got `{}` => {:?}", key, String::from_utf8_lossy(&v));
//!         }
//!         Err(err) => {
//!             if err.is_key_not_found() {
//!                 eprintln!("can not find `{}`", key);
//!             } else {
//!                 eprintln!("etcd get failed: `{:?}`", err);
//!             }
//!             
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
pub mod pb;
pub mod transport;
mod utils;

mod auth;
mod kv;
mod lease;
mod watch;

pub use client::{Client, SimpleClient};
pub use error::{ErrKind, Error};
pub use kv::KvClient;
pub use lease::{LeaseClient, LeaseKeepAliver};
pub use watch::{WatchClient, Watcher};
