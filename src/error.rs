use std::string::FromUtf8Error;

use failure::Fail;

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
