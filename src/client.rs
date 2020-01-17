use crate::error::{EtcdClientError, Result};
use crate::pb;

use crate::auth::AuthClient;
use crate::kv::KvClient;
use crate::watch::{WatchClient, Watcher};

use tonic::transport::{channel::Channel, Endpoint};
use tonic::{metadata::MetadataValue, Request};

pub(crate) const TOKEN_ID: &str = "token";

pub struct EtcdClient {
    pub kv: KvClient,
    pub auth: AuthClient,
    pub watch: WatchClient,

    pub(crate) channel: Channel,
    pub(crate) interceptor: Option<tonic::Interceptor>,
}

impl EtcdClient {
    /// Create a new EtcdClient
    pub async fn new(
        endpoints: Vec<impl AsRef<str>>,
        auth: Option<(String, String)>,
    ) -> Result<Self> {
        let mut token = None;

        // try to get token
        if let Some((name, password)) = auth {
            for endpoint in &endpoints {
                if let Ok(t) = get_token(endpoint, &name, &password).await {
                    token = Some(t);
                    break;
                }
            }
            // after try all, report error when not got.
            if token.is_none() {
                return Err(EtcdClientError::ErrMsg("can not get token".to_string()));
            }
        }

        let channel = new_channel(endpoints).await?;

        let mut interceptor = None;
        if let Some(t) = token {
            let token = MetadataValue::from_str(&t).unwrap();
            interceptor = Some(
                {
                    move |mut req: Request<()>| {
                        req.metadata_mut().insert(TOKEN_ID, token.clone());

                        Ok(req)
                    }
                }
                .into(),
            )
        }

        Ok(EtcdClient {
            auth: AuthClient::new(channel.clone(), interceptor.clone()),
            kv: KvClient::new(channel.clone(), interceptor.clone()),
            watch: WatchClient::new(channel.clone(), interceptor.clone()),
            channel,
            interceptor,
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
        self.watch.watch(key).await
    }
}

async fn get_token(endpoint: impl AsRef<str>, name: &str, password: &str) -> Result<String> {
    let channel = new_channel(vec![endpoint]).await?;

    let mut auth_client = AuthClient::new(channel, None);

    let req = crate::pb::AuthenticateRequest {
        name: name.to_string(),
        password: password.to_string(),
    };

    let resp = auth_client.do_authenticate(req).await?;

    Ok(resp.token)
}

async fn new_channel(endpoints: Vec<impl AsRef<str>>) -> Result<Channel> {
    let mut eps: Vec<Endpoint> = Vec::new();

    for ep in endpoints {
        eps.push(Channel::builder(ep.as_ref().parse()?));
    }

    match eps.len() {
        0 => Err(EtcdClientError::ErrMsg("endpoint uri empty".to_string())),
        1 => eps[0].connect().await.map_err(EtcdClientError::from),
        _ => Ok(Channel::balance_list(eps.into_iter())),
    }
}
