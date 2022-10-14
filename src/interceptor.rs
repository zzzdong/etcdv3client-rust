use std::convert::TryFrom;
use std::sync::{Arc, RwLock};

use tonic::{metadata::MetadataValue, service::Interceptor};

pub(crate) const TOKEN_FIELD_NAME: &str = "token";

#[derive(Debug, Clone)]
pub(crate) struct CredentialInterceptor {
    token: Arc<RwLock<String>>,
}

impl CredentialInterceptor {
    pub(crate) fn new(token: Arc<RwLock<String>>) -> Self {
        CredentialInterceptor { token }
    }

    pub fn empty() -> Self {
        Self::new(Arc::new(RwLock::new(String::new())))
    }
}

impl Interceptor for CredentialInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let token = self.token.read().unwrap();
        if !token.is_empty() {
            let token = MetadataValue::try_from(token.as_str()).unwrap();
            request.metadata_mut().insert(TOKEN_FIELD_NAME, token);
        }

        Ok(request)
    }
}
