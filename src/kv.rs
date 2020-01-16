use std::fmt;

use tonic::transport::channel::Channel;

use crate::error::{EtcdClientError, Result};
use crate::pb::{self, kv_client::KvClient as PbKvClient};
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
        DoRange::new(key, &mut self.inner)
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
        let resp = self.do_range([0x00]).with_range_end([0x00]).finish().await?;

        Ok(resp.kvs.iter().map(|kv| kv.clone()).collect())
    }

    pub fn do_put(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> DoPut {
        DoPut::new(key, value, &mut self.inner)
    }

    pub async fn put(&mut self, request: pb::PutRequest) -> Result<pb::PutResponse> {
        Ok(self.inner.put(request).await?.into_inner())
    }

    pub async fn put_kv(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>)-> Result<()> {
        let _ = self.do_put(key, value).finish().await?;
        Ok(())
    }

    pub fn delete_range(&mut self, key: impl AsRef<[u8]>) -> DoDeleteRange {
        DoDeleteRange::new(key, &mut self.inner)
    }

    pub async fn do_delete_range(
        &mut self,
        request: pb::DeleteRangeRequest,
    ) -> Result<pb::DeleteRangeResponse> {
        Ok(self.inner.delete_range(request).await?.into_inner())
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
    client: &'a mut PbKvClient<Channel>,
}

impl<'a> DoRange<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut PbKvClient<Channel>) -> Self {
        DoRange {
            request: pb::RangeRequest::new(key),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::RangeResponse> {
        let DoRange { request, client } = self;
        Ok(client.range(request).await?.into_inner())
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
    client: &'a mut PbKvClient<Channel>,
}

impl<'a> DoPut<'a> {
    pub fn new(
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
        client: &'a mut PbKvClient<Channel>,
    ) -> Self {
        DoPut {
            request: pb::PutRequest::new(key, value),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::PutResponse> {
        let DoPut { request, client } = self;
        Ok(client.put(request).await?.into_inner())
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
    client: &'a mut PbKvClient<Channel>,
}

impl<'a> DoDeleteRange<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut PbKvClient<Channel>) -> Self {
        DoDeleteRange {
            request: pb::DeleteRangeRequest::new(key),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::DeleteRangeResponse> {
        let DoDeleteRange { request, client } = self;
        Ok(client.delete_range(request).await?.into_inner())
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

pub fn build_prefix_end(prefix: impl AsRef<[u8]>) -> Vec<u8> {
    let no_prefix_end = Vec::new();

    if prefix.as_ref().is_empty() {
        return no_prefix_end;
    }

    let mut end = prefix.as_ref().to_vec();

    for i in (0..end.len()).rev() {
        if end[i] < 0xff {
            end[i] += 1;
            return end[0..=i].to_vec();
        }
    }

    no_prefix_end
}
