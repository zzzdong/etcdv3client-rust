use futures::Future;
use tower_grpc::Request;

use crate::pb::etcdserverpb::client::Auth;
use crate::pb::etcdserverpb::AuthenticateRequest;

use crate::conn::{ConnBuilder, HTTPConn};
use crate::error::EtcdClientError;

pub struct SimpleAuthClient {
    inner: Auth<HTTPConn>,
}

impl SimpleAuthClient {
    pub fn new(
        ip: &str,
        port: u16,
    ) -> impl Future<Item = SimpleAuthClient, Error = EtcdClientError> {
        ConnBuilder::new(ip, port).build().map(|c| SimpleAuthClient {
            inner: Auth::new(c),
        })
    }

    pub fn get_token(
        &mut self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> impl Future<Item = String, Error = EtcdClientError> {
        self.inner
            .authenticate(Request::new(AuthenticateRequest {
                name: name.as_ref().to_owned(),
                password: password.as_ref().to_owned(),
            }))
            .map_err(EtcdClientError::GRPCRequest)
            .and_then(|resp| Ok(resp.into_inner().token))
    }
}
