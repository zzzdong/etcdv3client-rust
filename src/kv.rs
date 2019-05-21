// kv client
#![allow(dead_code)]

use failure::{err_msg, Error};
use futures::{future, future::Either, Future};
use hyper::client::connect::{Destination, HttpConnector};
use tower_grpc::Request;

use tower_hyper::{client, util};
use tower_util::MakeService;

use crate::comm::etcdserverpb::client::Kv;
use crate::comm::etcdserverpb::{DeleteRangeRequest, PutRequest, RangeRequest};
use crate::comm::mvccpb::KeyValue;

use crate::comm::EtcdClientError;
use crate::comm::HTTPConn;

pub struct SimpleKV {
    pub key: String,
    pub value: Vec<u8>,
}

pub struct BytesKV {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

pub struct SimpleKvClient {
    inner: Kv<HTTPConn>,
}

impl SimpleKvClient {
    pub fn new(ip: &str, port: u16) -> impl Future<Item = SimpleKvClient, Error = EtcdClientError> {
        let uri: http::Uri = match format!("http://{}:{}", ip, port).parse() {
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

        let connector = util::Connector::new(HttpConnector::new(4));
        let settings = client::Builder::new().http2_only(true).clone();
        let mut make_client = client::Connect::with_builder(connector, settings);

        Either::B(
            make_client
                .make_service(dst)
                .map(move |conn| {
                    let conn = tower_request_modifier::Builder::new()
                        .set_origin(uri)
                        .build(conn)
                        .unwrap();

                    SimpleKvClient {
                        inner: Kv::new(conn),
                    }
                })
                .map_err(|e| EtcdClientError::ErrMsg(format!("make service failed, {:?}", e))),
        )
    }

    pub fn get_bytes(
        &mut self,
        key: &str,
    ) -> impl Future<Item = Option<Vec<u8>>, Error = EtcdClientError> {
        self.inner
            .range(Request::new(RangeRequest {
                key: key.as_bytes().to_vec(),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|resp| Ok(resp.into_inner().kvs.first().map(|kv| kv.value.to_vec())))
    }

    #[inline]
    pub fn get_string(
        &mut self,
        key: &str,
    ) -> impl Future<Item = Option<String>, Error = EtcdClientError> {
        self.get_bytes(key)
            .map(|bs| bs.map(|b| String::from_utf8_lossy(&b).to_string()))
    }

    pub fn get_with_prefix(
        &mut self,
        prefix: &str,
    ) -> impl Future<Item = Vec<SimpleKV>, Error = EtcdClientError> {
        self.inner
            .range(Request::new(RangeRequest {
                key: prefix.as_bytes().to_vec(),
                range_end: SimpleKvClient::build_prefix_end(prefix),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|resp| Ok(resp.into_inner().kvs.iter().map(|kv| SimpleKV{
                key:  String::from_utf8_lossy(&kv.key).to_string(),
                value: kv.value.to_vec(),
            } ).collect()))
    }

    pub fn all(&mut self) -> impl Future<Item = Vec<KeyValue>, Error = EtcdClientError> {
        self.inner
            .range(Request::new(RangeRequest {
                key: vec![0x00],
                range_end: vec![0x00],
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|resp| Ok(resp.into_inner().kvs.iter().map(|kv| kv.clone()).collect()))
    }

    pub fn put_bytes(
        &mut self,
        key: &str,
        value: &[u8],
    ) -> impl Future<Item = (), Error = EtcdClientError> {
        self.inner
            .put(Request::new(PutRequest {
                key: key.as_bytes().to_vec(),
                value: value.to_vec(),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|_resp| Ok(()))
    }

    #[inline]
    pub fn put_string(
        &mut self,
        key: &str,
        value: &str,
    ) -> impl Future<Item = (), Error = EtcdClientError> {
        self.put_bytes(key, value.as_bytes())
    }

    pub fn delete(&mut self, key: &str) -> impl Future<Item = (), Error = EtcdClientError> {
        self.inner
            .delete_range(Request::new(DeleteRangeRequest {
                key: key.as_bytes().to_vec(),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|_resp| Ok(()))
    }

    fn build_prefix_end(prefix: &str) -> Vec<u8> {
        let no_prefix_end = Vec::new();

        let b = prefix.as_bytes();
        if b.len() == 0 {
            return no_prefix_end;
        }

        let mut end = b.to_vec();

        for i in (0..end.len()).rev() {
            if end[i] < 0xff {
                end[i] = end[i] + 1;
                return end[0..i + 1].to_vec();
            }
        }

        return no_prefix_end;
    }
}
