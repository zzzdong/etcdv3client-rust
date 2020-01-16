mod client;
mod error;
pub mod pb;
mod watcher;

mod kv;
mod auth;

pub use client::EtcdClient;
pub use error::{EtcdClientError, WatchError};
pub use watcher::Watcher;
