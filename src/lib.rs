mod client;
mod error;
pub mod pb;
mod utils;

mod auth;
mod kv;
mod watch;

pub use client::EtcdClient;
pub use error::{EtcdClientError, WatchError};
