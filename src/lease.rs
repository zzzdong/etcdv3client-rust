use std::fmt;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

use tokio::sync::mpsc::{channel, Sender};
use tonic::codec::Streaming;
use tonic::IntoRequest;
use tonic::IntoStreamingRequest;

use crate::error::{ErrKind, Error, Result};
use crate::pb::{self};
use crate::transport::GrpcService;

const MPSC_CHANNEL_SIZE: usize = 1;

#[derive(Debug, Clone)]
pub struct InnerLeaseClient<S> {
    transport: S,
}
impl<S> InnerLeaseClient<S>
where
    S: GrpcService,
{
    pub fn new(transport: S) -> Self {
        Self { transport }
    }
    pub async fn lease_grant(
        &mut self,
        request: impl tonic::IntoRequest<pb::LeaseGrantRequest>,
    ) -> Result<tonic::Response<pb::LeaseGrantResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseGrant");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn lease_revoke(
        &mut self,
        request: impl tonic::IntoRequest<pb::LeaseRevokeRequest>,
    ) -> Result<tonic::Response<pb::LeaseRevokeResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseRevoke");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn lease_keep_alive(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = pb::LeaseKeepAliveRequest>,
    ) -> Result<tonic::Response<tonic::codec::Streaming<pb::LeaseKeepAliveResponse>>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseKeepAlive");
        self.transport
            .streaming(request.into_streaming_request(), path)
            .await
    }
    pub async fn lease_time_to_live(
        &mut self,
        request: impl tonic::IntoRequest<pb::LeaseTimeToLiveRequest>,
    ) -> Result<tonic::Response<pb::LeaseTimeToLiveResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseTimeToLive");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn lease_leases(
        &mut self,
        request: impl tonic::IntoRequest<pb::LeaseLeasesRequest>,
    ) -> Result<tonic::Response<pb::LeaseLeasesResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Lease/LeaseLeases");
        self.transport.unary(request.into_request(), path).await
    }
}
#[derive(Debug, Clone)]
pub struct LeaseClient<S> {
    inner: InnerLeaseClient<S>,
}
impl<S> LeaseClient<S>
where
    S: GrpcService,
{
    pub async fn lease_grant(
        &mut self,
        request: pb::LeaseGrantRequest,
    ) -> Result<pb::LeaseGrantResponse> {
        self.inner
            .lease_grant(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn lease_revoke(
        &mut self,
        request: pb::LeaseRevokeRequest,
    ) -> Result<pb::LeaseRevokeResponse> {
        self.inner
            .lease_revoke(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn lease_keep_alive(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = pb::LeaseKeepAliveRequest>,
    ) -> Result<tonic::codec::Streaming<pb::LeaseKeepAliveResponse>> {
        self.inner
            .lease_keep_alive(request.into_streaming_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn lease_time_to_live(
        &mut self,
        request: pb::LeaseTimeToLiveRequest,
    ) -> Result<pb::LeaseTimeToLiveResponse> {
        self.inner
            .lease_time_to_live(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn lease_leases(
        &mut self,
        request: pb::LeaseLeasesRequest,
    ) -> Result<pb::LeaseLeasesResponse> {
        self.inner
            .lease_leases(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
}

impl<S> LeaseClient<S>
where
    S: GrpcService,
{
    pub(crate) fn new(transport: S) -> Self {
        LeaseClient {
            inner: InnerLeaseClient::new(transport),
        }
    }

    pub fn do_grant(&mut self, ttl: i64) -> DoLeaseGrantRequest<S> {
        pb::LeaseGrantRequest::new(ttl, 0).build(self)
    }

    /// Grant a lease with the given ttl.
    pub async fn grant(&mut self, ttl: i64) -> Result<pb::LeaseGrantResponse> {
        self.do_grant(ttl).await
    }

    /// Grant a lease with the given ttl and lease id.
    pub async fn grant_with_lease_id(
        &mut self,
        ttl: i64,
        lease_id: i64,
    ) -> Result<pb::LeaseGrantResponse> {
        self.do_grant(ttl).with_id(lease_id).await
    }

    /// Revoke the given lease.
    pub async fn revoke(&mut self, lease_id: i64) -> Result<()> {
        let request = pb::LeaseRevokeRequest::new(lease_id);
        self.lease_revoke(request).await.map(|_| ())
    }

    /// Keep the lease alive.
    #[must_use]
    pub fn do_keep_alive(&mut self, lease_id: i64) -> DoLeaseKeepAlive<S> {
        DoLeaseKeepAlive::new(lease_id, self)
    }

    /// Keep the lease alive.
    /// When call, it sent the first keep alive message and return LeaseKeepAliver for further call.
    pub async fn keep_alive(&mut self, lease_id: i64) -> Result<LeaseKeepAliver> {
        self.do_keep_alive(lease_id).await
    }

    pub async fn get_lease_info(
        &mut self,
        lease_id: i64,
        keys: bool,
    ) -> Result<pb::LeaseTimeToLiveResponse> {
        self.lease_time_to_live(pb::LeaseTimeToLiveRequest::new(lease_id, keys))
            .await
    }

    pub async fn list(&mut self) -> Result<pb::LeaseLeasesResponse> {
        self.lease_leases(pb::LeaseLeasesRequest::new()).await
    }
}

impl pb::LeaseGrantRequest {
    fn new(ttl: i64, id: i64) -> Self {
        pb::LeaseGrantRequest { ttl, id }
    }

    pub fn build<'a, S: GrpcService>(
        self,
        client: &'a mut LeaseClient<S>,
    ) -> DoLeaseGrantRequest<'a, S> {
        DoLeaseGrantRequest {
            request: self,
            client,
        }
    }
}

#[must_use]
pub struct DoLeaseGrantRequest<'a, S> {
    pub request: pb::LeaseGrantRequest,
    pub(crate) client: &'a mut LeaseClient<S>,
}
impl<'a, S> DoLeaseGrantRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut LeaseClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_ttl(mut self, ttl: i64) -> Self {
        self.request.ttl = ttl;
        self
    }
    pub fn with_id(mut self, id: i64) -> Self {
        self.request.id = id;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoLeaseGrantRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::LeaseGrantResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseGrantResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseGrantRequest { request, client } = self;
        Box::pin(async move { client.lease_grant(request).await })
    }
}

impl pb::LeaseRevokeRequest {
    fn new(id: i64) -> Self {
        pb::LeaseRevokeRequest { id }
    }

    pub fn build<'a, S: GrpcService>(
        self,
        client: &'a mut LeaseClient<S>,
    ) -> DoLeaseRevokeRequest<'a, S> {
        DoLeaseRevokeRequest {
            request: self,
            client,
        }
    }
}

#[must_use]
pub struct DoLeaseRevokeRequest<'a, S> {
    pub request: pb::LeaseRevokeRequest,
    pub(crate) client: &'a mut LeaseClient<S>,
}
impl<'a, S> DoLeaseRevokeRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut LeaseClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_id(mut self, id: i64) -> Self {
        self.request.id = id;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoLeaseRevokeRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::LeaseRevokeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseRevokeResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseRevokeRequest { request, client } = self;
        Box::pin(async move { client.lease_revoke(request).await })
    }
}

impl pb::LeaseKeepAliveRequest {
    pub fn new(lease_id: i64) -> Self {
        pb::LeaseKeepAliveRequest { id: lease_id }
    }
}

impl pb::LeaseTimeToLiveRequest {
    pub fn build<'a, S: GrpcService>(
        self,
        client: &'a mut LeaseClient<S>,
    ) -> DoLeaseTimeToLiveRequest<'a, S> {
        DoLeaseTimeToLiveRequest {
            request: self,
            client,
        }
    }
}

#[must_use]
pub struct DoLeaseTimeToLiveRequest<'a, S> {
    pub request: pb::LeaseTimeToLiveRequest,
    pub(crate) client: &'a mut LeaseClient<S>,
}
impl<'a, S> DoLeaseTimeToLiveRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut LeaseClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_id(mut self, id: i64) -> Self {
        self.request.id = id;
        self
    }
    pub fn with_keys(mut self, keys: bool) -> Self {
        self.request.keys = keys;
        self
    }
}

impl<'a, S> std::future::IntoFuture for DoLeaseTimeToLiveRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::LeaseTimeToLiveResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<pb::LeaseTimeToLiveResponse>>
                + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseTimeToLiveRequest { request, client } = self;
        Box::pin(async move { client.lease_time_to_live(request).await })
    }
}

impl pb::LeaseLeasesRequest {
    pub fn build<'a, S: GrpcService>(
        self,
        client: &'a mut LeaseClient<S>,
    ) -> DoLeaseLeasesRequest<'a, S> {
        DoLeaseLeasesRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoLeaseLeasesRequest<'a, S> {
    pub request: pb::LeaseLeasesRequest,
    pub(crate) client: &'a mut LeaseClient<S>,
}
impl<'a, S> DoLeaseLeasesRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut LeaseClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoLeaseLeasesRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::LeaseLeasesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseLeasesResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseLeasesRequest { request, client } = self;
        Box::pin(async move { client.lease_leases(request).await })
    }
}

pub struct DoLeaseKeepAlive<'a, S> {
    pub lease_id: i64,
    client: &'a mut LeaseClient<S>,
}

impl<'a, S> DoLeaseKeepAlive<'a, S>
where
    S: GrpcService,
{
    pub fn new(lease_id: i64, client: &'a mut LeaseClient<S>) -> Self {
        DoLeaseKeepAlive { lease_id, client }
    }

    async fn send(self) -> Result<LeaseKeepAliver> {
        let DoLeaseKeepAlive { lease_id, client } = self;

        let (req_tx, req_rx) = channel::<pb::LeaseKeepAliveRequest>(MPSC_CHANNEL_SIZE);

        let request = pb::LeaseKeepAliveRequest::new(lease_id);
        req_tx
            .send(request)
            .await
            .map_err(|err| Error::new(ErrKind::LeaseRequestFailed, err))?;

        let rx = tokio_stream::wrappers::ReceiverStream::new(req_rx);
        let resp = client.lease_keep_alive(rx.into_streaming_request()).await?;

        Ok(LeaseKeepAliver::new(lease_id, req_tx, resp))
    }
}

impl<'a, S> fmt::Debug for DoLeaseKeepAlive<'a, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoLeaseKeepAlive")
            .field("lease_id", &self.lease_id)
            .finish()
    }
}

impl<'a, S> IntoFuture for DoLeaseKeepAlive<'a, S>
where
    S: GrpcService,
{
    type Output = Result<LeaseKeepAliver>;
    type IntoFuture = Pin<Box<dyn Future<Output = Result<LeaseKeepAliver>> + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

pub struct LeaseKeepAliver {
    lease_id: i64,
    req_tx: Sender<pb::LeaseKeepAliveRequest>,
    inbound: Streaming<crate::pb::LeaseKeepAliveResponse>,
}

impl LeaseKeepAliver {
    pub fn new(
        lease_id: i64,
        req_tx: Sender<pb::LeaseKeepAliveRequest>,
        inbound: Streaming<crate::pb::LeaseKeepAliveResponse>,
    ) -> Self {
        LeaseKeepAliver {
            lease_id,
            req_tx,
            inbound,
        }
    }

    pub async fn keep_alive(&mut self) -> Result<()> {
        let request = pb::LeaseKeepAliveRequest::new(self.lease_id);

        self.req_tx
            .send(request)
            .await
            .map_err(|err| Error::new(ErrKind::LeaseRequestFailed, err))?;

        Ok(())
    }

    pub async fn message(&mut self) -> Result<Option<pb::LeaseKeepAliveResponse>> {
        match self.inbound.message().await? {
            Some(resp) => Ok(Some(resp)),
            None => Ok(None),
        }
    }
}

impl fmt::Debug for LeaseKeepAliver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LeaseKeepAliver")
            .field("lease_id", &self.lease_id)
            .finish()
    }
}

impl pb::LeaseTimeToLiveRequest {
    pub fn new(lease_id: i64, keys: bool) -> Self {
        pb::LeaseTimeToLiveRequest { id: lease_id, keys }
    }
}

impl pb::LeaseLeasesRequest {
    fn new() -> Self {
        pb::LeaseLeasesRequest {}
    }
}
