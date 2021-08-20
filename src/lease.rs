use std::fmt;

use tokio::sync::mpsc::{channel, Sender};
use tonic::codec::Streaming;
use tonic::transport::channel::Channel;

use crate::error::{LeaseError, Result};
use crate::pb::{self, lease_client::LeaseClient as PbLeaseClient};
use crate::EtcdClient;

use helper::*;

const MPSC_CHANNEL_SIZE: usize = 1;

pub struct LeaseClient {
    inner: PbLeaseClient<Channel>,
}

impl LeaseClient {
    pub fn new(channel: Channel) -> Self {
        let client = PbLeaseClient::new(channel);
        LeaseClient { inner: client }
    }

    pub fn with_client(client: &EtcdClient) -> Self {
        let channel = client.channel.clone();
        Self::new(channel)
    }

    pub fn do_grant(&mut self, ttl: i64) -> DoLeaseGrantRequest {
        DoLeaseGrantRequest::new(ttl, self)
    }

    pub async fn grant(&mut self, ttl: i64, lease_id: i64) -> Result<pb::LeaseGrantResponse> {
        self.do_grant(ttl).with_lease_id(lease_id).finish().await
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
        self.do_keep_alive(lease_id).finish().await
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

    pub async fn finish(self) -> Result<LeaseKeepAliver> {
        let DoLeaseKeepAlive { lease_id, client } = self;

        let (req_tx, req_rx) = channel::<pb::LeaseKeepAliveRequest>(MPSC_CHANNEL_SIZE);

        let request = pb::LeaseKeepAliveRequest::new(lease_id);
        req_tx.send(request).await.map_err(LeaseError::from)?;

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
            .map_err(LeaseError::LeaseKeepAliveRequestError)?;

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
