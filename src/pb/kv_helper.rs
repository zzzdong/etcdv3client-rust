impl KvClient {
    pub async fn range(&mut self, request: pb::RangeRequest) -> Result<pb::RangeResponse> {
        Ok(self.inner.range(request).await?.into_inner())
    }
    pub async fn put(&mut self, request: pb::PutRequest) -> Result<pb::PutResponse> {
        Ok(self.inner.put(request).await?.into_inner())
    }
    pub async fn delete_range(
        &mut self,
        request: pb::DeleteRangeRequest,
    ) -> Result<pb::DeleteRangeResponse> {
        Ok(self.inner.delete_range(request).await?.into_inner())
    }
    pub async fn txn(&mut self, request: pb::TxnRequest) -> Result<pb::TxnResponse> {
        Ok(self.inner.txn(request).await?.into_inner())
    }
    pub async fn compact(
        &mut self,
        request: pb::CompactionRequest,
    ) -> Result<pb::CompactionResponse> {
        Ok(self.inner.compact(request).await?.into_inner())
    }
}
pub struct DoRangeRequest<'a> {
    pub request: pb::RangeRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoRangeRequest<'a> {
    pub fn from_client(client: &'a mut KvClient) -> Self {
        DoRangeRequest {
            request: Default::default(),
            client,
        }
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
impl<'a> std::future::IntoFuture for DoRangeRequest<'a> {
    type Output = Result<pb::RangeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::RangeResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoRangeRequest { request, client } = self;
        Box::pin(async move { client.range(request).await })
    }
}
pub struct DoPutRequest<'a> {
    pub request: pb::PutRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoPutRequest<'a> {
    pub fn from_client(client: &'a mut KvClient) -> Self {
        DoPutRequest {
            request: Default::default(),
            client,
        }
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
impl<'a> std::future::IntoFuture for DoPutRequest<'a> {
    type Output = Result<pb::PutResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::PutResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoPutRequest { request, client } = self;
        Box::pin(async move { client.put(request).await })
    }
}
pub struct DoDeleteRangeRequest<'a> {
    pub request: pb::DeleteRangeRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoDeleteRangeRequest<'a> {
    pub fn from_client(client: &'a mut KvClient) -> Self {
        DoDeleteRangeRequest {
            request: Default::default(),
            client,
        }
    }
    pub fn with_key(mut self, key: Vec<u8>) -> Self {
        self.request.key = key;
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
impl<'a> std::future::IntoFuture for DoDeleteRangeRequest<'a> {
    type Output = Result<pb::DeleteRangeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::DeleteRangeResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoDeleteRangeRequest { request, client } = self;
        Box::pin(async move { client.delete_range(request).await })
    }
}
pub struct DoTxnRequest<'a> {
    pub request: pb::TxnRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoTxnRequest<'a> {
    pub fn from_client(client: &'a mut KvClient) -> Self {
        DoTxnRequest {
            request: Default::default(),
            client,
        }
    }
}
impl<'a> std::future::IntoFuture for DoTxnRequest<'a> {
    type Output = Result<pb::TxnResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::TxnResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoTxnRequest { request, client } = self;
        Box::pin(async move { client.txn(request).await })
    }
}
pub struct DoCompactionRequest<'a> {
    pub request: pb::CompactionRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoCompactionRequest<'a> {
    pub fn from_client(client: &'a mut KvClient) -> Self {
        DoCompactionRequest {
            request: Default::default(),
            client,
        }
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
impl<'a> std::future::IntoFuture for DoCompactionRequest<'a> {
    type Output = Result<pb::CompactionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::CompactionResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoCompactionRequest { request, client } = self;
        Box::pin(async move { client.compact(request).await })
    }
}
