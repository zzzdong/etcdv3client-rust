use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use arc_swap::ArcSwapOption;
use http::{HeaderValue, Request, Response};
use tonic::body::BoxBody;
use tonic::transport::Body;
use tonic::transport::Channel;
use tower::Service;

use crate::{pb::auth_client::AuthClient, EtcdClientError};

pub(crate) const TOKEN_ID: &str = "token";

#[derive(Clone)]
pub struct EtcdSvc {
    inner: Channel,
    auth: Arc<Option<(String, String)>>,
    token: Arc<ArcSwapOption<String>>,
}

impl EtcdSvc {
    pub fn new(inner: Channel, auth: Arc<Option<(String, String)>>) -> Self {
        EtcdSvc {
            inner,
            auth,
            token: Arc::new(ArcSwapOption::empty()),
        }
    }

    pub async fn fetch_token(
        channel: Channel,
        auth: Arc<Option<(String, String)>>,
        token: Arc<ArcSwapOption<String>>,
    ) -> crate::error::Result<()> {
        if auth.is_some() && token.load().is_none() {
            if let Some(ref auth) = *auth {
                let mut auth_client = AuthClient::new(channel);

                match auth_client
                    .authenticate(crate::pb::AuthenticateRequest {
                        name: auth.0.to_string(),
                        password: auth.1.to_string(),
                    })
                    .await
                {
                    Ok(resp) => token.store(Some(Arc::new(resp.into_inner().token))),
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn refresh_token(&self) -> crate::error::Result<()> {
        Self::fetch_token(self.inner.clone(), self.auth.clone(), self.token.clone()).await
    }
}

#[allow(clippy::type_complexity)]
impl Service<Request<BoxBody>> for EtcdSvc {
    type Response = Response<Body>;
    type Error = EtcdClientError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: Request<BoxBody>) -> Self::Future {
        let channel = self.inner.clone();
        let auth = self.auth.clone();
        let token = self.token.clone();

        let mut inner = std::mem::replace(&mut self.inner, channel);

        Box::pin(async move {
            if let Err(e) = EtcdSvc::fetch_token(inner.clone(), auth, token.clone()).await {
                tracing::warn!("refresh token failed, err: {:?}", e);
            }

            let t = token.load();
            if let Some(ref t) = *t {
                req.headers_mut()
                    .insert(TOKEN_ID, HeaderValue::from_bytes(t.as_bytes()).unwrap());
            }

            let fut = inner.call(req).await.map_err(Into::into);

            if let Ok(ref resp) = fut {
                if let Some(status) = tonic::Status::from_header_map(resp.headers()) {
                    if status.code() == tonic::Code::Unauthenticated {
                        token.store(None);
                    }
                }
            }

            fut
        })
    }
}
