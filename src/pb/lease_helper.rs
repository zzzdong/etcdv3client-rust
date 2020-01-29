impl LeaseClient {
    pub async fn lease_grant(
        &mut self,
        request: pb::LeaseGrantRequest,
    ) -> Result<pb::LeaseGrantResponse> {
        Ok(self.inner.lease_grant(request).await?.into_inner())
    }
    pub async fn lease_revoke(
        &mut self,
        request: pb::LeaseRevokeRequest,
    ) -> Result<pb::LeaseRevokeResponse> {
        Ok(self.inner.lease_revoke(request).await?.into_inner())
    }
    pub async fn lease_time_to_live(
        &mut self,
        request: pb::LeaseTimeToLiveRequest,
    ) -> Result<pb::LeaseTimeToLiveResponse> {
        Ok(self.inner.lease_time_to_live(request).await?.into_inner())
    }
    pub async fn lease_leases(
        &mut self,
        request: pb::LeaseLeasesRequest,
    ) -> Result<pb::LeaseLeasesResponse> {
        Ok(self.inner.lease_leases(request).await?.into_inner())
    }
}
pub struct DoLeaseGrantRequest<'a> {
    pub request: pb::LeaseGrantRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseGrantRequest<'a> {
    pub async fn finish(self) -> Result<pb::LeaseGrantResponse> {
        let DoLeaseGrantRequest { request, client } = self;
        client.lease_grant(request).await
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
pub struct DoLeaseRevokeRequest<'a> {
    pub request: pb::LeaseRevokeRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseRevokeRequest<'a> {
    pub async fn finish(self) -> Result<pb::LeaseRevokeResponse> {
        let DoLeaseRevokeRequest { request, client } = self;
        client.lease_revoke(request).await
    }
    pub fn with_id(mut self, id: i64) -> Self {
        self.request.id = id;
        self
    }
}
pub struct DoLeaseTimeToLiveRequest<'a> {
    pub request: pb::LeaseTimeToLiveRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseTimeToLiveRequest<'a> {
    pub async fn finish(self) -> Result<pb::LeaseTimeToLiveResponse> {
        let DoLeaseTimeToLiveRequest { request, client } = self;
        client.lease_time_to_live(request).await
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
pub struct DoLeaseLeasesRequest<'a> {
    pub request: pb::LeaseLeasesRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseLeasesRequest<'a> {
    pub async fn finish(self) -> Result<pb::LeaseLeasesResponse> {
        let DoLeaseLeasesRequest { request, client } = self;
        client.lease_leases(request).await
    }
}
