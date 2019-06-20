use std::sync::Arc;

use futures::Future;

use grpc::ClientStub;

pub use crate::pb::kv::KeyValue;
pub use crate::pb::rpc::{AuthenticateRequest, AuthenticateResponse};
pub use crate::pb::rpc_grpc::{Auth as Service, AuthClient};

use crate::client::EtcdClientError;

pub struct SimpleAuthClient {
    inner: AuthClient,
}

impl SimpleAuthClient {
    /// Reture a SimpleAuthClient
    pub fn new(client: Arc<grpc::Client>) -> SimpleAuthClient {
        SimpleAuthClient {
            inner: AuthClient::with_client(client),
        }
    }

    pub fn get_token(&self, name: &str, password: &str) -> Result<String, EtcdClientError> {
        let mut req = AuthenticateRequest::new();
        req.set_name(name.to_string());
        req.set_password(password.to_string());

        let resp = self.inner.authenticate(grpc::RequestOptions::new(), req).wait_drop_metadata()
            .map_err(EtcdClientError::GRPC)?;
        
        Ok(resp.get_token().to_string())
    }

}
