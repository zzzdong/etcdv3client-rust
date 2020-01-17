use std::fmt;

use tonic::transport::channel::Channel;

use crate::error::{EtcdClientError, Result};
use crate::pb::{self, kv_client::KvClient as PbKvClient};
use crate::utils::build_prefix_end;
use crate::EtcdClient;

pub struct KvClient {
    inner: PbKvClient<Channel>,
}

impl KvClient {
    pub fn new(channel: Channel, interceptor: Option<tonic::Interceptor>) -> Self {
        let client = match interceptor {
            Some(i) => PbKvClient::with_interceptor(channel, i),
            None => PbKvClient::new(channel),
        };

        KvClient { inner: client }
    }

    pub fn with_client(client: &EtcdClient) -> Self {
        let channel = client.channel.clone();
        let interceptor = client.interceptor.clone();
        Self::new(channel, interceptor)
    }

    /// Do range request
    ///
    /// ```rust
    /// let resp = client.do_range("hello").with_prefix().finish().await.unwrap();
    /// ```
    pub fn do_range(&mut self, key: impl AsRef<[u8]>) -> DoRange {
        DoRange::new(key, self)
    }

    /// It can also do a raw range request:
    ///
    /// ```rust
    /// let mut request = pb::RangeRequest::new("hello");
    /// request.count_only = true;
    /// let resp = client.range(request).await.unwrap();
    /// ```
    pub async fn range(&mut self, request: pb::RangeRequest) -> Result<pb::RangeResponse> {
        Ok(self.inner.range(request).await?.into_inner())
    }

    /// Get value by key
    #[inline]
    pub async fn get(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        let resp = self.do_range(key).finish().await?;
        let kv = resp
            .kvs
            .first()
            .ok_or_else(|| EtcdClientError::KeyNotFound)?;
        Ok(kv.value.clone())
    }

    /// Get string by key
    #[inline]
    pub async fn get_string(&mut self, key: impl AsRef<[u8]>) -> Result<String> {
        let value = self.get(key).await?;

        String::from_utf8(value).map_err(EtcdClientError::FromUtf8)
    }

    /// Get key-value pairs with prefix
    #[inline]
    pub async fn get_with_prefix(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<pb::KeyValue>> {
        let resp = self.do_range(key).with_prefix().finish().await?;

        Ok(resp.kvs.iter().map(|kv| kv.clone()).collect())
    }

    /// Get all key-value pairs
    #[inline]
    pub async fn all(&mut self) -> Result<Vec<pb::KeyValue>> {
        let resp = self
            .do_range([0x00])
            .with_range_end([0x00])
            .finish()
            .await?;

        Ok(resp.kvs.iter().map(|kv| kv.clone()).collect())
    }

    /// Do put request
    /// ```rust
    /// client.do_put("hello", "world").finish().await.unwrap();
    /// ```
    pub fn do_put(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> DoPut {
        DoPut::new(key, value, self)
    }

    pub async fn put(&mut self, request: pb::PutRequest) -> Result<pb::PutResponse> {
        Ok(self.inner.put(request).await?.into_inner())
    }

    /// Put a key-value paire
    pub async fn put_kv(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        self.do_put(key, value).finish().await.map(|_| ())
    }

    /// Do delete range request
    /// ```rust
    /// client.do_delete_range("hello").finish().await.unwrap();
    /// ```
    pub fn do_delete_range(&mut self, key: impl AsRef<[u8]>) -> DoDeleteRange {
        DoDeleteRange::new(key, self)
    }

    pub async fn delete_range(
        &mut self,
        request: pb::DeleteRangeRequest,
    ) -> Result<pb::DeleteRangeResponse> {
        Ok(self.inner.delete_range(request).await?.into_inner())
    }

    /// Delete a key-value paire
    pub async fn delete(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        self.do_delete_range(key).finish().await.map(|_| ())
    }

    pub fn do_txn(&mut self) -> DoTxn {
        DoTxn::new(self)
    }

    /// Txn processes multiple requests in a single transaction.
    pub async fn txn(&mut self, request: pb::TxnRequest) -> Result<pb::TxnResponse> {
        Ok(self.inner.txn(request).await?.into_inner())
    }

    /// Compact compacts the event history in the etcd key-value store.
    pub async fn compact(&mut self, revision: i64, physical: bool) -> Result<()> {
        let req = pb::CompactionRequest::new(revision, physical);
        let _resp = self.inner.compact(req).await?.into_inner();
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

pub struct DoRange<'a> {
    pub request: pb::RangeRequest,
    client: &'a mut KvClient,
}

impl<'a> DoRange<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut KvClient) -> Self {
        DoRange {
            request: pb::RangeRequest::new(key),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::RangeResponse> {
        let DoRange { request, client } = self;
        client.range(request).await
    }

    /// The key range end to fetch.
    pub fn with_range_end(mut self, end: impl AsRef<[u8]>) -> Self {
        self.request.range_end = end.as_ref().to_vec();
        self
    }

    /// Get with key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.request.range_end = build_prefix_end(&self.request.key);
        self
    }

    /// The maximum number of keys returned for the request. When limit is set to 0, it is treated as no limit.
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.request.limit = limit;
        self
    }

    /// Return only the keys and not the values.
    pub fn with_keys_only(mut self) -> Self {
        self.request.keys_only = true;
        self
    }

    /// Return only the count of the keys in the range.
    pub fn with_count_only(mut self) -> Self {
        self.request.count_only = true;
        self
    }
}

impl<'a> fmt::Debug for DoRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunRange")
            .field("request", &self.request)
            .finish()
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

pub struct DoPut<'a> {
    pub request: pb::PutRequest,
    client: &'a mut KvClient,
}

impl<'a> DoPut<'a> {
    pub fn new(key: impl AsRef<[u8]>, value: impl AsRef<[u8]>, client: &'a mut KvClient) -> Self {
        DoPut {
            request: pb::PutRequest::new(key, value),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::PutResponse> {
        let DoPut { request, client } = self;
        client.put(request).await
    }

    /// The lease ID to associate with the key in the key-value store. A lease value of 0 indicates no lease.
    pub fn with_lease(mut self, lease: i64) -> Self {
        self.request.lease = lease;
        self
    }

    ///  When set, responds with the key-value pair data before the update from this Put request.
    pub fn with_prev_kv(mut self) -> Self {
        self.request.prev_kv = true;
        self
    }
}

impl<'a> fmt::Debug for DoPut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunPut")
            .field("request", &self.request)
            .finish()
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

pub struct DoDeleteRange<'a> {
    pub request: pb::DeleteRangeRequest,
    client: &'a mut KvClient,
}

impl<'a> DoDeleteRange<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut KvClient) -> Self {
        DoDeleteRange {
            request: pb::DeleteRangeRequest::new(key),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::DeleteRangeResponse> {
        let DoDeleteRange { request, client } = self;
        client.delete_range(request).await
    }

    /// The key range end to delete.
    pub fn with_range_end(mut self, end: impl AsRef<[u8]>) -> Self {
        self.request.range_end = end.as_ref().to_vec();
        self
    }

    /// Delete key-value pairs with key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.request.range_end = build_prefix_end(&self.request.key);
        self
    }

    ///  When set, responds with the key-value pair data before the update from this Put request.
    pub fn with_prev_kv(mut self) -> Self {
        self.request.prev_kv = true;
        self
    }
}

impl<'a> fmt::Debug for DoDeleteRange<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RunDeleteRange")
            .field("request", &self.request)
            .finish()
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
        self.compare = cmps.into_iter().map(|c| c.into()).collect();
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

pub struct DoTxn<'a> {
    pub request: pb::TxnRequest,
    client: &'a mut KvClient,
}

impl<'a> DoTxn<'a> {
    pub fn new(client: &'a mut KvClient) -> Self {
        DoTxn {
            request: pb::TxnRequest {
                ..Default::default()
            },
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::TxnResponse> {
        let DoTxn { request, client } = self;
        client.txn(request).await
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
