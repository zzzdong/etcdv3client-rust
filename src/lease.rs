use std::fmt;

use crate::error::{LeaseError, Result};
use crate::pb::{self, lease_client::LeaseClient as PbLeaseClient};
use crate::EtcdClient;

use tonic::transport::channel::Channel;

use tokio::sync::mpsc::{channel, Sender};
use tonic::codec::Streaming;

const MPSC_CHANNEL_SIZE: usize = 1;

pub struct LeaseClient {
    inner: PbLeaseClient<Channel>,
}

impl LeaseClient {
    pub fn new(channel: Channel, interceptor: Option<tonic::Interceptor>) -> Self {
        let client = match interceptor {
            Some(i) => PbLeaseClient::with_interceptor(channel, i),
            None => PbLeaseClient::new(channel),
        };

        LeaseClient { inner: client }
    }

    pub fn with_client(client: &EtcdClient) -> Self {
        let channel = client.channel.clone();
        let interceptor = client.interceptor.clone();
        Self::new(channel, interceptor)
    }

    /// LeaseGrant creates a lease which expires if the server
    /// does not receive a keepAlive within a given time to live period.
    pub async fn lease_grant(
        &mut self,
        request: pb::LeaseGrantRequest,
    ) -> Result<pb::LeaseGrantResponse> {
        Ok(self.inner.lease_grant(request).await?.into_inner())
    }

    pub fn do_lease_grant(&mut self, ttl: i64) -> DoLeaseGrant {
        DoLeaseGrant::new(ttl, self)
    }

    pub async fn grant_lease(&mut self, ttl: i64, lease_id: i64) -> Result<pb::LeaseGrantResponse> {
        self.do_lease_grant(ttl)
            .with_lease_id(lease_id)
            .finish()
            .await
    }

    /// LeaseRevoke revokes a lease.
    pub async fn lease_revoke(
        &mut self,
        request: pb::LeaseRevokeRequest,
    ) -> Result<pb::LeaseRevokeResponse> {
        Ok(self.inner.lease_revoke(request).await?.into_inner())
    }

    pub async fn revoke_lease(&mut self, lease_id: i64) -> Result<()> {
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

    pub fn do_lease_keep_alive(&mut self, lease_id: i64) -> DoLeaseKeepAlive {
        DoLeaseKeepAlive::new(lease_id, self)
    }

    pub async fn keep_lease_alive(&mut self, lease_id: i64) -> Result<LeaseKeepAliver> {
        self.do_lease_keep_alive(lease_id).finish().await
    }

    /// LeaseTimeToLive retrieves lease information.
    pub async fn lease_time_to_live(
        &mut self,
        request: pb::LeaseTimeToLiveRequest,
    ) -> Result<pb::LeaseTimeToLiveResponse> {
        Ok(self.inner.lease_time_to_live(request).await?.into_inner())
    }

    pub async fn get_lease_info(
        &mut self,
        lease_id: i64,
        keys: bool,
    ) -> Result<pb::LeaseTimeToLiveResponse> {
        self.lease_time_to_live(pb::LeaseTimeToLiveRequest::new(lease_id, keys))
            .await
    }

    /// LeaseLeases lists all existing leases.
    pub async fn lease_leases(
        &mut self,
        request: pb::LeaseLeasesRequest,
    ) -> Result<pb::LeaseLeasesResponse> {
        Ok(self.inner.lease_leases(request).await?.into_inner())
    }

    pub async fn list_leases(&mut self) -> Result<pb::LeaseLeasesResponse> {
        self.lease_leases(pb::LeaseLeasesRequest::new()).await
    }
}

impl pb::LeaseGrantRequest {
    fn new(ttl: i64, id: i64) -> Self {
        pb::LeaseGrantRequest { ttl, id }
    }
}

pub struct DoLeaseGrant<'a> {
    pub request: pb::LeaseGrantRequest,
    client: &'a mut LeaseClient,
}

impl<'a> DoLeaseGrant<'a> {
    pub fn new(ttl: i64, client: &'a mut LeaseClient) -> Self {
        DoLeaseGrant {
            request: pb::LeaseGrantRequest::new(ttl, 0),
            client,
        }
    }

    pub async fn finish(self) -> Result<pb::LeaseGrantResponse> {
        let DoLeaseGrant { request, client } = self;
        client.lease_grant(request).await
    }

    pub fn with_lease_id(mut self, lease_id: i64) -> Self {
        self.request.id = lease_id;
        self
    }
}

impl<'a> fmt::Debug for DoLeaseGrant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DoLeaseGrant")
            .field("request", &self.request)
            .finish()
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
    pub request: pb::LeaseKeepAliveRequest,
    client: &'a mut LeaseClient,
}

impl<'a> DoLeaseKeepAlive<'a> {
    pub fn new(lease_id: i64, client: &'a mut LeaseClient) -> Self {
        DoLeaseKeepAlive {
            request: pb::LeaseKeepAliveRequest::new(lease_id),
            client,
        }
    }

    pub async fn finish(self) -> Result<LeaseKeepAliver> {
        let DoLeaseKeepAlive { request, client } = self;

        let (req_tx, req_rx) = channel::<pb::LeaseKeepAliveRequest>(MPSC_CHANNEL_SIZE);

        let resp = client.lease_keep_alive(req_rx).await?;

        Ok(LeaseKeepAliver::new(request.id, req_tx, resp))
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
