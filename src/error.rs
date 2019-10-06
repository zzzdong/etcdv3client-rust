use http::header;
use http::uri;
use std::string::FromUtf8Error;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum EtcdClientError {
    #[fail(display = "uri error: {}", _0)]
    Uri(uri::InvalidUri),
    #[fail(display = "headervalue error: {}", _0)]
    HeaderValue(header::InvalidHeaderValue),
    #[fail(display = "from utf8 error: {}", _0)]
    FromUtf8(FromUtf8Error),
    #[fail(display = "error message: {}", _0)]
    ErrMsg(String),
    #[fail(display = "GRPC request error: {:?}", _0)]
    GRPCRequest(tonic::Status),
    #[fail(display = "key not found")]
    KeyNotFound,
}

impl From<uri::InvalidUri> for EtcdClientError {
    fn from(e: uri::InvalidUri) -> Self {
        EtcdClientError::Uri(e)
    }
}

impl From<header::InvalidHeaderValue> for EtcdClientError {
    fn from(e: header::InvalidHeaderValue) -> Self {
        EtcdClientError::HeaderValue(e)
    }
}

impl From<tonic::Status> for EtcdClientError {
    fn from(e: tonic::Status) -> Self {
        EtcdClientError::GRPCRequest(e)
    }
}
