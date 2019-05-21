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

    pub fn get_all(&self) -> Result<Vec<KeyValue>, EtcdClientError> {
        let mut req = RangeRequest::new();

        req.set_key(vec![0x00]);
        req.set_range_end(vec![0x00]);

        let resp = self
            .inner
            .range(grpc::RequestOptions::new(), req)
            .wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;

        let kvs = resp
            .get_kvs()
            .iter()
            .map(|kv| kv.clone())
            .collect();

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
        if b.len() == 0 {
            return no_prefix_end;
        }

        let mut end = b.to_vec();

        for i in (0..end.len()).rev() {
            if end[i] < 0xff {
                end[i] = end[i] + 1;
                return end[0..i + 1].to_vec();
            }
        }

        return no_prefix_end;
    }
}
