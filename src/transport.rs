use http::uri::PathAndQuery;
use tonic::{async_trait, metadata::AsciiMetadataValue};

use crate::{auth::InnerAuthClient, error::Result, Error};

const TOKEN_FIELD_NAME: &str = "token";

pub(crate) type Transport = CredentialInterceptor<GrpcClient>;

#[async_trait]
pub trait GrpcService {
    async fn unary<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;

    async fn client_streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<T>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;

    async fn server_streaming<M, T>(
        &mut self,
        req: tonic::Request<M>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;

    async fn streaming<S, M, T>(
        &mut self,
        req: tonic::Request<S>,
        path: PathAndQuery,
    ) -> Result<tonic::Response<tonic::Streaming<T>>>
    where
        S: futures::Stream<Item = M> + Send + 'static,
        M: prost::Message + Clone + Send + Sync + 'static,
        T: prost::Message + Default + Send + Sync + 'static;
}

#[derive(Debug, Clone)]
pub struct GrpcClient {
    inner: tonic::client::Grpc<tonic::transport::Channel>,
}

impl GrpcClient {
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

#[async_trait]
impl GrpcService for GrpcClient {
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
pub(crate) struct CredentialInterceptor<C> {
    credential: (String, String),
    token: AsciiMetadataValue,
    inner: C,
}

impl<C> CredentialInterceptor<C>
where
    C: GrpcService + Clone,
{
    pub fn new(credential: (String, String), token: &str, inner: C) -> Self {
        Self {
            credential,
            token: AsciiMetadataValue::try_from(token).unwrap(),
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

        let (mut req, req_cloned) = Self::clone_request(req);

        match self.inner.unary(req_cloned, path.clone()).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                if err.should_refresh_token() {
                    self.refresh_token().await?;
                    self.insert_token(&mut req);

                    self.inner.unary(req, path).await
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
        let (mut req, req_cloned) = Self::clone_request(req);

        match self.inner.server_streaming(req_cloned, path.clone()).await {
            Ok(resp) => Ok(resp),
            Err(err) => {
                if self.should_refresh_token(&err) {
                    self.refresh_token().await?;
                    self.insert_token(&mut req);

                    self.inner.server_streaming(req, path).await
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

    fn clone_request<M: Clone>(req: tonic::Request<M>) -> (tonic::Request<M>, tonic::Request<M>) {
        let (metadata, _extensions, message) = req.into_parts();

        let mut req_cloned = tonic::Request::new(message.clone());
        *req_cloned.metadata_mut() = metadata.clone();

        let mut req = tonic::Request::new(message);
        *req.metadata_mut() = metadata;

        (req, req_cloned)
    }

    fn should_refresh_token(&self, err: &Error) -> bool {
        if self.credential.0.is_empty() || self.credential.1.is_empty() {
            return false;
        }

        err.should_refresh_token()
    }

    fn insert_token<M>(&self, req: &mut tonic::Request<M>) {
        if !self.token.is_empty() {
            req.metadata_mut()
                .insert(TOKEN_FIELD_NAME, self.token.clone());
        }
    }

    async fn refresh_token(&mut self) -> Result<()> {
        if !self.credential.0.is_empty() {
            let token = InnerAuthClient::new(self.inner.clone())
                .get_token(&self.credential.0, &self.credential.1)
                .await?;

            self.token =
                AsciiMetadataValue::try_from(token).expect("token to AsciiMetadataValue failed");
        }

        Ok(())
    }
}

#[async_trait]
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
