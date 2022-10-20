use crate::client::Transport;
use crate::error::{ErrKind, Error, Result};
use crate::pb::{self, kv_client::KvClient as PbKvClient};
use crate::utils::build_prefix_end;
use crate::Client;

use helper::*;

pub struct KvClient {
    inner: PbKvClient<Transport>,
}

impl KvClient {
    pub(crate) fn new(transport: Transport) -> Self {
        let inner = PbKvClient::new(transport);

        KvClient { inner }
    }

    pub fn with_client(client: &Client) -> Self {
        Self::new(client.transport.clone())
    }

    /// Do range request
    ///
    /// ```no_run
    /// # use etcdv3client::{EtcdClient, Error, KvClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = EtcdClient::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).do_range("hello").with_prefix().await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub fn do_range(&mut self, key: impl AsRef<[u8]>) -> DoRangeRequest {
        DoRangeRequest::new(key, self)
    }

    /// Get value by key
    #[inline]
    pub async fn get(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        let resp = self.do_range(key).await?;
        let kv = resp
            .kvs
            .first()
            .ok_or_else(|| Error::from_kind(ErrKind::KeyNotFound))?;
        Ok(kv.value.clone())
    }

    /// Get string by key
    ///
    /// ```no_run
    /// # use etcdv3client::{EtcdClient, Error, KvClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = EtcdClient::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).get("hello").await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub async fn get_string(&mut self, key: impl AsRef<[u8]>) -> Result<String> {
        let value = self.get(key).await?;

        String::from_utf8(value).map_err(|err| Error::new(ErrKind::InvalidData, err))
    }

    /// Get key-value pairs with prefix
    #[inline]
    pub async fn get_with_prefix(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<pb::KeyValue>> {
        let resp = self.do_range(key).with_prefix().await?;

        Ok(resp.kvs.to_vec())
    }

    /// Get all key-value pairs
    #[inline]
    pub async fn all(&mut self) -> Result<Vec<pb::KeyValue>> {
        let resp = self.do_range([0x00]).with_range_end(vec![0x00]).await?;

        Ok(resp.kvs.to_vec())
    }

    /// Do put request
    ///
    /// ```no_run
    /// # use etcdv3client::{EtcdClient, Error, KvClient, pb};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = EtcdClient::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).do_put("hello", "world").with_prev_kv(true).await.unwrap();
    /// # Ok(())
    /// # }
    pub fn do_put(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> DoPutRequest {
        DoPutRequest::new(key, value, self)
    }

    /// Put a key-value paire
    pub async fn put_kv(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        self.do_put(key, value).await.map(|_| ())
    }

    /// Do delete range request
    ///
    /// ```no_run
    /// # use etcdv3client::{EtcdClient, Error, KvClient, pb};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = EtcdClient::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).do_delete_range("hello").with_prefix().await.unwrap();
    /// # Ok(())
    /// # }
    pub fn do_delete_range(&mut self, key: impl AsRef<[u8]>) -> DoDeleteRangeRequest {
        DoDeleteRangeRequest::new(key, self)
    }

    /// Delete a key-value paire
    pub async fn delete(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        self.do_delete_range(key).await.map(|_| ())
    }

    pub fn do_txn(&mut self) -> DoTxnRequest {
        DoTxnRequest::new(self)
    }

    pub fn do_compaction(&mut self, revision: i64, physical: bool) -> DoCompactionRequest {
        DoCompactionRequest::new(revision, physical, self)
    }

    /// Compact compacts the event history in the etcd key-value store.
    pub async fn compact_history(&mut self, revision: i64, physical: bool) -> Result<()> {
        let _resp = self.do_compaction(revision, physical).await?;
        Ok(())
    }
}

impl pb::RangeRequest {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        pb::RangeRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        }
    }
}

impl<'a> DoRangeRequest<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut KvClient) -> Self {
        DoRangeRequest {
            request: pb::RangeRequest::new(key),
            client,
        }
    }

    /// Get with key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.request.range_end = build_prefix_end(&self.request.key);
        self
    }
}

impl pb::PutRequest {
    pub fn new(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Self {
        pb::PutRequest {
            key: key.as_ref().to_vec(),
            value: value.as_ref().to_vec(),
            ..Default::default()
        }
    }
}

impl<'a> DoPutRequest<'a> {
    pub fn new(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>, client: &'a mut KvClient) -> Self {
        DoPutRequest {
            request: pb::PutRequest::new(key, value),
            client,
        }
    }
}

impl pb::DeleteRangeRequest {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        pb::DeleteRangeRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        }
    }
}

impl<'a> DoDeleteRangeRequest<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut KvClient) -> Self {
        DoDeleteRangeRequest {
            request: pb::DeleteRangeRequest::new(key),
            client,
        }
    }

    /// Delete key-value pairs with key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.request.range_end = build_prefix_end(&self.request.key);
        self
    }
}

impl pb::Compare {
    pub fn new(
        key: impl AsRef<[u8]>,
        result: pb::compare::CompareResult,
        target_union: pb::compare::TargetUnion,
    ) -> Self {
        let target = match target_union {
            pb::compare::TargetUnion::Version(_) => pb::compare::CompareTarget::Version,
            pb::compare::TargetUnion::CreateRevision(_) => pb::compare::CompareTarget::Create,
            pb::compare::TargetUnion::ModRevision(_) => pb::compare::CompareTarget::Mod,
            pb::compare::TargetUnion::Value(_) => pb::compare::CompareTarget::Value,
            pb::compare::TargetUnion::Lease(_) => pb::compare::CompareTarget::Lease,
        };

        pb::Compare {
            key: key.as_ref().to_vec(),
            result: result.into(),
            target: target.into(),
            target_union: Some(target_union),
            ..Default::default()
        }
    }

    /// Set key range end.
    pub fn with_range_end(mut self, end: impl AsRef<[u8]>) -> Self {
        self.range_end = end.as_ref().to_vec();
        self
    }

    /// Set key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.range_end = build_prefix_end(&self.key);
        self
    }
}

impl pb::TxnRequest {
    pub fn new() -> Self {
        pb::TxnRequest {
            ..Default::default()
        }
    }

    pub fn with_if(mut self, cmps: Vec<pb::Compare>) -> Self {
        self.compare = cmps;
        self
    }

    pub fn with_then(mut self, ops: Vec<pb::RequestOp>) -> Self {
        self.success = ops;
        self
    }

    pub fn with_else(mut self, ops: Vec<pb::RequestOp>) -> Self {
        self.failure = ops;
        self
    }
}

impl From<pb::RangeRequest> for pb::RequestOp {
    fn from(request: pb::RangeRequest) -> Self {
        let request_op = pb::request_op::Request::RequestRange(request);
        pb::RequestOp {
            request: Some(request_op),
        }
    }
}

impl From<pb::PutRequest> for pb::RequestOp {
    fn from(request: pb::PutRequest) -> Self {
        let request_op = pb::request_op::Request::RequestPut(request);
        pb::RequestOp {
            request: Some(request_op),
        }
    }
}

impl From<pb::DeleteRangeRequest> for pb::RequestOp {
    fn from(request: pb::DeleteRangeRequest) -> Self {
        let request_op = pb::request_op::Request::RequestDeleteRange(request);
        pb::RequestOp {
            request: Some(request_op),
        }
    }
}

impl From<pb::TxnRequest> for pb::RequestOp {
    fn from(request: pb::TxnRequest) -> Self {
        let request_op = pb::request_op::Request::RequestTxn(request);
        pb::RequestOp {
            request: Some(request_op),
        }
    }
}

impl<'a> DoTxnRequest<'a> {
    pub fn new(client: &'a mut KvClient) -> Self {
        DoTxnRequest {
            request: pb::TxnRequest {
                ..Default::default()
            },
            client,
        }
    }

    pub fn with_if(mut self, cmps: Vec<pb::Compare>) -> Self {
        self.request = self.request.with_if(cmps);
        self
    }

    pub fn with_then(mut self, ops: Vec<pb::RequestOp>) -> Self {
        self.request = self.request.with_then(ops);
        self
    }

    pub fn with_else(mut self, ops: Vec<pb::RequestOp>) -> Self {
        self.request = self.request.with_else(ops);
        self
    }
}

impl pb::CompactionRequest {
    pub fn new(revision: i64, physical: bool) -> Self {
        pb::CompactionRequest { revision, physical }
    }
}

impl<'a> DoCompactionRequest<'a> {
    pub fn new(revision: i64, physical: bool, client: &'a mut KvClient) -> Self {
        DoCompactionRequest {
            request: pb::CompactionRequest::new(revision, physical),
            client,
        }
    }
}

mod helper {
    #![allow(dead_code)]

    use crate::error::Result;
    use crate::kv::KvClient;
    use crate::pb;

    include!("pb/kv_helper.rs");
}
