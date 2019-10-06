// kv client
#![allow(dead_code)]

use tonic::Request;

use crate::error::EtcdClientError;
use crate::pb::etcdserverpb::{client::KvClient, DeleteRangeRequest, PutRequest, RangeRequest};
use crate::pb::mvccpb::KeyValue;

pub struct SimpleKvClient {
    inner: KvClient<tonic::transport::channel::Channel>,
}

impl SimpleKvClient {
    pub async fn new(
        endpoints: Vec<impl AsRef<str>>,
        token: Option<&str>,
    ) -> Result<SimpleKvClient, EtcdClientError> {
        let channel = crate::conn::new_channel(endpoints, token)?;

        let client = KvClient::new(channel);
        Ok(SimpleKvClient { inner: client })
    }

    pub async fn get(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<u8>, EtcdClientError> {
        let resp = self
            .inner
            .range(Request::new(RangeRequest {
                key: key.as_ref().to_vec(),
                ..Default::default()
            }))
            .await?;

        resp.into_inner()
            .kvs
            .first()
            .map(|kv| kv.value.to_vec())
            .ok_or_else(|| EtcdClientError::KeyNotFound)
    }

    #[inline]
    pub async fn get_string(&mut self, key: impl AsRef<[u8]>) -> Result<String, EtcdClientError> {
        let resp = self.get(key).await?;

        String::from_utf8(resp).map_err(EtcdClientError::FromUtf8)
    }

    pub async fn get_with_prefix(
        &mut self,
        prefix: impl AsRef<[u8]>,
    ) -> Result<Vec<KeyValue>, EtcdClientError> {
        let resp = self
            .inner
            .range(Request::new(RangeRequest {
                key: prefix.as_ref().to_vec(),
                range_end: SimpleKvClient::build_prefix_end(prefix),
                ..Default::default()
            }))
            .await?;

        Ok(resp.into_inner().kvs)
    }

    pub async fn all(&mut self) -> Result<Vec<KeyValue>, EtcdClientError> {
        let resp = self
            .inner
            .range(Request::new(RangeRequest {
                key: vec![0x00],
                range_end: vec![0x00],
                ..Default::default()
            }))
            .await?;

        Ok(resp.into_inner().kvs.iter().map(|kv| kv.clone()).collect())
    }

    pub async fn put(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), EtcdClientError> {
        let _resp = self
            .inner
            .put(Request::new(PutRequest {
                key: key.as_ref().to_vec(),
                value: value.as_ref().to_vec(),
                ..Default::default()
            }))
            .await?;
        Ok(())
    }

    pub async fn delete(&mut self, key: impl AsRef<[u8]>) -> Result<(), EtcdClientError> {
        let _resp = self
            .inner
            .delete_range(Request::new(DeleteRangeRequest {
                key: key.as_ref().to_vec(),
                ..Default::default()
            }))
            .await?;

        Ok(())
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
