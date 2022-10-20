use std::sync::{Arc, RwLock};

use crate::error::{ErrKind, Error, Result};
use crate::interceptor::CredentialInterceptor;
use crate::pb;

use crate::auth::AuthClient;
use crate::kv::KvClient;
use crate::lease::{LeaseClient, LeaseKeepAliver};
use crate::watch::{WatchClient, Watcher};

use http::Uri;
use tonic::codegen::InterceptedService;
use tonic::transport::{channel::Channel, Endpoint};

pub(crate) type Transport =
    InterceptedService<tonic::transport::channel::Channel, CredentialInterceptor>;

pub struct Client {
    pub kv: KvClient,
    pub auth: AuthClient,
    pub watch: WatchClient,
    pub lease: LeaseClient,

    pub(crate) transport: Transport,

    credential: Option<(String, String)>,
    token: Arc<RwLock<String>>,
}

impl Client {
    /// Create a new Client
    pub async fn new(
        endpoints: Vec<impl AsRef<str>>,
        credential: Option<(String, String)>,
    ) -> Result<Self> {
        let mut ep_uris = Vec::new();

        // check endpoints
        for ep in &endpoints {
            let uri: Uri = ep
                .as_ref()
                .parse()
                .map_err(|err| Error::new(ErrKind::Endpoint, err))?;
            if uri.scheme().is_none() {
                return Err(Error::new(ErrKind::Endpoint, "endpoint scheme is empty"));
            }
            ep_uris.push(uri);
        }

        // try to get token
        let token = if let Some((name, password)) = &credential {
            get_token(&ep_uris, name, password).await?
        } else {
            String::new()
        };

        let channel = new_channel(ep_uris).await?;
        let token = Arc::new(RwLock::new(token));

        let transport = InterceptedService::new(channel, CredentialInterceptor::new(token.clone()));

        Ok(Client {
            auth: AuthClient::new(transport.clone()),
            kv: KvClient::new(transport.clone()),
            watch: WatchClient::new(transport.clone()),
            lease: LeaseClient::new(transport.clone()),
            transport,
            credential,
            token,
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

    /// Refresh token
    pub async fn refresh_token(&mut self) -> Result<()> {
        if let Some((username, password)) = &self.credential {
            let token = self.auth.get_token(username, password).await?;

            *self.token.write().unwrap() = token;
        }

        Ok(())
    }
}

async fn get_token(endpoints: &[Uri], name: &str, password: &str) -> Result<String> {
    for ep in endpoints {
        let channel = connect_to(ep.to_owned()).await?;
        let transport = InterceptedService::new(channel, CredentialInterceptor::empty());

        let mut auth_client = AuthClient::new(transport);

        match auth_client.get_token(name, password).await {
            Ok(token) => {
                return Ok(token);
            }
            Err(err) => {
                if err.kind() == ErrKind::AuthNotEnabled {
                    return Ok(String::new());
                }
                // FIXME: when error, just let it go?
            }
        }
    }

    Err(Error::new(ErrKind::AuthFailed, ""))
}

async fn new_channel(endpoints: Vec<Uri>) -> Result<Channel> {
    let mut eps: Vec<Endpoint> = Vec::new();

    for ep in endpoints {
        eps.push(Channel::builder(ep));
    }

    match eps.len() {
        0 => Err(Error::new(ErrKind::Endpoint, "endpoint uri empty")),
        1 => eps[0]
            .connect()
            .await
            .map_err(|err| Error::new(ErrKind::ConnectFailed, err)),
        _ => Ok(Channel::balance_list(eps.into_iter())),
    }
}

async fn connect_to(endpoint: Uri) -> Result<Channel> {
    Channel::builder(endpoint)
        .connect()
        .await
        .map_err(|err| Error::new(ErrKind::ConnectFailed, err))
}
