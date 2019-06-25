use futures::{future, future::Either, Future};
use hyper::client::connect::{Destination, HttpConnector};
use tower_hyper::{client, util};
use tower_util::MakeService;

pub(crate) const TOKEN_ID: &str = "token";

pub(crate) const HTTPCONN_SIZE: usize = 1;

pub(crate) type HTTPConn = tower_request_modifier::RequestModifier<
    tower_hyper::client::Connection<tower_grpc::BoxBody>,
    tower_grpc::BoxBody,
>;

use crate::error::EtcdClientError;


pub struct ConnBuilder {
    ip: String,
    port: u16,
    token: Option<String>,
}

impl ConnBuilder {
    pub fn new(ip: impl AsRef<str>, port: u16) -> Self {
        ConnBuilder {
            ip: ip.as_ref().to_owned(),
            port,
            token: None,
        }
    }

    pub fn with_token(mut self, token: impl AsRef<str>) -> Self {
        self.token = Some(token.as_ref().to_owned());
        self
    }

    pub fn build(self) -> impl Future<Item = HTTPConn, Error = EtcdClientError> {
        let uri: http::Uri = match format!("http://{}:{}", self.ip, self.port).parse() {
            Ok(uri) => uri,
            Err(e) => {
                return Either::A(future::err(EtcdClientError::ErrMsg(format!(
                    "parse uri failed, {:?}",
                    e
                ))))
            }
        };

        let dst = match Destination::try_from_uri(uri.clone()) {
            Ok(dst) => dst,
            Err(e) => {
                return Either::A(future::err(EtcdClientError::ErrMsg(format!(
                    "build dst from uri failed, {:?}",
                    e
                ))))
            }
        };

        let connector = util::Connector::new(HttpConnector::new(HTTPCONN_SIZE));
        let settings = client::Builder::new().http2_only(true).clone();
        let mut make_client = client::Connect::with_builder(connector, settings);

        Either::B(
            make_client
                .make_service(dst)
                .map(move |conn| {
                    let mut c = tower_request_modifier::Builder::new().set_origin(uri);

                    if let Some(ref t) = self.token {
                        c = c.add_header(TOKEN_ID, t.to_owned());
                    }

                    c.build(conn).unwrap()
                })
                .map_err(|e| EtcdClientError::ErrMsg(format!("build http connection failed, {:?}", e))),
        )
    }
}


