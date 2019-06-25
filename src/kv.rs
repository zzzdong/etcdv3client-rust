// kv client
#![allow(dead_code)]

use futures::Future;

use tower_grpc::Request;

use crate::pb::etcdserverpb::client::Kv;
use crate::pb::etcdserverpb::{DeleteRangeRequest, PutRequest, RangeRequest};
use crate::pb::mvccpb::KeyValue;

use crate::conn::{ConnBuilder, HTTPConn};
use crate::error::EtcdClientError;

pub struct SimpleKvClient {
    inner: Kv<HTTPConn>,
}

impl SimpleKvClient {
    pub fn new(
        ip: &str,
        port: u16,
        token: Option<String>,
    ) -> impl Future<Item = SimpleKvClient, Error = EtcdClientError> {
        let mut c = ConnBuilder::new(ip, port);
        if let Some(t) = token {
            c = c.with_token(t);
        }

        c.build().map(|c| SimpleKvClient { inner: Kv::new(c) })
    }

    pub fn get(
        &mut self,
        key: impl AsRef<[u8]>,
    ) -> impl Future<Item = Vec<u8>, Error = EtcdClientError> {
        self.inner
            .range(Request::new(RangeRequest {
                key: key.as_ref().to_vec(),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|resp| {
                resp.into_inner()
                    .kvs
                    .first()
                    .map(|kv| kv.value.to_vec())
                    .ok_or_else(|| EtcdClientError::KeyNotFound)
            })
    }

    #[inline]
    pub fn get_string(
        &mut self,
        key: impl AsRef<[u8]>,
    ) -> impl Future<Item = String, Error = EtcdClientError> {
        self.get(key)
            .and_then(|bs| String::from_utf8(bs).map_err(EtcdClientError::FromUtf8))
    }

    pub fn get_with_prefix(
        &mut self,
        prefix: impl AsRef<[u8]>,
    ) -> impl Future<Item = Vec<KeyValue>, Error = EtcdClientError> {
        self.inner
            .range(Request::new(RangeRequest {
                key: prefix.as_ref().to_vec(),
                range_end: SimpleKvClient::build_prefix_end(prefix),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|resp| Ok(resp.into_inner().kvs))
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

    pub fn put(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> impl Future<Item = (), Error = EtcdClientError> {
        self.inner
            .put(Request::new(PutRequest {
                key: key.as_ref().to_vec(),
                value: value.as_ref().to_vec(),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|_resp| Ok(()))
    }

    pub fn delete(
        &mut self,
        key: impl AsRef<[u8]>,
    ) -> impl Future<Item = (), Error = EtcdClientError> {
        self.inner
            .delete_range(Request::new(DeleteRangeRequest {
                key: key.as_ref().to_vec(),
                ..Default::default()
            }))
            .map_err(|e| EtcdClientError::GRPCRequest(e))
            .and_then(|_resp| Ok(()))
    }

    fn build_prefix_end(prefix: impl AsRef<[u8]>) -> Vec<u8> {
        let no_prefix_end = Vec::new();

        if prefix.as_ref().is_empty() {
            return no_prefix_end;
        }

        let mut end = prefix.as_ref().to_vec();

        for i in (0..end.len()).rev() {
            if end[i] < 0xff {
                end[i] += 1;
                return end[0..=i].to_vec();
            }
        }

        no_prefix_end
    }
}
