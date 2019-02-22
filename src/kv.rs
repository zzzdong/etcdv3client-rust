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

pub struct SimpleKVClient {
    inner: KVClient,
}

impl SimpleKVClient {
    pub fn new(client: Arc<grpc::Client>) -> SimpleKVClient {
        SimpleKVClient {
            inner: KVClient::with_client(client),
        }
    }

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
            .ok_or_else(|| EtcdClientError::KeyNotFound(key.to_string()))
    }

    #[inline]
    pub fn get_string(&self, key: &str) -> Result<String, EtcdClientError> {
        self.get_bytes(key)
            .and_then(|e| String::from_utf8(e).map_err(EtcdClientError::FromUtf8))
    }

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

    #[inline]
    pub fn put_string(&self, key: &str, value: &str) -> Result<(), EtcdClientError> {
        self.put_bytes(key, value.as_bytes().to_vec())
    }

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

    pub fn get_str(
        &self,
        key: &'static str,
    ) -> impl Future<Item = String, Error = EtcdClientError> {
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
                    .ok_or_else(|| EtcdClientError::KeyNotFound(key.to_owned().clone()))
            })
            .and_then(|b| String::from_utf8(b).map_err(EtcdClientError::FromUtf8))
    }
}
