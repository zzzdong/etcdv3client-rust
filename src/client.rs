use crate::error::{EtcdClientError, Result};
use crate::pb;

use crate::auth::AuthClient;
use crate::kv::KvClient;
use crate::lease::{LeaseClient, LeaseKeepAliver};
use crate::watch::{WatchClient, Watcher};

use http::Uri;
use tonic::transport::{channel::Channel, Endpoint};

pub struct EtcdClient {
    pub kv: KvClient,
    pub auth: AuthClient,
    pub watch: WatchClient,
    pub lease: LeaseClient,

    pub(crate) channel: Channel,
}

impl EtcdClient {
    /// Create a new EtcdClient
    pub async fn new(
        endpoints: Vec<impl AsRef<str>>,
        auth: Option<(String, String)>,
    ) -> Result<Self> {
        let mut ep_uris = Vec::new();

        // check endpoints
        for ep in &endpoints {
            let uri: Uri = ep.as_ref().parse()?;
            if uri.scheme().is_none() {
                return Err(EtcdClientError::EndpointError);
            }
            ep_uris.push(uri);
        }

        let mut token = None;

        // try to get token
        if let Some((name, password)) = auth {
            for ep in &ep_uris {
                if let Ok(t) = get_token(ep.clone(), &name, &password).await {
                    token = Some(t);
                    break;
                }
            }
            // after try all, report error when not got.
            if token.is_none() {
                return Err(EtcdClientError::ErrMsg("can not get token".to_string()));
            }
        }

        let channel = new_channel(ep_uris).await?;

        Ok(EtcdClient {
            auth: AuthClient::new(channel.clone()),
            kv: KvClient::new(channel.clone()),
            watch: WatchClient::new(channel.clone()),
            lease: LeaseClient::new(channel.clone()),
            channel,
        })
    }

    /// Get value by key
    #[inline]
    pub async fn get(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        self.kv.get(key).await
    }

    /// Get string by key
    #[inline]
    pub async fn get_string(&mut self, key: impl AsRef<[u8]>) -> Result<String> {
        self.kv.get_string(key).await
    }

    /// Get key-value pairs with prefix
    #[inline]
    pub async fn get_with_prefix(&mut self, key: impl AsRef<[u8]>) -> Result<Vec<pb::KeyValue>> {
        self.kv.get_with_prefix(key).await
    }

    /// Get all key-value pairs
    #[inline]
    pub async fn all(&mut self) -> Result<Vec<pb::KeyValue>> {
        self.kv.all().await
    }

    /// Put a key-value pair
    #[inline]
    pub async fn put(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>) -> Result<()> {
        self.kv.put_kv(key, value).await
    }

    /// Delete a key-value pair
    #[inline]
    pub async fn delete(&mut self, key: impl AsRef<[u8]>) -> Result<()> {
        self.kv.delete(key).await
    }

    /// Watch a key
    pub async fn watch(&mut self, key: impl AsRef<[u8]>) -> Result<Watcher> {
        self.watch.watch_key(key).await
    }

    /// Grant a lease
    pub async fn grant_lease(&mut self, ttl: i64, lease_id: i64) -> Result<pb::LeaseGrantResponse> {
        self.lease.grant(ttl, lease_id).await
    }

    /// Revoke a lease
    pub async fn revoke_lease(&mut self, lease_id: i64) -> Result<()> {
        self.lease.revoke(lease_id).await
    }

    /// Create LeaseKeepAliver to keep a lease alive
    pub async fn keep_lease_alive(&mut self, lease_id: i64) -> Result<LeaseKeepAliver> {
        self.lease.keep_alive(lease_id).await
    }

    /// Get a lease info
    pub async fn get_lease_info(
        &mut self,
        lease_id: i64,
        keys: bool,
    ) -> Result<pb::LeaseTimeToLiveResponse> {
        self.lease.get_lease_info(lease_id, keys).await
    }

    /// List all leases
    pub async fn list_leases(&mut self) -> Result<pb::LeaseLeasesResponse> {
        self.lease.list().await
    }
}

async fn get_token(endpoint: Uri, name: &str, password: &str) -> Result<String> {
    let channel = new_channel(vec![endpoint]).await?;

    let mut auth_client = AuthClient::new(channel);

    auth_client.get_token(name, password).await
}

async fn new_channel(endpoints: Vec<Uri>) -> Result<Channel> {
    let mut eps: Vec<Endpoint> = Vec::new();

    for ep in endpoints {
        eps.push(Channel::builder(ep));
    }

    match eps.len() {
        0 => Err(EtcdClientError::ErrMsg("endpoint uri empty".to_string())),
        1 => eps[0].connect().await.map_err(EtcdClientError::from),
        _ => Ok(Channel::balance_list(eps.into_iter())),
    }
}
