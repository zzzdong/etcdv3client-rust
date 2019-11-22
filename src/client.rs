use crate::error::EtcdClientError;
use crate::pb::etcdserverpb::client::*;

use crate::pb::etcdserverpb::*;
use crate::KeyValue;

use tonic::transport::channel::Channel;

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
        cred: Option<(String, String)>,
    ) -> Result<Self, EtcdClientError> {
        let mut token = None;

        if let Some((name, password)) = cred {
            for endpoint in &endpoints {
                if let Ok(t) = get_token(endpoint, &name, &password).await {
                    token = Some(t);
                    break;
                }
            }
        }

        let channel = crate::conn::new_channel(endpoints, token).await?;

        Ok(EtcdClient {
            kv: KvClient::new(channel.clone()),
            watch: WatchClient::new(channel.clone()),
            lease: LeaseClient::new(channel.clone()),
            cluster: ClusterClient::new(channel.clone()),
            maintenance: MaintenanceClient::new(channel.clone()),
            auth: AuthClient::new(channel.clone()),
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
    let channel = crate::conn::new_channel(vec![endpoint], Option::<&str>::None).await?;

    let mut auth_client = AuthClient::new(channel);

    let req = AuthenticateRequest {
        name: name.to_string(),
        password: password.to_string(),
    };

    let resp = auth_client.authenticate(req).await?.into_inner();

    Ok(resp.token)
}
