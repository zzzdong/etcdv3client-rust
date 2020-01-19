use std::fmt;

use crate::error::{EtcdClientError, Result, WatchError};
use crate::pb::{self, watch_client::WatchClient as PbWatchClient};
use crate::utils::build_prefix_end;
use crate::EtcdClient;

use tokio::sync::mpsc::{channel, Sender};
use tonic::codec::Streaming;
use tonic::transport::channel::Channel;

const MPSC_CHANNEL_SIZE: usize = 1;

pub struct WatchClient {
    inner: PbWatchClient<Channel>,
}

impl WatchClient {
    pub fn new(channel: Channel, interceptor: Option<tonic::Interceptor>) -> Self {
        let client = match interceptor {
            Some(i) => PbWatchClient::with_interceptor(channel, i),
            None => PbWatchClient::new(channel),
        };

        WatchClient { inner: client }
    }

    pub fn with_client(client: &EtcdClient) -> Self {
        let channel = client.channel.clone();
        let interceptor = client.interceptor.clone();
        Self::new(channel, interceptor)
    }

    /// watch
    ///
    /// ```no_run
    /// # use etcdv3client::{EtcdClient, EtcdClientError, WatchClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), EtcdClientError> {
    /// # let client = EtcdClient::new(vec!["localhost:2379"], None).await?;
    /// let resp = WatchClient::with_client(&client).do_watch("hello").with_prefix().finish().await.unwrap();
    /// # Ok(())
    /// # }
    pub fn do_watch(&mut self, key: impl AsRef<[u8]>) -> DoCreateWatch {
        DoCreateWatch::new(key, self)
    }

    pub async fn watch(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = pb::WatchRequest>,
    ) -> Result<tonic::codec::Streaming<pb::WatchResponse>> {
        Ok(self.inner.watch(request).await?.into_inner())
    }

    /// watch a key
    pub async fn watch_key(&mut self, key: impl AsRef<[u8]>) -> Result<Watcher> {
        self.do_watch(key).finish().await
    }

    /// watch a key with prefix
    pub async fn watch_prefix(&mut self, key: impl AsRef<[u8]>) -> Result<Watcher> {
        self.do_watch(key).with_prefix().finish().await
    }
}

pub struct DoCreateWatch<'a> {
    pub request: pb::WatchCreateRequest,
    client: &'a mut WatchClient,
}

impl<'a> DoCreateWatch<'a> {
    pub fn new(key: impl AsRef<[u8]>, client: &'a mut WatchClient) -> Self {
        DoCreateWatch {
            request: pb::WatchCreateRequest::new(key),
            client,
        }
    }

    pub async fn finish(self) -> Result<Watcher> {
        let DoCreateWatch { request, client } = self;

        let create_watch = pb::watch_request::RequestUnion::CreateRequest(request);
        let create_req = pb::WatchRequest {
            request_union: Some(create_watch),
        };

        let (mut req_tx, req_rx) = channel::<pb::WatchRequest>(MPSC_CHANNEL_SIZE);
        req_tx.send(create_req).await.map_err(WatchError::from)?;

        let mut resp = client.watch(req_rx).await?;
        let watch_id;

        match resp.message().await? {
            Some(msg) => watch_id = msg.watch_id,
            None => return Err(EtcdClientError::from(WatchError::StartWatchError)),
        }

        let watcher = Watcher::new(watch_id, req_tx, resp);

        Ok(watcher)
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

    ///  When set, responds with the key-value pair data before the update from this Put request.
    pub fn with_prev_kv(mut self) -> Self {
        self.request.prev_kv = true;
        self
    }
}

impl<'a> fmt::Debug for DoCreateWatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoCreateWatch")
            .field("request", &self.request)
            .finish()
    }
}

impl pb::WatchCreateRequest {
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        pb::WatchCreateRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        }
    }
}

impl pb::WatchRequest {
    pub fn create_watch(key: impl AsRef<[u8]>) -> Self {
        let request = pb::WatchCreateRequest::new(key);
        let request_union = pb::watch_request::RequestUnion::CreateRequest(request);

        pb::WatchRequest {
            request_union: Some(request_union),
        }
    }

    pub fn progress_watch() -> Self {
        let request = pb::WatchProgressRequest {};
        let request_union = pb::watch_request::RequestUnion::ProgressRequest(request);

        pb::WatchRequest {
            request_union: Some(request_union),
        }
    }

    pub fn cancel_watch(watch_id: i64) -> Self {
        let request = pb::WatchCancelRequest { watch_id };
        let request_union = pb::watch_request::RequestUnion::CancelRequest(request);

        pb::WatchRequest {
            request_union: Some(request_union),
        }
    }
}

impl From<pb::WatchCreateRequest> for pb::WatchRequest {
    fn from(request: pb::WatchCreateRequest) -> Self {
        let request_union = pb::watch_request::RequestUnion::CreateRequest(request);

        pb::WatchRequest {
            request_union: Some(request_union),
        }
    }
}

impl From<pb::WatchProgressRequest> for pb::WatchRequest {
    fn from(request: pb::WatchProgressRequest) -> Self {
        let request_union = pb::watch_request::RequestUnion::ProgressRequest(request);

        pb::WatchRequest {
            request_union: Some(request_union),
        }
    }
}

impl From<pb::WatchCancelRequest> for pb::WatchRequest {
    fn from(request: pb::WatchCancelRequest) -> Self {
        let request_union = pb::watch_request::RequestUnion::CancelRequest(request);

        pb::WatchRequest {
            request_union: Some(request_union),
        }
    }
}

pub struct Watcher {
    watch_id: i64,
    req_tx: Sender<pb::WatchRequest>,
    inbound: Streaming<crate::pb::WatchResponse>,
}

impl Watcher {
    pub(crate) fn new(
        watch_id: i64,
        req_tx: Sender<pb::WatchRequest>,
        inbound: Streaming<crate::pb::WatchResponse>,
    ) -> Watcher {
        Watcher {
            watch_id,
            req_tx,
            inbound,
        }
    }

    pub async fn progress(&mut self) -> Result<()> {
        let request = pb::WatchRequest::progress_watch();

        self.req_tx
            .send(request)
            .await
            .map_err(WatchError::WatchRequestError)?;

        Ok(())
    }

    pub async fn cancel(&mut self) -> Result<()> {
        let request = pb::WatchRequest::cancel_watch(self.watch_id);

        self.req_tx
            .send(request)
            .await
            .map_err(WatchError::WatchRequestError)?;

        Ok(())
    }

    pub async fn message(&mut self) -> Result<Option<pb::WatchResponse>> {
        match self.inbound.message().await? {
            Some(resp) => Ok(Some(resp)),
            None => Ok(None),
        }
    }
}

impl fmt::Debug for Watcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Watcher")
            .field("watch_id", &self.watch_id)
            .finish()
    }
}
