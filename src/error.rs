use std::string::FromUtf8Error;

use http::header;
use http::uri;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::pb::LeaseKeepAliveRequest;
use crate::pb::WatchRequest;

pub(crate) type Result<T> = std::result::Result<T, EtcdClientError>;

#[derive(Error, Debug)]
pub enum EtcdClientError {
    #[error("uri invalid")]
    Uri(#[from] uri::InvalidUri),
    #[error("headervalue invalid")]
    HeaderValue(#[from] header::InvalidHeaderValue),
    #[error("from utf8 error")]
    FromUtf8(#[from] FromUtf8Error),
    #[error("GRPC request error")]
    GRPCError(#[from] tonic::Status),
    #[error("transport error")]
    TransportError(#[from] tonic::transport::Error),
    #[error("error message: {0}")]
    ErrMsg(String),
    #[error("key not found")]
    KeyNotFound,
    #[error("endpoint error")]
    EndpointError,
    #[error("watch error")]
    WatchError(#[from] WatchError),
    #[error("lease error")]
    LeaseError(#[from] LeaseError),
}

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("watch request error")]
    WatchRequestError(#[from] SendError<WatchRequest>),
    #[error("start watch failed")]
    StartWatchError,
    #[error("watch canceled")]
    WatchCanceled,
    #[error("watch finished")]
    WatchFinished,
}

#[derive(Error, Debug)]
pub enum LeaseError {
    #[error("watch request error")]
    LeaseKeepAliveRequestError(#[from] SendError<LeaseKeepAliveRequest>),
}
