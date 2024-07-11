use crate::error::Result;
use crate::grpc::GrpcService;
use crate::pb;

use tonic::IntoRequest;

#[derive(Debug, Clone)]
pub struct InnerAuthClient<S> {
    service: S,
}
impl<S> InnerAuthClient<S>
where
    S: GrpcService,
{
    pub fn new(service: S) -> Self {
        Self { service }
    }
    pub async fn auth_enable(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthEnableRequest>,
    ) -> Result<tonic::Response<pb::AuthEnableResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthEnable");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn auth_disable(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthDisableRequest>,
    ) -> Result<tonic::Response<pb::AuthDisableResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthDisable");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn auth_status(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthStatusRequest>,
    ) -> Result<tonic::Response<pb::AuthStatusResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthStatus");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn authenticate(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthenticateRequest>,
    ) -> Result<tonic::Response<pb::AuthenticateResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/Authenticate");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_add(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserAddRequest>,
    ) -> Result<tonic::Response<pb::AuthUserAddResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserAdd");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_get(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserGetRequest>,
    ) -> Result<tonic::Response<pb::AuthUserGetResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserGet");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_list(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserListRequest>,
    ) -> Result<tonic::Response<pb::AuthUserListResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserList");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_delete(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserDeleteRequest>,
    ) -> Result<tonic::Response<pb::AuthUserDeleteResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserDelete");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_change_password(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserChangePasswordRequest>,
    ) -> Result<tonic::Response<pb::AuthUserChangePasswordResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserChangePassword");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_grant_role(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserGrantRoleRequest>,
    ) -> Result<tonic::Response<pb::AuthUserGrantRoleResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserGrantRole");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn user_revoke_role(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserRevokeRoleRequest>,
    ) -> Result<tonic::Response<pb::AuthUserRevokeRoleResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserRevokeRole");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn role_add(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleAddRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleAddResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleAdd");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn role_get(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleGetRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleGetResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleGet");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn role_list(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleListRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleListResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleList");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn role_delete(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleDeleteRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleDeleteResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleDelete");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn role_grant_permission(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleGrantPermissionRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleGrantPermissionResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleGrantPermission");
        self.service.unary(request.into_request(), path).await
    }
    pub async fn role_revoke_permission(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleRevokePermissionRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleRevokePermissionResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleRevokePermission");
        self.service.unary(request.into_request(), path).await
    }

    pub async fn get_token(
        &mut self,
        name: impl ToString,
        password: impl ToString,
    ) -> Result<String> {
        self.authenticate(pb::AuthenticateRequest {
            name: name.to_string(),
            password: password.to_string(),
        })
        .await
        .map(|resp| resp.into_inner().token)
    }
}
#[derive(Debug, Clone)]
pub struct AuthClient<S> {
    inner: InnerAuthClient<S>,
}
impl<S> AuthClient<S>
where
    S: GrpcService,
{
    pub async fn auth_enable(
        &mut self,
        request: pb::AuthEnableRequest,
    ) -> Result<pb::AuthEnableResponse> {
        self.inner
            .auth_enable(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn auth_disable(
        &mut self,
        request: pb::AuthDisableRequest,
    ) -> Result<pb::AuthDisableResponse> {
        self.inner
            .auth_disable(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn auth_status(
        &mut self,
        request: pb::AuthStatusRequest,
    ) -> Result<pb::AuthStatusResponse> {
        self.inner
            .auth_status(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn authenticate(
        &mut self,
        request: pb::AuthenticateRequest,
    ) -> Result<pb::AuthenticateResponse> {
        self.inner
            .authenticate(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_add(
        &mut self,
        request: pb::AuthUserAddRequest,
    ) -> Result<pb::AuthUserAddResponse> {
        self.inner
            .user_add(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_get(
        &mut self,
        request: pb::AuthUserGetRequest,
    ) -> Result<pb::AuthUserGetResponse> {
        self.inner
            .user_get(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_list(
        &mut self,
        request: pb::AuthUserListRequest,
    ) -> Result<pb::AuthUserListResponse> {
        self.inner
            .user_list(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_delete(
        &mut self,
        request: pb::AuthUserDeleteRequest,
    ) -> Result<pb::AuthUserDeleteResponse> {
        self.inner
            .user_delete(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_change_password(
        &mut self,
        request: pb::AuthUserChangePasswordRequest,
    ) -> Result<pb::AuthUserChangePasswordResponse> {
        self.inner
            .user_change_password(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_grant_role(
        &mut self,
        request: pb::AuthUserGrantRoleRequest,
    ) -> Result<pb::AuthUserGrantRoleResponse> {
        self.inner
            .user_grant_role(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn user_revoke_role(
        &mut self,
        request: pb::AuthUserRevokeRoleRequest,
    ) -> Result<pb::AuthUserRevokeRoleResponse> {
        self.inner
            .user_revoke_role(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn role_add(
        &mut self,
        request: pb::AuthRoleAddRequest,
    ) -> Result<pb::AuthRoleAddResponse> {
        self.inner
            .role_add(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn role_get(
        &mut self,
        request: pb::AuthRoleGetRequest,
    ) -> Result<pb::AuthRoleGetResponse> {
        self.inner
            .role_get(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn role_list(
        &mut self,
        request: pb::AuthRoleListRequest,
    ) -> Result<pb::AuthRoleListResponse> {
        self.inner
            .role_list(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn role_delete(
        &mut self,
        request: pb::AuthRoleDeleteRequest,
    ) -> Result<pb::AuthRoleDeleteResponse> {
        self.inner
            .role_delete(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn role_grant_permission(
        &mut self,
        request: pb::AuthRoleGrantPermissionRequest,
    ) -> Result<pb::AuthRoleGrantPermissionResponse> {
        self.inner
            .role_grant_permission(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
    pub async fn role_revoke_permission(
        &mut self,
        request: pb::AuthRoleRevokePermissionRequest,
    ) -> Result<pb::AuthRoleRevokePermissionResponse> {
        self.inner
            .role_revoke_permission(request.into_request())
            .await
            .map(|rsp| rsp.into_inner())
    }
}

impl<S> AuthClient<S>
where
    S: GrpcService,
{
    pub(crate) fn new(service: S) -> Self {
        AuthClient {
            inner: InnerAuthClient::new(service),
        }
    }

    pub fn do_auth_enable(&mut self) -> DoAuthEnableRequest<S> {
        pb::AuthEnableRequest::default().build(self)
    }

    pub async fn enable_auth(&mut self) -> Result<()> {
        self.do_auth_enable().await.map(|_| ())
    }

    pub fn do_auth_disable(&mut self) -> DoAuthDisableRequest<S> {
        pb::AuthDisableRequest::default().build(self)
    }

    pub async fn disable_auth(&mut self) -> Result<()> {
        self.do_auth_disable().await.map(|_| ())
    }

    pub fn do_authenticate(
        &mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> DoAuthenticateRequest<'_, S> {
        pb::AuthenticateRequest::new(name.into(), password.into()).build(self)
    }

    pub async fn get_token(
        &mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<String> {
        let resp = self.do_authenticate(name, password).await?;
        Ok(resp.token)
    }

    pub fn do_user_add(
        &mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> DoAuthUserAddRequest<'_, S> {
        pb::AuthUserAddRequest::new(name.into(), password.into()).build(self)
    }

    pub async fn add_user(
        &mut self,
        name: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<()> {
        let _resp = self.do_user_add(name, password).await?;
        Ok(())
    }

    pub fn do_user_get(&mut self, name: impl Into<String>) -> DoAuthUserGetRequest<S> {
        pb::AuthUserGetRequest::new(name.into()).build(self)
    }

    pub async fn get_user(&mut self, name: impl Into<String>) -> Result<pb::AuthUserGetResponse> {
        self.do_user_get(name).await
    }
}

impl pb::AuthEnableRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthEnableRequest<'_, S> {
        DoAuthEnableRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthEnableRequest<'a, S> {
    pub request: pb::AuthEnableRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthEnableRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthEnableRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthEnableResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthEnableResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthEnableRequest { request, client } = self;
        Box::pin(async move { client.auth_enable(request).await })
    }
}
impl pb::AuthDisableRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthDisableRequest<'_, S> {
        DoAuthDisableRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthDisableRequest<'a, S> {
    pub request: pb::AuthDisableRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthDisableRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthDisableRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthDisableResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthDisableResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthDisableRequest { request, client } = self;
        Box::pin(async move { client.auth_disable(request).await })
    }
}
impl pb::AuthStatusRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthStatusRequest<'_, S> {
        DoAuthStatusRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthStatusRequest<'a, S> {
    pub request: pb::AuthStatusRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthStatusRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthStatusRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthStatusResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthStatusResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthStatusRequest { request, client } = self;
        Box::pin(async move { client.auth_status(request).await })
    }
}
impl pb::AuthenticateRequest {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }

    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthenticateRequest<'_, S> {
        DoAuthenticateRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthenticateRequest<'a, S> {
    pub request: pb::AuthenticateRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthenticateRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
    pub fn with_password(mut self, password: String) -> Self {
        self.request.password = password;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthenticateRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthenticateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthenticateResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthenticateRequest { request, client } = self;
        Box::pin(async move { client.authenticate(request).await })
    }
}
impl pb::AuthUserAddRequest {
    pub fn new(name: String, password: String) -> Self {
        Self {
            name,
            password,
            ..Default::default()
        }
    }

    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthUserAddRequest<'_, S> {
        DoAuthUserAddRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserAddRequest<'a, S> {
    pub request: pb::AuthUserAddRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserAddRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
    pub fn with_password(mut self, password: String) -> Self {
        self.request.password = password;
        self
    }
    pub fn with_hashed_password(mut self, hashed_password: String) -> Self {
        self.request.hashed_password = hashed_password;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserAddRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserAddResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserAddResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserAddRequest { request, client } = self;
        Box::pin(async move { client.user_add(request).await })
    }
}
impl pb::AuthUserGetRequest {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthUserGetRequest<'_, S> {
        DoAuthUserGetRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserGetRequest<'a, S> {
    pub request: pb::AuthUserGetRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserGetRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserGetRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserGetResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserGetResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserGetRequest { request, client } = self;
        Box::pin(async move { client.user_get(request).await })
    }
}
impl pb::AuthUserListRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthUserListRequest<'_, S> {
        DoAuthUserListRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserListRequest<'a, S> {
    pub request: pb::AuthUserListRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserListRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserListRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserListResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserListResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserListRequest { request, client } = self;
        Box::pin(async move { client.user_list(request).await })
    }
}
impl pb::AuthUserDeleteRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthUserDeleteRequest<'_, S> {
        DoAuthUserDeleteRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserDeleteRequest<'a, S> {
    pub request: pb::AuthUserDeleteRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserDeleteRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserDeleteRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserDeleteResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<pb::AuthUserDeleteResponse>> + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserDeleteRequest { request, client } = self;
        Box::pin(async move { client.user_delete(request).await })
    }
}
impl pb::AuthUserChangePasswordRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthUserChangePasswordRequest<'_, S> {
        DoAuthUserChangePasswordRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserChangePasswordRequest<'a, S> {
    pub request: pb::AuthUserChangePasswordRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserChangePasswordRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
    pub fn with_password(mut self, password: String) -> Self {
        self.request.password = password;
        self
    }
    pub fn with_hashed_password(mut self, hashed_password: String) -> Self {
        self.request.hashed_password = hashed_password;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserChangePasswordRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserChangePasswordResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<pb::AuthUserChangePasswordResponse>,
                > + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserChangePasswordRequest { request, client } = self;
        Box::pin(async move { client.user_change_password(request).await })
    }
}
impl pb::AuthUserGrantRoleRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthUserGrantRoleRequest<'_, S> {
        DoAuthUserGrantRoleRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserGrantRoleRequest<'a, S> {
    pub request: pb::AuthUserGrantRoleRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserGrantRoleRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_user(mut self, user: String) -> Self {
        self.request.user = user;
        self
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserGrantRoleRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserGrantRoleResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<pb::AuthUserGrantRoleResponse>>
                + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserGrantRoleRequest { request, client } = self;
        Box::pin(async move { client.user_grant_role(request).await })
    }
}
impl pb::AuthUserRevokeRoleRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthUserRevokeRoleRequest<'_, S> {
        DoAuthUserRevokeRoleRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthUserRevokeRoleRequest<'a, S> {
    pub request: pb::AuthUserRevokeRoleRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthUserRevokeRoleRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthUserRevokeRoleRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthUserRevokeRoleResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<pb::AuthUserRevokeRoleResponse>>
                + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserRevokeRoleRequest { request, client } = self;
        Box::pin(async move { client.user_revoke_role(request).await })
    }
}
impl pb::AuthRoleAddRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthRoleAddRequest<'_, S> {
        DoAuthRoleAddRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthRoleAddRequest<'a, S> {
    pub request: pb::AuthRoleAddRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthRoleAddRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthRoleAddRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthRoleAddResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleAddResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleAddRequest { request, client } = self;
        Box::pin(async move { client.role_add(request).await })
    }
}
impl pb::AuthRoleGetRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthRoleGetRequest<'_, S> {
        DoAuthRoleGetRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthRoleGetRequest<'a, S> {
    pub request: pb::AuthRoleGetRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthRoleGetRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthRoleGetRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthRoleGetResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleGetResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleGetRequest { request, client } = self;
        Box::pin(async move { client.role_get(request).await })
    }
}
impl pb::AuthRoleListRequest {
    pub fn build<S: GrpcService>(self, client: &mut AuthClient<S>) -> DoAuthRoleListRequest<'_, S> {
        DoAuthRoleListRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthRoleListRequest<'a, S> {
    pub request: pb::AuthRoleListRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthRoleListRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthRoleListRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthRoleListResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleListResponse>> + 'a>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleListRequest { request, client } = self;
        Box::pin(async move { client.role_list(request).await })
    }
}
impl pb::AuthRoleDeleteRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthRoleDeleteRequest<'_, S> {
        DoAuthRoleDeleteRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthRoleDeleteRequest<'a, S> {
    pub request: pb::AuthRoleDeleteRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthRoleDeleteRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthRoleDeleteRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthRoleDeleteResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleDeleteResponse>> + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleDeleteRequest { request, client } = self;
        Box::pin(async move { client.role_delete(request).await })
    }
}
impl pb::AuthRoleGrantPermissionRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthRoleGrantPermissionRequest<'_, S> {
        DoAuthRoleGrantPermissionRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthRoleGrantPermissionRequest<'a, S> {
    pub request: pb::AuthRoleGrantPermissionRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthRoleGrantPermissionRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthRoleGrantPermissionRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthRoleGrantPermissionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<pb::AuthRoleGrantPermissionResponse>,
                > + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleGrantPermissionRequest { request, client } = self;
        Box::pin(async move { client.role_grant_permission(request).await })
    }
}
impl pb::AuthRoleRevokePermissionRequest {
    pub fn build<S: GrpcService>(
        self,
        client: &mut AuthClient<S>,
    ) -> DoAuthRoleRevokePermissionRequest<'_, S> {
        DoAuthRoleRevokePermissionRequest {
            request: self,
            client,
        }
    }
}
#[must_use]
pub struct DoAuthRoleRevokePermissionRequest<'a, S> {
    pub request: pb::AuthRoleRevokePermissionRequest,
    pub(crate) client: &'a mut AuthClient<S>,
}
impl<'a, S> DoAuthRoleRevokePermissionRequest<'a, S>
where
    S: GrpcService,
{
    pub fn with_client(mut self, client: &'a mut AuthClient<S>) -> Self {
        self.client = client;
        self
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
    pub fn with_key(mut self, key: Vec<u8>) -> Self {
        self.request.key = key;
        self
    }
    pub fn with_range_end(mut self, range_end: Vec<u8>) -> Self {
        self.request.range_end = range_end;
        self
    }
}
impl<'a, S> std::future::IntoFuture for DoAuthRoleRevokePermissionRequest<'a, S>
where
    S: GrpcService,
{
    type Output = Result<pb::AuthRoleRevokePermissionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<pb::AuthRoleRevokePermissionResponse>,
                > + 'a,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleRevokePermissionRequest { request, client } = self;
        Box::pin(async move { client.role_revoke_permission(request).await })
    }
}
