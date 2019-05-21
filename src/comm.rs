// comm.rs

use std::string::FromUtf8Error;

use failure::Fail;
use tower_hyper::client::ConnectError;

pub mod mvccpb {
    include!(concat!(env!("OUT_DIR"), "/mvccpb.rs"));
}

pub mod authpb {
    include!(concat!(env!("OUT_DIR"), "/authpb.rs"));
}

pub mod etcdserverpb {
    include!(concat!(env!("OUT_DIR"), "/etcdserverpb.rs"));
}

pub(crate) type HTTPConn = tower_request_modifier::RequestModifier<
    tower_hyper::client::Connection<tower_grpc::BoxBody>,
    tower_grpc::BoxBody,
>;

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
