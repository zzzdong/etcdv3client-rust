mod client;
mod error;
pub mod pb;
mod watcher;

pub use client::EtcdClient;
pub use error::{EtcdClientError, WatchError};
pub use watcher::Watcher;
