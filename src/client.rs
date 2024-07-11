use crate::auth::{AuthClient, InnerAuthClient};
use crate::error::{ErrKind, Error, Result};
use crate::grpc::{CredentialInterceptor, GrpcService, TonicClient};
use crate::kv::KvClient;
use crate::lease::{LeaseClient, LeaseKeepAliver};
use crate::pb;
use crate::watch::{WatchClient, Watcher};

use http::Uri;
use tonic::transport::{channel::Channel, Endpoint};

pub type EtcdClient = Client<CredentialInterceptor<TonicClient>>;

#[derive(Debug, Clone)]
pub struct Client<S> {
    pub kv: KvClient<S>,
    pub auth: AuthClient<S>,
    pub watch: WatchClient<S>,
    pub lease: LeaseClient<S>,

    pub(crate) service: S,
}

impl Client<CredentialInterceptor<TonicClient>> {
    /// Create a new Client
    pub async fn new<U>(
        endpoints: impl Into<Vec<U>>,
        credential: impl Into<Option<(String, String)>>,
    ) -> Result<Self>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: std::error::Error + std::marker::Send + Sync + 'static,
    {
        let mut ep_uris = Vec::new();

        // check endpoints
        let endpoints = endpoints.into();
        for ep in endpoints {
            let uri = Uri::try_from(ep).map_err(|err| Error::new(ErrKind::Endpoint, err))?;
            if uri.scheme().is_none() {
                return Err(Error::new(ErrKind::Endpoint, "endpoint scheme is empty"));
            }
            ep_uris.push(uri);
        }

        // try to get token
        let credential = credential.into();
        let token = match &credential {
            Some((name, password)) => {
                let token = get_token(&ep_uris, name, password).await?;
                let token = token
                    .try_into()
                    .map_err(|err| Error::new(ErrKind::AuthFailed, err))?;
                Some(token)
            }
            None => None,
        };

        let channel = new_channel(ep_uris).await?;
        let service = CredentialInterceptor::new(credential, token, TonicClient::new(channel));

        Ok(Client::with_service(service))
    }
}

impl<S: GrpcService> Client<S> {
    pub fn service(&self) -> S {
        self.service.clone()
    }

    pub fn with_service(service: S) -> Self {
        Client {
            auth: AuthClient::new(service.clone()),
            kv: KvClient::new(service.clone()),
            watch: WatchClient::new(service.clone()),
            lease: LeaseClient::new(service.clone()),
            service,
        }
    }

    /// Get value by key
    #[inline]
    pub async fn get(&mut self, key: impl Into<Vec<u8>>) -> Result<Vec<u8>> {
        self.kv.get(key).await
    }

    /// Get string by key
    #[inline]
    pub async fn get_string(&mut self, key: impl Into<Vec<u8>>) -> Result<String> {
        self.kv.get_string(key).await
    }

    /// Get key-value pairs with prefix
    #[inline]
    pub async fn get_with_prefix(&mut self, key: impl Into<Vec<u8>>) -> Result<Vec<pb::KeyValue>> {
        self.kv.get_with_prefix(key).await
    }

    /// Get all key-value pairs
    #[inline]
    pub async fn all(&mut self) -> Result<Vec<pb::KeyValue>> {
        self.kv.all().await
    }

    /// Put a key-value pair
    #[inline]
    pub async fn put(&mut self, key: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Result<()> {
        self.kv.put_kv(key, value).await
    }

    /// Delete a key-value pair
    #[inline]
    pub async fn delete(&mut self, key: impl Into<Vec<u8>>) -> Result<()> {
        self.kv.delete(key).await
    }

    /// Watch a key
    pub async fn watch(&mut self, key: impl Into<Vec<u8>>) -> Result<Watcher> {
        self.watch.watch_key(key).await
    }

    /// Grant a lease
    pub async fn grant_lease(&mut self, ttl: i64) -> Result<pb::LeaseGrantResponse> {
        self.lease.grant(ttl).await
    }

    /// Grant a lease with lease id
    pub async fn grant_with_lease_id(
        &mut self,
        ttl: i64,
        lease_id: i64,
    ) -> Result<pb::LeaseGrantResponse> {
        self.lease.grant_with_lease_id(ttl, lease_id).await
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

    // /// Refresh token
    // pub async fn refresh_token(&mut self) -> Result<()> {
    //     if let Some((username, password)) = &self.credential {
    //         let token = self.auth.get_token(username, password).await?;

    //         *self.token.write().unwrap() = token;
    //     }

    //     Ok(())
    // }
}



async fn get_token(endpoints: &[Uri], name: &str, password: &str) -> Result<String> {
    for ep in endpoints {
        let channel = connect_to(ep.to_owned()).await?;

        let mut auth_client = InnerAuthClient::new(TonicClient::new(channel));

        match auth_client
            .get_token(name.to_string(), password.to_string())
            .await
        {
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

    Err(Error::new(ErrKind::AuthFailed, "all endpoints failed"))
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
