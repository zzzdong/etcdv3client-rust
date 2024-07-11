use http::uri::PathAndQuery;
use std::future::Future;
use tonic::metadata::AsciiMetadataValue;

use crate::{auth::InnerAuthClient, error::Result, utils::TOKEN_FIELD_NAME};

pub trait GrpcService: Send + Clone + std::fmt::Debug {
    fn unary<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> impl Future<Output = Result<tonic::Response<T>>> + Send
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;

    fn client_streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> impl Future<Output = Result<tonic::Response<T>>> + Send
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;

    fn server_streaming<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> impl Future<Output = Result<tonic::Response<tonic::Streaming<T>>>> + Send
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;

    fn streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> impl Future<Output = Result<tonic::Response<tonic::Streaming<T>>>> + Send
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;
}

#[derive(Debug, Clone)]
pub struct TonicClient {
    inner: tonic::client::Grpc<tonic::transport::Channel>,
}

impl TonicClient {
    pub fn new(channel: tonic::transport::Channel) -> Self {
        Self {
            inner: tonic::client::Grpc::new(channel),
        }
    }

    pub async fn unary<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.inner.ready().await.map_err(|e| {
            crate::Error::new(
                crate::ErrKind::Grpc,
                format!("Service was not ready: {}", e),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();

        self.inner.unary(req, path, codec).await.map_err(Into::into)
    }

    pub async fn client_streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.inner.ready().await.map_err(|e| {
            crate::Error::new(
                crate::ErrKind::Grpc,
                format!("Service was not ready: {}", e),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();

        self.inner
            .client_streaming(req, path, codec)
            .await
            .map_err(Into::into)
    }

    pub async fn server_streaming<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.inner.ready().await.map_err(|e| {
            crate::Error::new(
                crate::ErrKind::Grpc,
                format!("Service was not ready: {}", e),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();

        self.inner
            .server_streaming(req, path, codec)
            .await
            .map_err(Into::into)
    }

    pub async fn streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.inner.ready().await.map_err(|e| {
            crate::Error::new(
                crate::ErrKind::Grpc,
                format!("Service was not ready: {}", e),
            )
        })?;
        let codec = tonic::codec::ProstCodec::default();

        self.inner
            .streaming(req, path, codec)
            .await
            .map_err(Into::into)
    }
}

impl GrpcService for TonicClient {
    async fn unary<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.unary(req, path).await
    }

    async fn client_streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.client_streaming(req, path).await
    }

    async fn server_streaming<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.server_streaming(req, path).await
    }

    async fn streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.streaming(req, path).await
    }
}

#[derive(Debug, Clone)]
pub struct CredentialInterceptor<C> {
    credential: Option<(String, String)>,
    token: Option<AsciiMetadataValue>,
    inner: C,
}

impl<C> CredentialInterceptor<C>
where
    C: GrpcService + Clone,
{
    pub fn new(
        credential: impl Into<Option<(String, String)>>,
        token: impl Into<Option<AsciiMetadataValue>>,
        inner: C,
    ) -> Self {
        Self {
            credential: credential.into(),
            token: token.into(),
            inner,
        }
    }

    pub async fn unary<M, T>(
        &mut self,
        mut req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.insert_token(&mut req);

        let (req, mut req_cloned) = Self::clone_request(req);

        match self.inner.unary(req, path.clone()).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                if err.is_auth_not_enabled() {
                    tracing::warn!("auth not enabled, retry with remove auth token.");
                    self.token.take();
                    self.inner.unary(req_cloned, path).await
                } else if err.should_refresh_token() {
                    tracing::debug!(?err, "refreshing token");
                    self.refresh_token().await?;
                    self.insert_token(&mut req_cloned);

                    self.inner.unary(req_cloned, path).await
                } else {
                    Err(err)
                }
            }
        }
    }

    pub async fn client_streaming<S, M, T>(
        &mut self,
        mut req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        // when streaming, always refresh token.
        self.refresh_token().await?;
        self.insert_token(&mut req);

        self.inner.client_streaming(req, path).await
    }

    pub async fn server_streaming<M, T>(
        &mut self,
        mut req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        self.insert_token(&mut req);
        let (req, mut req_cloned) = Self::clone_request(req);

        match self.inner.server_streaming(req, path.clone()).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                if self.should_refresh_token(&err) {
                    self.refresh_token().await?;
                    self.insert_token(&mut req_cloned);

                    self.inner.server_streaming(req_cloned, path).await
                } else {
                    Err(err)
                }
            }
        }
    }

    pub async fn streaming<S, M, T>(
        &mut self,
        mut req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        // when streaming, always refresh token.
        self.refresh_token().await?;
        self.insert_token(&mut req);

        self.inner.streaming(req, path).await
    }

    /// Clone request.
    fn clone_request<M: Clone>(req: tonic::Request<M>) -> (tonic::Request<M>, tonic::Request<M>) {
        let (metadata, extensions, message) = req.into_parts();

        let req_cloned =
            tonic::Request::from_parts(metadata.clone(), extensions.clone(), message.clone());
        let req = tonic::Request::from_parts(metadata, extensions, message);

        (req, req_cloned)
    }

    fn should_refresh_token(&self, err: &crate::Error) -> bool {
        if self.credential.is_none() {
            return false;
        }

        err.should_refresh_token()
    }

    fn insert_token<M>(&self, req: &mut tonic::Request<M>) {
        if let Some(ref token) = self.token {
            req.metadata_mut().insert(TOKEN_FIELD_NAME, token.clone());
        }
    }

    async fn refresh_token(&mut self) -> Result<()> {
        if let Some(ref cred) = self.credential {
            let span = tracing::span!(tracing::Level::TRACE, "refresh_token");
            let _ = span.enter();

            match InnerAuthClient::new(self.inner.clone())
                .get_token(&cred.0, &cred.1)
                .await
            {
                Ok(token) => {
                    self.token = AsciiMetadataValue::try_from(token)
                        .map_err(|err| crate::Error::new(crate::ErrKind::AuthFailed, err))?
                        .into();
                }
                Err(err) if err.is_auth_not_enabled() => {
                    tracing::warn!("auth not enabled, remove auth token.");
                    self.token.take();
                }
                Err(err) => {
                    tracing::error!("get_token failed: {err:?}");
                    return Err(err);
                }
            }
        }

        Ok(())
    }
}

impl<C> GrpcService for CredentialInterceptor<C>
where
    C: GrpcService + Send + Sync + Clone,
{
    async fn unary<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        CredentialInterceptor::unary(self, req, path).await
    }

    async fn client_streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        CredentialInterceptor::client_streaming(self, req, path).await
    }

    async fn server_streaming<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        CredentialInterceptor::server_streaming(self, req, path).await
    }

    async fn streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static,
    {
        CredentialInterceptor::streaming(self, req, path).await
    }
}
