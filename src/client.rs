// client.rs

use std::string::FromUtf8Error;
use std::sync::Arc;

use bytes::Bytes;
use failure::Fail;

use crate::auth::SimpleAuthClient;
use crate::kv::SimpleKVClient;

#[derive(Debug, Fail)]
pub enum EtcdClientError {
    #[fail(display = "grpc error: {}", _0)]
    GRPC(grpc::Error),
    #[fail(display = "from utf8 error: {}", _0)]
    FromUtf8(FromUtf8Error),
    #[fail(display = "key not found")]
    KeyNotFound,
    #[fail(display = "auth failed")]
    AuthFailed,
}

pub struct EtcdV3Client {
    conn: Arc<grpc::Client>,
}

impl EtcdV3Client {
    pub fn new(host: &str, port: u16) -> Result<EtcdV3Client, EtcdClientError> {
        let grpc_client = Arc::new(
            grpc::Client::new_plain(host, port, Default::default())
                .map_err(EtcdClientError::GRPC)?,
        );
        Ok(EtcdV3Client { conn: grpc_client })
    }

    pub fn new_simple_kv(&self) -> SimpleKVClient {
        SimpleKVClient::new(self.conn.clone())
    }

    pub fn new_auth(&self) -> SimpleAuthClient {
        SimpleAuthClient::new(self.conn.clone())
    }
}

pub(crate) struct RequestOptsBuilder {
    token: Option<String>,
}

impl RequestOptsBuilder {
    pub fn new() -> RequestOptsBuilder {
        RequestOptsBuilder { token: None }
    }

    pub fn with_token(&mut self, token: impl AsRef<str>) {
        self.token = Some(token.as_ref().to_owned());
    }

    pub fn build(&self) -> grpc::RequestOptions {
        let mut opt = grpc::RequestOptions::new();

        if let Some(ref t) = self.token {
            opt.metadata
                .add(grpc::MetadataKey::from("token"), Bytes::from(t.as_bytes()))
        }

        opt
    }
}
