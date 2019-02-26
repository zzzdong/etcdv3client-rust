// client.rs

use std::string::FromUtf8Error;
use std::sync::Arc;

use failure::Fail;

use crate::kv::SimpleKVClient;

#[derive(Debug, Fail)]
pub enum EtcdClientError {
    #[fail(display = "grpc error: {}", _0)]
    GRPC(grpc::Error),
    #[fail(display = "from utf8 error: {}", _0)]
    FromUtf8(FromUtf8Error),
    #[fail(display = "key not found")]
    KeyNotFound,
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
}
