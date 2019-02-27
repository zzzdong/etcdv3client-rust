// kv client

use std::sync::Arc;

use futures::Future;

use grpc::ClientStub;

pub use crate::pb::rpc::{CompactionRequest, CompactionResponse};
pub use crate::pb::rpc::{DeleteRangeRequest, DeleteRangeResponse};
pub use crate::pb::rpc::{PutRequest, PutResponse};
pub use crate::pb::rpc::{RangeRequest, RangeResponse};
pub use crate::pb::rpc::{TxnRequest, TxnResponse};
pub use crate::pb::rpc_grpc::{KVClient, KV as Service};

use crate::client::EtcdClientError;

/// SimpleKV with String as key
pub struct SimpleKV {
    pub key: String,
    pub value: Vec<u8>,
}

/// KVClient which use String as key
pub struct SimpleKVClient {
    inner: KVClient,
}

impl SimpleKVClient {
    /// Reture a SimpleKVClient
    pub fn new(client: Arc<grpc::Client>) -> SimpleKVClient {
        SimpleKVClient {
            inner: KVClient::with_client(client),
        }
    }

    /// Get bytes by key
    pub fn get_bytes(&self, key: &str) -> Result<Vec<u8>, EtcdClientError> {
        let mut req = RangeRequest::new();
        req.set_key(key.as_bytes().to_vec());

        let resp = self
            .inner
            .range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        resp.get_kvs()
            .first()
            .map(|kv| kv.get_value().to_vec())
            .ok_or_else(|| EtcdClientError::KeyNotFound)
    }

    /// Get String by key
    #[inline]
    pub fn get_string(&self, key: &str) -> Result<String, EtcdClientError> {
        self.get_bytes(key)
            .and_then(|e| String::from_utf8(e).map_err(EtcdClientError::FromUtf8))
    }

    /// Put bytes with key
    pub fn put_bytes(&self, key: &str, value: Vec<u8>) -> Result<(), EtcdClientError> {
        let mut req = PutRequest::new();
        req.set_key(key.as_bytes().to_vec());
        req.set_value(value);

        let _resp = self
            .inner
            .put(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        Ok(())
    }

    /// Put String with key
    #[inline]
    pub fn put_string(&self, key: &str, value: &str) -> Result<(), EtcdClientError> {
        self.put_bytes(key, value.as_bytes().to_vec())
    }

    /// Delete a key
    pub fn delete(&self, key: &str) -> Result<(), EtcdClientError> {
        let mut req = DeleteRangeRequest::new();
        req.set_key(key.as_bytes().to_vec());
        let _resp = self
            .inner
            .delete_range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        Ok(())
    }

    /// Get bytes with prefix
    pub fn get_with_prefix(&self, prefix: &str) -> Result<Vec<SimpleKV>, EtcdClientError> {
        let mut req = RangeRequest::new();
        let mut end = prefix.as_bytes().to_vec();
        end.push(0xFF);

        req.set_key(prefix.as_bytes().to_vec());
        req.set_range_end(end);

        let resp = self
            .inner
            .range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        let kv = resp
            .get_kvs()
            .iter()
            .map(|kv| SimpleKV {
                key: String::from_utf8_lossy(kv.get_key()).to_string(),
                value: kv.get_value().to_vec(),
            })
            .collect();

        Ok(kv)
    }

    pub fn get_str(&self, key: &str) -> impl Future<Item = String, Error = EtcdClientError> {
        let mut req = RangeRequest::new();
        req.set_key(key.as_bytes().to_vec());

        self.inner
            .range(grpc::RequestOptions::new(), req)
            .drop_metadata()
            .map_err(EtcdClientError::GRPC)
            .and_then(move |resp| {
                resp.get_kvs()
                    .first()
                    .map(|kv| kv.get_value().to_vec())
                    .ok_or_else(|| EtcdClientError::KeyNotFound)
            })
            .and_then(|b| String::from_utf8(b).map_err(EtcdClientError::FromUtf8))
    }
}
