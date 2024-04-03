use std::fmt;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

use crate::error::{ErrKind, Error, Result};
use crate::pb;
use crate::transport::{GrpcService, Transport};
use crate::utils::build_prefix_end;
use crate::Client;

use tokio::sync::mpsc::{channel, Sender};
use tonic::codec::Streaming;
use tonic::IntoStreamingRequest;

const MPSC_CHANNEL_SIZE: usize = 1;

#[derive(Debug, Clone)]
pub struct InnerWatchClient<S> {
    transport: S,
}
impl<S> InnerWatchClient<S>
where
    S: GrpcService,
{
    pub fn new(transport: S) -> Self {
        Self { transport }
    }
    pub async fn watch(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = pb::WatchRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<pb::WatchResponse>>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Watch/Watch");
        self.transport
            .streaming(request.into_streaming_request(), path)
            .await
    }
}
#[derive(Debug, Clone)]
pub struct WatchClient<S> {
    inner: InnerWatchClient<S>,
}
impl<S> WatchClient<S>
where
    S: GrpcService,
{
    pub async fn watch(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = pb::WatchRequest>,
    ) -> Result<tonic::codec::Streaming<pb::WatchResponse>> {
        self.inner
            .watch(request.into_streaming_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
}

impl<S> WatchClient<S>
where
    S: GrpcService + Send,
{
    pub fn new(transport: S) -> Self {
        WatchClient {
            inner: InnerWatchClient::new(transport),
        }
    }

    /// do watch
    ///
    /// ```no_run
    /// # use etcdv3client::{Client, Error, WatchClient};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// # let client = Client::new(vec!["localhost:2379"], None).await?;
    /// let resp = WatchClient::with_client(&client).do_watch("hello").with_prefix().await.unwrap();
    /// # Ok(())
    /// # }
    pub fn do_watch(&mut self, key: impl Into<Vec<u8>>) -> DoCreateWatch<S> {
        DoCreateWatch::new(key, self)
    }

    /// watch a key
    pub async fn watch_key(&mut self, key: impl Into<Vec<u8>>) -> Result<Watcher> {
        self.do_watch(key).await
    }

    /// watch a key with prefix
    pub async fn watch_prefix(&mut self, key: impl Into<Vec<u8>>) -> Result<Watcher> {
        self.do_watch(key).with_prefix().await
    }
}

pub struct DoCreateWatch<'a, S> {
    pub request: pb::WatchCreateRequest,
    client: &'a mut WatchClient<S>,
}

impl<'a, S> DoCreateWatch<'a, S>
where
    S: GrpcService,
{
    pub fn new(key: impl Into<Vec<u8>>, client: &'a mut WatchClient<S>) -> Self {
        DoCreateWatch {
            request: pb::WatchCreateRequest::new(key),
            client,
        }
    }

    async fn send(self) -> Result<Watcher> {
        let DoCreateWatch { request, client } = self;

        let create_watch = pb::watch_request::RequestUnion::CreateRequest(request);
        let create_req = pb::WatchRequest {
            request_union: Some(create_watch),
        };

        let (req_tx, req_rx) = channel::<pb::WatchRequest>(MPSC_CHANNEL_SIZE);
        req_tx
            .send(create_req)
            .await
            .map_err(|err| Error::new(ErrKind::WatchRequestFailed, err))?;

        let rx = tokio_stream::wrappers::ReceiverStream::new(req_rx);
        let mut resp = client.watch(rx.into_streaming_request()).await?;

        let watch_id = match resp.message().await? {
            Some(msg) => msg.watch_id,
            None => return Err(Error::from_kind(ErrKind::WatchStartFailed)),
        };

        let watcher = Watcher::new(watch_id, req_tx, resp);

        Ok(watcher)
    }

    /// The key range end to fetch.
    pub fn with_range_end(mut self, end: impl Into<Vec<u8>>) -> Self {
        self.request.range_end = end.into();
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

impl<'a, S> fmt::Debug for DoCreateWatch<'a, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoCreateWatch")
            .field("request", &self.request)
            .finish()
    }
}

impl<'a, S> IntoFuture for DoCreateWatch<'a, S>
where
    S: GrpcService
{
    type Output = Result<Watcher>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<Watcher>> + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

impl pb::WatchCreateRequest {
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        pb::WatchCreateRequest {
            key: key.into(),
            ..Default::default()
        }
    }
}

impl pb::WatchRequest {
    pub fn create_watch(key: impl Into<Vec<u8>>) -> Self {
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
            .map_err(|err| Error::new(ErrKind::WatchRequestFailed, err))?;

        Ok(())
    }

    pub async fn cancel(&mut self) -> Result<()> {
        let request = pb::WatchRequest::cancel_watch(self.watch_id);

        self.req_tx
            .send(request)
            .await
            .map_err(|err| Error::new(ErrKind::WatchRequestFailed, err))?;

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
