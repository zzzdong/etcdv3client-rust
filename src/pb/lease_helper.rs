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
    pub fn from_client(client: &'a mut LeaseClient) -> Self {
        DoLeaseGrantRequest {
            request: Default::default(),
            client,
        }
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
impl<'a> std::future::IntoFuture for DoLeaseGrantRequest<'a> {
    type Output = Result<pb::LeaseGrantResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseGrantResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseGrantRequest { request, client } = self;
        Box::pin(async move { client.lease_grant(request).await })
    }
}
pub struct DoLeaseRevokeRequest<'a> {
    pub request: pb::LeaseRevokeRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseRevokeRequest<'a> {
    pub fn from_client(client: &'a mut LeaseClient) -> Self {
        DoLeaseRevokeRequest {
            request: Default::default(),
            client,
        }
    }
    pub fn with_id(mut self, id: i64) -> Self {
        self.request.id = id;
        self
    }
}
impl<'a> std::future::IntoFuture for DoLeaseRevokeRequest<'a> {
    type Output = Result<pb::LeaseRevokeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseRevokeResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseRevokeRequest { request, client } = self;
        Box::pin(async move { client.lease_revoke(request).await })
    }
}
pub struct DoLeaseTimeToLiveRequest<'a> {
    pub request: pb::LeaseTimeToLiveRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseTimeToLiveRequest<'a> {
    pub fn from_client(client: &'a mut LeaseClient) -> Self {
        DoLeaseTimeToLiveRequest {
            request: Default::default(),
            client,
        }
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
impl<'a> std::future::IntoFuture for DoLeaseTimeToLiveRequest<'a> {
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
pub struct DoLeaseLeasesRequest<'a> {
    pub request: pb::LeaseLeasesRequest,
    pub(crate) client: &'a mut LeaseClient,
}
impl<'a> DoLeaseLeasesRequest<'a> {
    pub fn from_client(client: &'a mut LeaseClient) -> Self {
        DoLeaseLeasesRequest {
            request: Default::default(),
            client,
        }
    }
}
impl<'a> std::future::IntoFuture for DoLeaseLeasesRequest<'a> {
    type Output = Result<pb::LeaseLeasesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseLeasesResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseLeasesRequest { request, client } = self;
        Box::pin(async move { client.lease_leases(request).await })
    }
}
