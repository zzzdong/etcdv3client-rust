use std::string::FromUtf8Error;

use failure::Fail;
use tower_hyper::client::ConnectError;

#[derive(Debug, Fail)]
pub enum EtcdClientError {
    #[fail(display = "connect error: {}", _0)]
    Connect(ConnectError<std::io::Error>),
    #[fail(display = "from utf8 error: {}", _0)]
    FromUtf8(FromUtf8Error),
    #[fail(display = "error message: {}", _0)]
    ErrMsg(String),
    #[fail(display = "GRPC request error: {:?}", _0)]
    GRPCRequest(tower_grpc::Status),
    #[fail(display = "key not found")]
    KeyNotFound,
}
