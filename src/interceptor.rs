use std::convert::TryFrom;

use tonic::{metadata::MetadataValue, service::Interceptor};

pub(crate) const TOKEN_ID: &str = "token";

#[derive(Debug, Clone)]
pub(crate) struct InsertTokenHeader {
    token: Option<String>,
}

impl InsertTokenHeader {
    pub(crate) fn new(token: Option<String>) -> Self {
        InsertTokenHeader { token }
    }

    pub fn empty() -> Self {
        Self::new(None)
    }
}

impl Interceptor for InsertTokenHeader {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        if let Some(ref t) = self.token {
            let token = MetadataValue::try_from(t).unwrap();
            request.metadata_mut().insert(TOKEN_ID, token);
        }

        Ok(request)
    }
}
