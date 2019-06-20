// client.rs

use std::string::FromUtf8Error;
use std::sync::Arc;

use failure::Fail;

use crate::kv::SimpleKVClient;
use crate::auth::SimpleAuthClient;

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

    // pub fn auth(self, user: &str, password: &str) -> Result<EtcdV3Client, EtcdClientError> {
    //     self.new_auth().get_token(user, password).map(|_|self).map_err(|_| EtcdClientError::AuthFailed)
    // }

    pub fn new_simple_kv(&self) -> SimpleKVClient {
        SimpleKVClient::new(self.conn.clone())
    }

    pub fn new_auth(&self) -> SimpleAuthClient {
        SimpleAuthClient::new(self.conn.clone())
    }
}
