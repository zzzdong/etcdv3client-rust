use crate::error::{EtcdClientError, WatchError};
use crate::pb::*;
use crate::pb::{
    auth_client::AuthClient, cluster_client::ClusterClient, kv_client::KvClient,
    lease_client::LeaseClient, maintenance_client::MaintenanceClient, watch_client::WatchClient,
};
use crate::watcher::Watcher;

use tokio::sync::mpsc;
use tonic::transport::channel::Channel;
use tonic::{metadata::MetadataValue, Request};

pub(crate) const TOKEN_ID: &str = "token";

pub struct EtcdClient {
    pub kv: KvClient<Channel>,
    pub watch: WatchClient<Channel>,
    pub lease: LeaseClient<Channel>,
    pub cluster: ClusterClient<Channel>,
    pub maintenance: MaintenanceClient<Channel>,
    pub auth: AuthClient<Channel>,
}

impl EtcdClient {
    /// Create a new EtcdClient
    pub async fn new(
        endpoints: Vec<impl AsRef<str>>,
        auth: Option<(String, String)>,
    ) -> Result<Self, EtcdClientError> {
        let mut token = None;

        if let Some((name, password)) = auth {
            for endpoint in &endpoints {
                if let Ok(t) = get_token(endpoint, &name, &password).await {
                    token = Some(t);
                    break;
                }
            }
        }

        let token = token.map(|t| MetadataValue::from_str(&t).unwrap());

        let channel = crate::conn::new_channel(endpoints).await?;

        let interceptor = move |mut req: Request<()>| {
            if let Some(token) = token.clone() {
                req.metadata_mut().insert(TOKEN_ID, token);
            }
            Ok(req)
        };

        Ok(EtcdClient {
            kv: KvClient::with_interceptor(channel.clone(), interceptor.clone()),
            watch: WatchClient::with_interceptor(channel.clone(), interceptor.clone()),
            lease: LeaseClient::with_interceptor(channel.clone(), interceptor.clone()),
            cluster: ClusterClient::with_interceptor(channel.clone(), interceptor.clone()),
            maintenance: MaintenanceClient::with_interceptor(channel.clone(), interceptor.clone()),
            auth: AuthClient::with_interceptor(channel, interceptor),
        })
    }

    /// get a value with the given key
    pub async fn get(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<u8>, EtcdClientError> {
        let req = RangeRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        };

        let resp = self.kv.range(req).await?.into_inner();

        resp.kvs
            .first()
            .map(|kv| kv.value.to_vec())
            .ok_or_else(|| EtcdClientError::KeyNotFound)
    }

    /// get a string with the given key
    pub async fn get_string(&mut self, key: impl AsRef<[u8]>) -> Result<String, EtcdClientError> {
        let value = self.get(key).await?;
        String::from_utf8(value).map_err(EtcdClientError::from)
    }

    /// get key-value paire with prefix
    pub async fn get_prefix(
        &mut self,
        key: impl AsRef<[u8]>,
    ) -> Result<Vec<KeyValue>, EtcdClientError> {
        let req = RangeRequest {
            key: key.as_ref().to_vec(),
            range_end: build_prefix_end(key),
            ..Default::default()
        };

        let resp = self.kv.range(req).await?.into_inner();

        Ok(resp.kvs)
    }

    /// get all key-value pairs
    pub async fn get_all(&mut self) -> Result<Vec<KeyValue>, EtcdClientError> {
        let req = RangeRequest {
            key: vec![0x00],
            range_end: vec![0x00],
            ..Default::default()
        };

        let resp = self.kv.range(req).await?.into_inner();

        Ok(resp.kvs)
    }

    /// put a key-value pair
    pub async fn put(
        &mut self,
        key: impl AsRef<[u8]>,
        value: impl AsRef<[u8]>,
    ) -> Result<(), EtcdClientError> {
        let req = PutRequest {
            key: key.as_ref().to_vec(),
            value: value.as_ref().to_vec(),
            ..Default::default()
        };
        let _resp = self.kv.put(req).await?;
        Ok(())
    }

    /// delete a key-value
    pub async fn delete(&mut self, key: impl AsRef<[u8]>) -> Result<(), EtcdClientError> {
        let req = DeleteRangeRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        };
        let _resp = self.kv.delete_range(req).await?.into_inner();

        Ok(())
    }

    /// delete key-value pairs with prefix
    pub async fn delete_prefix(
        &mut self,
        key: impl AsRef<[u8]>,
    ) -> Result<Vec<KeyValue>, EtcdClientError> {
        let req = DeleteRangeRequest {
            key: key.as_ref().to_vec(),
            range_end: build_prefix_end(key),
            ..Default::default()
        };

        let resp = self.kv.delete_range(req).await?.into_inner();

        Ok(resp.prev_kvs)
    }

    /// get a token with the given username and password
    pub async fn get_token(
        &mut self,
        username: impl ToString,
        password: impl ToString,
    ) -> Result<String, EtcdClientError> {
        let req = AuthenticateRequest {
            name: username.to_string(),
            password: password.to_string(),
        };

        let resp = self.auth.authenticate(req).await?.into_inner();

        Ok(resp.token)
    }

    /// watch a key
    pub async fn watch(&mut self, key: impl AsRef<[u8]>) -> Result<Watcher, EtcdClientError> {
        let req = WatchCreateRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        };

        self.inner_watch(req).await
    }

    /// watch a key with prefix
    pub async fn watch_prefix(
        &mut self,
        key: impl AsRef<[u8]>,
    ) -> Result<Watcher, EtcdClientError> {
        let req = WatchCreateRequest {
            key: key.as_ref().to_vec(),
            ..Default::default()
        };

        self.inner_watch(req).await
    }

    async fn inner_watch(&mut self, req: WatchCreateRequest) -> Result<Watcher, EtcdClientError> {
        let (req_tx, req_rx) = mpsc::unbounded_channel::<WatchRequest>();

        let create_watch = watch_request::RequestUnion::CreateRequest(req);
        let create_req = WatchRequest {
            request_union: Some(create_watch),
        };

        req_tx.send(create_req).map_err(WatchError::from)?;

        let resp = self.watch.watch(req_rx).await?;
        let mut inbound = resp.into_inner();
        let watch_id;

        match inbound.message().await? {
            Some(msg) => watch_id = msg.watch_id,
            None => return Err(EtcdClientError::from(WatchError::StartWatchError)),
        }

        let watcher = Watcher::new(watch_id, req_tx, inbound);

        Ok(watcher)
    }
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

async fn get_token(
    endpoint: impl AsRef<str>,
    name: &str,
    password: &str,
) -> Result<String, EtcdClientError> {
    let channel = crate::conn::new_channel(vec![endpoint]).await?;

    let mut auth_client = AuthClient::new(channel);

    let req = AuthenticateRequest {
        name: name.to_string(),
        password: password.to_string(),
    };

    let resp = auth_client.authenticate(req).await?.into_inner();

    Ok(resp.token)
}
