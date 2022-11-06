use std::fmt;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

use tokio::sync::mpsc::{channel, Sender};
use tonic::codec::Streaming;

use crate::client::Transport;
use crate::error::{ErrKind, Error, Result};
use crate::pb::{self, lease_client::LeaseClient as PbLeaseClient};
use crate::Client;

use helper::*;

const MPSC_CHANNEL_SIZE: usize = 1;

#[derive(Debug, Clone)]
pub struct LeaseClient {
    inner: PbLeaseClient<Transport>,
}

impl LeaseClient {
    pub(crate) fn new(transport: Transport) -> Self {
        let inner = PbLeaseClient::new(transport);

        LeaseClient { inner }
    }

    pub fn with_client(client: &Client) -> Self {
        Self::new(client.transport.clone())
    }

    pub fn do_grant(&mut self, ttl: i64) -> DoLeaseGrantRequest {
        DoLeaseGrantRequest::new(ttl, self)
    }

    pub async fn grant(&mut self, ttl: i64, lease_id: i64) -> Result<pb::LeaseGrantResponse> {
        self.do_grant(ttl).with_lease_id(lease_id).await
    }

    pub async fn revoke(&mut self, lease_id: i64) -> Result<()> {
        let request = pb::LeaseRevokeRequest::new(lease_id);
        self.lease_revoke(request).await.map(|_| ())
    }

    /// LeaseKeepAlive keeps the lease alive by streaming keep alive requests.
    pub async fn lease_keep_alive(
        &mut self,
        request: impl tonic::IntoStreamingRequest<Message = pb::LeaseKeepAliveRequest>,
    ) -> Result<tonic::codec::Streaming<pb::LeaseKeepAliveResponse>> {
        Ok(self.inner.lease_keep_alive(request).await?.into_inner())
    }

    pub fn do_keep_alive(&mut self, lease_id: i64) -> DoLeaseKeepAlive {
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
}

impl<'a> DoLeaseGrantRequest<'a> {
    pub fn new(ttl: i64, client: &'a mut LeaseClient) -> Self {
        DoLeaseGrantRequest {
            request: pb::LeaseGrantRequest::new(ttl, 0),
            client,
        }
    }

    pub fn with_lease_id(mut self, lease_id: i64) -> Self {
        self.request.id = lease_id;
        self
    }
}

impl pb::LeaseRevokeRequest {
    fn new(id: i64) -> Self {
        pb::LeaseRevokeRequest { id }
    }
}

impl pb::LeaseKeepAliveRequest {
    pub fn new(lease_id: i64) -> Self {
        pb::LeaseKeepAliveRequest { id: lease_id }
    }
}

pub struct DoLeaseKeepAlive<'a> {
    pub lease_id: i64,
    client: &'a mut LeaseClient,
}

impl<'a> DoLeaseKeepAlive<'a> {
    pub fn new(lease_id: i64, client: &'a mut LeaseClient) -> Self {
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
        let resp = client.lease_keep_alive(rx).await?;

        Ok(LeaseKeepAliver::new(lease_id, req_tx, resp))
    }
}

impl<'a> fmt::Debug for DoLeaseKeepAlive<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoLeaseKeepAlive")
            .field("lease_id", &self.lease_id)
            .finish()
    }
}

impl<'a> IntoFuture for DoLeaseKeepAlive<'a> {
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

mod helper {
    #![allow(dead_code)]

    use crate::error::Result;
    use crate::lease::LeaseClient;
    use crate::pb;

    include!("pb/lease_helper.rs");
}
