// kv client

use std::sync::Arc;

use futures::Future;

use grpc::ClientStub;

pub use crate::pb::kv::KeyValue;
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

    /// Get bytes kv
    pub fn get_bytes_kv(&self, key: Vec<u8>) -> Result<Vec<u8>, EtcdClientError> {
        let mut req = RangeRequest::new();
        req.set_key(key.to_vec());

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

    /// Get bytes by key
    #[inline]
    pub fn get_bytes(&self, key: &str) -> Result<Vec<u8>, EtcdClientError> {
        self.get_bytes_kv(key.as_bytes().to_vec())
    }

    /// Get String by key
    #[inline]
    pub fn get_string(&self, key: &str) -> Result<String, EtcdClientError> {
        self.get_bytes(key)
            .and_then(|e| String::from_utf8(e).map_err(EtcdClientError::FromUtf8))
    }

    /// Put bytes Kv
    pub fn put_bytes_kv(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), EtcdClientError> {
        let mut req = PutRequest::new();
        req.set_key(key);
        req.set_value(value);

        let _resp = self
            .inner
            .put(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        Ok(())
    }

    /// Put bytes with key
    #[inline]
    pub fn put_bytes(&self, key: &str, value: Vec<u8>) -> Result<(), EtcdClientError> {
        self.put_bytes_kv(key.as_bytes().to_vec(), value)
    }

    /// Put String with key
    #[inline]
    pub fn put_string(&self, key: &str, value: &str) -> Result<(), EtcdClientError> {
        self.put_bytes(key, value.as_bytes().to_vec())
    }

    /// Delete bytes key
    pub fn delete_bytes_key(&self, key: Vec<u8>) -> Result<(), EtcdClientError> {
        let mut req = DeleteRangeRequest::new();
        req.set_key(key);

        let _resp = self
            .inner
            .delete_range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        Ok(())
    }

    /// Delete a key
    #[inline]
    pub fn delete(&self, key: &str) -> Result<(), EtcdClientError> {
        self.delete_bytes_key(key.as_bytes().to_vec())
    }

    /// Get bytes with prefix
    pub fn get_with_prefix(&self, prefix: &str) -> Result<Vec<SimpleKV>, EtcdClientError> {
        let mut req = RangeRequest::new();
        let end = Self::build_prefix_end(prefix);

        req.set_key(prefix.as_bytes().to_vec());
        req.set_range_end(end);

        let resp = self
            .inner
            .range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        let kvs = resp
            .get_kvs()
            .iter()
            .map(|kv| SimpleKV {
                key: String::from_utf8_lossy(kv.get_key()).to_string(),
                value: kv.get_value().to_vec(),
            })
            .collect();

        Ok(kvs)
    }

    /// Get all keyvalue
    pub fn get_all(&self) -> Result<Vec<KeyValue>, EtcdClientError> {
        let mut req = RangeRequest::new();

        req.set_key(vec![0x00]);
        req.set_range_end(vec![0x00]);

        let resp = self
            .inner
            .range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        let kvs = resp.get_kvs().to_vec();

        Ok(kvs)
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

    fn build_prefix_end(prefix: &str) -> Vec<u8> {
        let no_prefix_end = Vec::new();

        let b = prefix.as_bytes();
        if b.is_empty() {
            return no_prefix_end;
        }

        let mut end = b.to_vec();

        for i in (0..end.len()).rev() {
            if end[i] < 0xff {
                end[i] += 1;
                return end[0..=1].to_vec();
            }
        }

        no_prefix_end
    }
}
