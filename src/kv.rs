use crate::error::{ErrKind, Error, Result};
use crate::pb::{self};
use crate::transport::GrpcService;
use crate::utils::build_prefix_end;
use tonic::IntoRequest;

#[derive(Debug, Clone)]
pub struct InnerKvClient<S> {
    transport: S,
}
impl<S> InnerKvClient<S>
where
    S: GrpcService,
{
    pub fn new(transport: S) -> Self {
        Self { transport }
    }
    pub async fn range(
        &mut self,
        request: impl tonic::IntoRequest<pb::RangeRequest>,
    ) -> Result<tonic::Response<pb::RangeResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Range");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn put(
        &mut self,
        request: impl tonic::IntoRequest<pb::PutRequest>,
    ) -> Result<tonic::Response<pb::PutResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Put");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn delete_range(
        &mut self,
        request: impl tonic::IntoRequest<pb::DeleteRangeRequest>,
    ) -> Result<tonic::Response<pb::DeleteRangeResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/DeleteRange");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn txn(
        &mut self,
        request: impl tonic::IntoRequest<pb::TxnRequest>,
    ) -> Result<tonic::Response<pb::TxnResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Txn");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn compact(
        &mut self,
        request: impl tonic::IntoRequest<pb::CompactionRequest>,
    ) -> Result<tonic::Response<pb::CompactionResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.KV/Compact");
        self.transport.unary(request.into_request(), path).await
    }
}
#[derive(Debug, Clone)]
pub struct KvClient<S> {
    inner: InnerKvClient<S>,
}
impl<S> KvClient<S>
where
    S: GrpcService,
{
    pub async fn range(&mut self, request: pb::RangeRequest) -> Result<pb::RangeResponse> {
        self.inner
            .range(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn put(&mut self, request: pb::PutRequest) -> Result<pb::PutResponse> {
        self.inner
            .put(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn delete_range(
        &mut self,
        request: pb::DeleteRangeRequest,
    ) -> Result<pb::DeleteRangeResponse> {
        self.inner
            .delete_range(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn txn(&mut self, request: pb::TxnRequest) -> Result<pb::TxnResponse> {
        self.inner
            .txn(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn compact(
        &mut self,
        request: pb::CompactionRequest,
    ) -> Result<pb::CompactionResponse> {
        self.inner
            .compact(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
}

impl<S> KvClient<S>
where
    S: GrpcService,
{
    pub(crate) fn new(transport: S) -> Self {
        KvClient {
            inner: InnerKvClient::new(transport),
        }
    }

    // pub fn with_client(client: &Client) -> Self {
    //     Self::new(client.transport.clone())
    // }

    /// Do range request
    ///
    /// ```no_run
    /// # use etcdv3client::{Client, Error, KvClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).do_range("hello").with_prefix().await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub fn do_range(&mut self, key: impl Into<Vec<u8>>) -> DoRangeRequest<S> {
        pb::RangeRequest::new(key).build(self)
    }

    /// Get value by key
    #[inline]
    pub async fn get(&mut self, key: impl Into<Vec<u8>>) -> Result<Vec<u8>> {
        let resp = self.do_range(key).with_limit(1).await?;
        let kv = resp
            .kvs
            .into_iter()
            .next()
            .ok_or_else(|| Error::from_kind(ErrKind::KeyNotFound))?;
        Ok(kv.value.clone())
    }

    /// Get string by key
    ///
    /// ```no_run
    /// # use etcdv3client::{Client, Error, KvClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).get("hello").await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub async fn get_string(&mut self, key: impl Into<Vec<u8>>) -> Result<String> {
        let value = self.get(key).await?;

        String::from_utf8(value).map_err(|err| Error::new(ErrKind::InvalidData, err))
    }

    /// Get key-value pairs with prefix
    #[inline]
    pub async fn get_with_prefix(&mut self, key: impl Into<Vec<u8>>) -> Result<Vec<pb::KeyValue>> {
        let resp = self.do_range(key).with_prefix().await?;

        Ok(resp.kvs)
    }

    /// Get all key-value pairs
    #[inline]
    pub async fn all(&mut self) -> Result<Vec<pb::KeyValue>> {
        let resp = self.do_range([0x00]).with_range_end(vec![0x00]).await?;

        Ok(resp.kvs)
    }

    /// Do put request
    ///
    /// ```no_run
    /// # use etcdv3client::{Client, Error, KvClient, pb};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).do_put("hello", "world").with_prev_kv(true).await.unwrap();
    /// # Ok(())
    /// # }
    pub fn do_put(
        &mut self,
        key: impl Into<Vec<u8>>,
        value: impl Into<Vec<u8>>,
    ) -> DoPutRequest<S> {
        pb::PutRequest::new(key, value).build(self)
    }

    /// Put a key-value paire
    pub async fn put_kv(
        &mut self,
        key: impl Into<Vec<u8>>,
        value: impl Into<Vec<u8>>,
    ) -> Result<()> {
        self.do_put(key, value).await.map(|_| ())
    }

    /// Do delete range request
    ///
    /// ```no_run
    /// # use etcdv3client::{Client, Error, KvClient, pb};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(vec!["localhost:2379"], None).await?;
    /// let resp = KvClient::with_client(&client).do_delete_range("hello").with_prefix().await.unwrap();
    /// # Ok(())
    /// # }
    pub fn do_delete_range(&mut self, key: impl Into<Vec<u8>>) -> DoDeleteRangeRequest<S> {
        pb::DeleteRangeRequest::new(key).build(self)
    }

    /// Delete a key-value paire
    pub async fn delete(&mut self, key: impl Into<Vec<u8>>) -> Result<()> {
        self.do_delete_range(key).await.map(|_| ())
    }

    pub fn do_txn(&mut self) -> DoTxnRequest<S> {
        pb::TxnRequest::default().build(self)
    }

    pub fn do_compaction(&mut self, revision: i64, physical: bool) -> DoCompactionRequest<S> {
        pb::CompactionRequest::new(revision, physical).build(self)
    }

    /// Compact compacts the event history in the etcd key-value store.
    pub async fn compact_history(&mut self, revision: i64, physical: bool) -> Result<()> {
        let _resp = self.do_compaction(revision, physical).await?;
        Ok(())
    }
}

impl pb::RangeRequest {
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        pb::RangeRequest {
            key: key.into(),
            ..Default::default()
        }
    }

    /// Set key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.range_end = build_prefix_end(&self.key);
        self
    }

    pub fn build<'a, S: GrpcService>(self, client: &'a mut KvClient<S>) -> DoRangeRequest<'a, S> {
        DoRangeRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoRangeRequest<'a, S> {
    pub request: pb::RangeRequest,
    pub(crate) client: &'a mut KvClient<S>,
}
impl<'a, S> DoRangeRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut KvClient<S>) -> Self {
        self.client = client;
        self
    }

    /// Set key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.request = self.request.with_prefix();
        self
    }

    pub fn with_key(mut self, key: Vec<u8>) -> Self {
        self.request.key = key;
        self
    }
    pub fn with_range_end(mut self, range_end: Vec<u8>) -> Self {
        self.request.range_end = range_end;
        self
    }
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.request.limit = limit;
        self
    }
    pub fn with_revision(mut self, revision: i64) -> Self {
        self.request.revision = revision;
        self
    }
    pub fn with_sort_order(mut self, sort_order: i32) -> Self {
        self.request.sort_order = sort_order;
        self
    }
    pub fn with_sort_target(mut self, sort_target: i32) -> Self {
        self.request.sort_target = sort_target;
        self
    }
    pub fn with_serializable(mut self, serializable: bool) -> Self {
        self.request.serializable = serializable;
        self
    }
    pub fn with_keys_only(mut self, keys_only: bool) -> Self {
        self.request.keys_only = keys_only;
        self
    }
    pub fn with_count_only(mut self, count_only: bool) -> Self {
        self.request.count_only = count_only;
        self
    }
    pub fn with_min_mod_revision(mut self, min_mod_revision: i64) -> Self {
        self.request.min_mod_revision = min_mod_revision;
        self
    }
    pub fn with_max_mod_revision(mut self, max_mod_revision: i64) -> Self {
        self.request.max_mod_revision = max_mod_revision;
        self
    }
    pub fn with_min_create_revision(mut self, min_create_revision: i64) -> Self {
        self.request.min_create_revision = min_create_revision;
        self
    }
    pub fn with_max_create_revision(mut self, max_create_revision: i64) -> Self {
        self.request.max_create_revision = max_create_revision;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoRangeRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::RangeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::RangeResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoRangeRequest { request, client } = self;
        Box::pin(async move { client.range(request).await })
    }
}
impl pb::PutRequest {
    pub fn new(key: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Self {
        pb::PutRequest {
            key: key.into(),
            value: value.into(),
            ..Default::default()
        }
    }

    pub fn build<'a, S: GrpcService>(self, client: &'a mut KvClient<S>) -> DoPutRequest<'a, S> {
        DoPutRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoPutRequest<'a, S> {
    pub request: pb::PutRequest,
    pub(crate) client: &'a mut KvClient<S>,
}
impl<'a, S> DoPutRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut KvClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_key(mut self, key: Vec<u8>) -> Self {
        self.request.key = key;
        self
    }
    pub fn with_value(mut self, value: Vec<u8>) -> Self {
        self.request.value = value;
        self
    }
    pub fn with_lease(mut self, lease: i64) -> Self {
        self.request.lease = lease;
        self
    }
    pub fn with_prev_kv(mut self, prev_kv: bool) -> Self {
        self.request.prev_kv = prev_kv;
        self
    }
    pub fn with_ignore_value(mut self, ignore_value: bool) -> Self {
        self.request.ignore_value = ignore_value;
        self
    }
    pub fn with_ignore_lease(mut self, ignore_lease: bool) -> Self {
        self.request.ignore_lease = ignore_lease;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoPutRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::PutResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::PutResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoPutRequest { request, client } = self;
        Box::pin(async move { client.put(request).await })
    }
}

impl pb::DeleteRangeRequest {
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        pb::DeleteRangeRequest {
            key: key.into(),
            ..Default::default()
        }
    }

    pub fn build<'a, S: GrpcService>(
        self,
        client: &'a mut KvClient<S>,
    ) -> DoDeleteRangeRequest<'a, S> {
        DoDeleteRangeRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoDeleteRangeRequest<'a, S> {
    pub request: pb::DeleteRangeRequest,
    pub(crate) client: &'a mut KvClient<S>,
}
impl<'a, S> DoDeleteRangeRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut KvClient<S>) -> Self {
        self.client = client;
        self
    }

    pub fn with_key(mut self, key: Vec<u8>) -> Self {
        self.request.key = key;
        self
    }

    /// Delete key-value pairs with key prefix.
    pub fn with_prefix(mut self) -> Self {
        self.request.range_end = build_prefix_end(&self.request.key);
        self
    }

    pub fn with_range_end(mut self, range_end: Vec<u8>) -> Self {
        self.request.range_end = range_end;
        self
    }
    pub fn with_prev_kv(mut self, prev_kv: bool) -> Self {
        self.request.prev_kv = prev_kv;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoDeleteRangeRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::DeleteRangeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::DeleteRangeResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoDeleteRangeRequest { request, client } = self;
        Box::pin(async move { client.delete_range(request).await })
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

impl pb::Compare {
    pub fn new(
        key: impl Into<Vec<u8>>,
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
            key: key.into(),
            result: result.into(),
            target: target.into(),
            target_union: Some(target_union),
            ..Default::default()
        }
    }

    /// Set key range end.
    pub fn with_range_end(mut self, end: impl Into<Vec<u8>>) -> Self {
        self.range_end = end.into();
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

    pub fn build<'a, S: GrpcService>(self, client: &'a mut KvClient<S>) -> DoTxnRequest<'a, S> {
        DoTxnRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoTxnRequest<'a, S> {
    pub request: pb::TxnRequest,
    pub(crate) client: &'a mut KvClient<S>,
}
impl<'a, S> DoTxnRequest<'a, S>
where
    S: GrpcService,
{
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

    pub fn with_client(mut self, client: &'a mut KvClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoTxnRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::TxnResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::TxnResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoTxnRequest { request, client } = self;
        Box::pin(async move { client.txn(request).await })
    }
}
impl pb::CompactionRequest {
    pub fn new(revision: i64, physical: bool) -> Self {
        pb::CompactionRequest { revision, physical }
    }

    pub fn build<'a, S: GrpcService>(
        self,
        client: &'a mut KvClient<S>,
    ) -> DoCompactionRequest<'a, S> {
        DoCompactionRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoCompactionRequest<'a, S> {
    pub request: pb::CompactionRequest,
    pub(crate) client: &'a mut KvClient<S>,
}
impl<'a, S> DoCompactionRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut KvClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_revision(mut self, revision: i64) -> Self {
        self.request.revision = revision;
        self
    }
    pub fn with_physical(mut self, physical: bool) -> Self {
        self.request.physical = physical;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoCompactionRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::CompactionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::CompactionResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoCompactionRequest { request, client } = self;
        Box::pin(async move { client.compact(request).await })
    }
}
