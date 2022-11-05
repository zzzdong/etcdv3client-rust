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
#[derive(Debug, Clone)]
pub struct DoLeaseGrantRequest {
    pub request: pb::LeaseGrantRequest,
    pub(crate) client: LeaseClient,
}
impl DoLeaseGrantRequest {
    pub fn from_client(client: &LeaseClient) -> Self {
        DoLeaseGrantRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoLeaseGrantRequest {
    type Output = Result<pb::LeaseGrantResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseGrantResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseGrantRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.lease_grant(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoLeaseRevokeRequest {
    pub request: pb::LeaseRevokeRequest,
    pub(crate) client: LeaseClient,
}
impl DoLeaseRevokeRequest {
    pub fn from_client(client: &LeaseClient) -> Self {
        DoLeaseRevokeRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_id(mut self, id: i64) -> Self {
        self.request.id = id;
        self
    }
}
impl std::future::IntoFuture for DoLeaseRevokeRequest {
    type Output = Result<pb::LeaseRevokeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseRevokeResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseRevokeRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.lease_revoke(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoLeaseTimeToLiveRequest {
    pub request: pb::LeaseTimeToLiveRequest,
    pub(crate) client: LeaseClient,
}
impl DoLeaseTimeToLiveRequest {
    pub fn from_client(client: &LeaseClient) -> Self {
        DoLeaseTimeToLiveRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoLeaseTimeToLiveRequest {
    type Output = Result<pb::LeaseTimeToLiveResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseTimeToLiveResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseTimeToLiveRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.lease_time_to_live(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoLeaseLeasesRequest {
    pub request: pb::LeaseLeasesRequest,
    pub(crate) client: LeaseClient,
}
impl DoLeaseLeasesRequest {
    pub fn from_client(client: &LeaseClient) -> Self {
        DoLeaseLeasesRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}
impl std::future::IntoFuture for DoLeaseLeasesRequest {
    type Output = Result<pb::LeaseLeasesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::LeaseLeasesResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoLeaseLeasesRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.lease_leases(request).await })
    }
}
