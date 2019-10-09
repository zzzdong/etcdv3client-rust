// auth client
#![allow(dead_code)]

use tonic::Request;

use crate::error::EtcdClientError;
use crate::pb::etcdserverpb::client::AuthClient;
use crate::pb::etcdserverpb::AuthenticateRequest;

pub struct SimpleAuthClient {
    inner: AuthClient<tonic::transport::channel::Channel>,
}

impl SimpleAuthClient {
    pub async fn new(
        endpoints: Vec<impl AsRef<str>>,
        token: Option<impl AsRef<str>>,
    ) -> Result<SimpleAuthClient, EtcdClientError> {
        let channel = crate::conn::new_channel(endpoints, token)?;

        let client = AuthClient::new(channel);
        Ok(SimpleAuthClient { inner: client })
    }

    pub async fn get_token(
        &mut self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<String, EtcdClientError> {
        let resp = self
            .inner
            .authenticate(Request::new(AuthenticateRequest {
                name: name.as_ref().to_owned(),
                password: password.as_ref().to_owned(),
            }))
            .await?;

        Ok(resp.into_inner().token)
    }
}
