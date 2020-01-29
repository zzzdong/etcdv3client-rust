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
    pub async fn finish(self) -> Result<pb::RangeResponse> {
        let DoRangeRequest { request, client } = self;
        client.range(request).await
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
pub struct DoPutRequest<'a> {
    pub request: pb::PutRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoPutRequest<'a> {
    pub async fn finish(self) -> Result<pb::PutResponse> {
        let DoPutRequest { request, client } = self;
        client.put(request).await
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
pub struct DoDeleteRangeRequest<'a> {
    pub request: pb::DeleteRangeRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoDeleteRangeRequest<'a> {
    pub async fn finish(self) -> Result<pb::DeleteRangeResponse> {
        let DoDeleteRangeRequest { request, client } = self;
        client.delete_range(request).await
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
pub struct DoTxnRequest<'a> {
    pub request: pb::TxnRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoTxnRequest<'a> {
    pub async fn finish(self) -> Result<pb::TxnResponse> {
        let DoTxnRequest { request, client } = self;
        client.txn(request).await
    }
}
pub struct DoCompactionRequest<'a> {
    pub request: pb::CompactionRequest,
    pub(crate) client: &'a mut KvClient,
}
impl<'a> DoCompactionRequest<'a> {
    pub async fn finish(self) -> Result<pb::CompactionResponse> {
        let DoCompactionRequest { request, client } = self;
        client.compact(request).await
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
