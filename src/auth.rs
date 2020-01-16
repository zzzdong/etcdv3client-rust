use std::fmt;

use crate::error::{EtcdClientError, Result};
use crate::pb::{self, auth_client::AuthClient as PbAuthClient};
use crate::EtcdClient;

use tonic::transport::channel::Channel;

pub struct AuthClient {
    inner: PbAuthClient<Channel>,
}

impl AuthClient {
    pub fn new(channel: Channel, interceptor: Option<tonic::Interceptor>) -> Self {
        let client = match interceptor {
            Some(i) => PbAuthClient::with_interceptor(channel, i),
            None => PbAuthClient::new(channel),
        };

        AuthClient { inner: client }
    }

    pub fn with_client(client: &EtcdClient) -> Self {
        let channel = client.channel.clone();
        let interceptor = client.interceptor.clone();
        Self::new(channel, interceptor)
    }

    pub async fn do_authenticate(
        &mut self,
        request: pb::AuthenticateRequest,
    ) -> Result<pb::AuthenticateResponse> {
        Ok(self.inner.authenticate(request).await?.into_inner())
    }
}
