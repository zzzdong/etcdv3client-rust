use crate::error::Result;
use crate::pb::{self};
use crate::transport::GrpcService;
use crate::transport::Transport;
use crate::Client;

use helper::*;
use tonic::IntoRequest;

#[derive(Debug, Clone)]
pub struct InnerAuthClient<S> {
    transport: S,
}
impl<S> InnerAuthClient<S>
where
    S: GrpcService,
{
    pub fn new(transport: S) -> Self {
        Self { transport }
    }
    pub async fn auth_enable(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthEnableRequest>,
    ) -> Result<tonic::Response<pb::AuthEnableResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthEnable");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn auth_disable(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthDisableRequest>,
    ) -> Result<tonic::Response<pb::AuthDisableResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthDisable");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn auth_status(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthStatusRequest>,
    ) -> Result<tonic::Response<pb::AuthStatusResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/AuthStatus");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn authenticate(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthenticateRequest>,
    ) -> Result<tonic::Response<pb::AuthenticateResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/Authenticate");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_add(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserAddRequest>,
    ) -> Result<tonic::Response<pb::AuthUserAddResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserAdd");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_get(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserGetRequest>,
    ) -> Result<tonic::Response<pb::AuthUserGetResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserGet");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_list(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserListRequest>,
    ) -> Result<tonic::Response<pb::AuthUserListResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserList");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_delete(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserDeleteRequest>,
    ) -> Result<tonic::Response<pb::AuthUserDeleteResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserDelete");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_change_password(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserChangePasswordRequest>,
    ) -> Result<tonic::Response<pb::AuthUserChangePasswordResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserChangePassword");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_grant_role(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserGrantRoleRequest>,
    ) -> Result<tonic::Response<pb::AuthUserGrantRoleResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserGrantRole");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn user_revoke_role(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthUserRevokeRoleRequest>,
    ) -> Result<tonic::Response<pb::AuthUserRevokeRoleResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/UserRevokeRole");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn role_add(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleAddRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleAddResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleAdd");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn role_get(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleGetRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleGetResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleGet");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn role_list(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleListRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleListResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleList");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn role_delete(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleDeleteRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleDeleteResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleDelete");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn role_grant_permission(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleGrantPermissionRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleGrantPermissionResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleGrantPermission");
        self.transport.unary(request.into_request(), path).await
    }
    pub async fn role_revoke_permission(
        &mut self,
        request: impl tonic::IntoRequest<pb::AuthRoleRevokePermissionRequest>,
    ) -> Result<tonic::Response<pb::AuthRoleRevokePermissionResponse>> {
        let path = http::uri::PathAndQuery::from_static("/etcdserverpb.Auth/RoleRevokePermission");
        self.transport.unary(request.into_request(), path).await
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
pub struct AuthClient {
    inner: InnerAuthClient<Transport>,
}

impl AuthClient {
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

impl AuthClient {
    pub(crate) fn new(transport: Transport) -> Self {
        AuthClient {
            inner: InnerAuthClient::new(transport),
        }
    }

    pub fn with_client(client: &Client) -> Self {
        Self::new(client.transport.clone())
    }

    pub fn do_auth_enable(&mut self) -> DoAuthEnableRequest {
        DoAuthEnableRequest::new(self)
    }

    pub async fn enable_auth(&mut self) -> Result<()> {
        self.do_auth_enable().await.map(|_| ())
    }

    pub fn do_auth_disable(&mut self) -> DoAuthDisableRequest {
        DoAuthDisableRequest::new(self)
    }

    pub async fn disable_auth(&mut self) -> Result<()> {
        self.do_auth_disable().await.map(|_| ())
    }

    pub fn do_authenticate(
        &mut self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> DoAuthenticateRequest {
        DoAuthenticateRequest::new(name, password, self)
    }

    pub async fn get_token(
        &mut self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<String> {
        let resp = self.do_authenticate(name, password).await?;
        Ok(resp.token)
    }

    pub fn do_user_add(
        &mut self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> DoAuthUserAddRequest {
        DoAuthUserAddRequest::new(name, password, "", self)
    }

    pub async fn add_user(
        &mut self,
        name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<()> {
        let _resp = self.do_user_add(name, password).await?;
        Ok(())
    }

    pub fn do_user_get(&mut self, name: impl AsRef<str>) -> DoAuthUserGetRequest {
        DoAuthUserGetRequest::new(name, self)
    }

    pub async fn get_user(&mut self, name: impl AsRef<str>) -> Result<pb::AuthUserGetResponse> {
        self.do_user_get(name).await
    }
}

impl<'a> DoAuthEnableRequest<'a> {
    pub fn new(client: &'a mut AuthClient) -> Self {
        DoAuthEnableRequest {
            request: Default::default(),
            client,
        }
    }
}

impl<'a> DoAuthDisableRequest<'a> {
    pub fn new(client: &'a mut AuthClient) -> Self {
        DoAuthDisableRequest {
            request: Default::default(),
            client,
        }
    }
}

impl pb::AuthenticateRequest {
    pub fn new(name: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        pb::AuthenticateRequest {
            name: name.as_ref().to_string(),
            password: password.as_ref().to_string(),
        }
    }
}

impl<'a> DoAuthenticateRequest<'a> {
    pub fn new(
        name: impl AsRef<str>,
        password: impl AsRef<str>,
        client: &'a mut AuthClient,
    ) -> Self {
        DoAuthenticateRequest {
            request: pb::AuthenticateRequest::new(name, password),
            client,
        }
    }
}

impl pb::AuthUserAddRequest {
    pub fn new(
        name: impl AsRef<str>,
        password: impl AsRef<str>,
        hashed_password: impl AsRef<str>,
    ) -> Self {
        pb::AuthUserAddRequest {
            name: name.as_ref().to_string(),
            password: password.as_ref().to_string(),
            hashed_password: hashed_password.as_ref().to_string(),
            options: None,
        }
    }

    pub fn set_no_password(&mut self) {
        self.options = Some(pb::UserAddOptions { no_password: true });
    }
}

impl<'a> DoAuthUserAddRequest<'a> {
    pub fn new(
        name: impl AsRef<str>,
        password: impl AsRef<str>,
        hashed_password: impl AsRef<str>,
        client: &'a mut AuthClient,
    ) -> Self {
        DoAuthUserAddRequest {
            request: pb::AuthUserAddRequest::new(name, password, hashed_password),
            client,
        }
    }

    pub fn without_password(mut self) -> Self {
        self.request.set_no_password();
        self
    }
}

impl pb::AuthUserGetRequest {
    pub fn new(name: impl AsRef<str>) -> Self {
        pb::AuthUserGetRequest {
            name: name.as_ref().to_string(),
        }
    }
}

impl<'a> DoAuthUserGetRequest<'a> {
    pub fn new(name: impl AsRef<str>, client: &'a mut AuthClient) -> Self {
        DoAuthUserGetRequest {
            request: pb::AuthUserGetRequest::new(name),
            client,
        }
    }
}

pub mod helper {
    #![allow(dead_code)]

    use crate::auth::AuthClient;
    use crate::error::Result;
    use crate::pb;

    pub struct DoAuthEnableRequest<'a> {
        pub request: pb::AuthEnableRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthEnableRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthEnableRequest {
                request: Default::default(),
                client,
            }
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthEnableRequest<'a> {
        type Output = Result<pb::AuthEnableResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthEnableResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthEnableRequest { request, client } = self;
            Box::pin(async move { client.auth_enable(request).await })
        }
    }
    pub struct DoAuthDisableRequest<'a> {
        pub request: pb::AuthDisableRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthDisableRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthDisableRequest {
                request: Default::default(),
                client,
            }
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthDisableRequest<'a> {
        type Output = Result<pb::AuthDisableResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthDisableResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthDisableRequest { request, client } = self;
            Box::pin(async move { client.auth_disable(request).await })
        }
    }
    pub struct DoAuthStatusRequest<'a> {
        pub request: pb::AuthStatusRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthStatusRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthStatusRequest {
                request: Default::default(),
                client,
            }
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthStatusRequest<'a> {
        type Output = Result<pb::AuthStatusResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthStatusResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthStatusRequest { request, client } = self;
            Box::pin(async move { client.auth_status(request).await })
        }
    }
    pub struct DoAuthenticateRequest<'a> {
        pub request: pb::AuthenticateRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthenticateRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthenticateRequest {
                request: Default::default(),
                client,
            }
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
    impl<'a> std::future::IntoFuture for DoAuthenticateRequest<'a> {
        type Output = Result<pb::AuthenticateResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthenticateResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthenticateRequest { request, client } = self;
            Box::pin(async move { client.authenticate(request).await })
        }
    }
    pub struct DoAuthUserAddRequest<'a> {
        pub request: pb::AuthUserAddRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserAddRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserAddRequest {
                request: Default::default(),
                client,
            }
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
    impl<'a> std::future::IntoFuture for DoAuthUserAddRequest<'a> {
        type Output = Result<pb::AuthUserAddResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthUserAddResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserAddRequest { request, client } = self;
            Box::pin(async move { client.user_add(request).await })
        }
    }
    pub struct DoAuthUserGetRequest<'a> {
        pub request: pb::AuthUserGetRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserGetRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserGetRequest {
                request: Default::default(),
                client,
            }
        }
        pub fn with_name(mut self, name: String) -> Self {
            self.request.name = name;
            self
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthUserGetRequest<'a> {
        type Output = Result<pb::AuthUserGetResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthUserGetResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserGetRequest { request, client } = self;
            Box::pin(async move { client.user_get(request).await })
        }
    }
    pub struct DoAuthUserListRequest<'a> {
        pub request: pb::AuthUserListRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserListRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserListRequest {
                request: Default::default(),
                client,
            }
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthUserListRequest<'a> {
        type Output = Result<pb::AuthUserListResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthUserListResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserListRequest { request, client } = self;
            Box::pin(async move { client.user_list(request).await })
        }
    }
    pub struct DoAuthUserDeleteRequest<'a> {
        pub request: pb::AuthUserDeleteRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserDeleteRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserDeleteRequest {
                request: Default::default(),
                client,
            }
        }
        pub fn with_name(mut self, name: String) -> Self {
            self.request.name = name;
            self
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthUserDeleteRequest<'a> {
        type Output = Result<pb::AuthUserDeleteResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthUserDeleteResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserDeleteRequest { request, client } = self;
            Box::pin(async move { client.user_delete(request).await })
        }
    }
    pub struct DoAuthUserChangePasswordRequest<'a> {
        pub request: pb::AuthUserChangePasswordRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserChangePasswordRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserChangePasswordRequest {
                request: Default::default(),
                client,
            }
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
    impl<'a> std::future::IntoFuture for DoAuthUserChangePasswordRequest<'a> {
        type Output = Result<pb::AuthUserChangePasswordResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = crate::error::Result<pb::AuthUserChangePasswordResponse>,
                    > + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserChangePasswordRequest { request, client } = self;
            Box::pin(async move { client.user_change_password(request).await })
        }
    }
    pub struct DoAuthUserGrantRoleRequest<'a> {
        pub request: pb::AuthUserGrantRoleRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserGrantRoleRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserGrantRoleRequest {
                request: Default::default(),
                client,
            }
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
    impl<'a> std::future::IntoFuture for DoAuthUserGrantRoleRequest<'a> {
        type Output = Result<pb::AuthUserGrantRoleResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = crate::error::Result<pb::AuthUserGrantRoleResponse>,
                    > + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserGrantRoleRequest { request, client } = self;
            Box::pin(async move { client.user_grant_role(request).await })
        }
    }
    pub struct DoAuthUserRevokeRoleRequest<'a> {
        pub request: pb::AuthUserRevokeRoleRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthUserRevokeRoleRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthUserRevokeRoleRequest {
                request: Default::default(),
                client,
            }
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
    impl<'a> std::future::IntoFuture for DoAuthUserRevokeRoleRequest<'a> {
        type Output = Result<pb::AuthUserRevokeRoleResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = crate::error::Result<pb::AuthUserRevokeRoleResponse>,
                    > + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthUserRevokeRoleRequest { request, client } = self;
            Box::pin(async move { client.user_revoke_role(request).await })
        }
    }
    pub struct DoAuthRoleAddRequest<'a> {
        pub request: pb::AuthRoleAddRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthRoleAddRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthRoleAddRequest {
                request: Default::default(),
                client,
            }
        }
        pub fn with_name(mut self, name: String) -> Self {
            self.request.name = name;
            self
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthRoleAddRequest<'a> {
        type Output = Result<pb::AuthRoleAddResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleAddResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthRoleAddRequest { request, client } = self;
            Box::pin(async move { client.role_add(request).await })
        }
    }
    pub struct DoAuthRoleGetRequest<'a> {
        pub request: pb::AuthRoleGetRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthRoleGetRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthRoleGetRequest {
                request: Default::default(),
                client,
            }
        }
        pub fn with_role(mut self, role: String) -> Self {
            self.request.role = role;
            self
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthRoleGetRequest<'a> {
        type Output = Result<pb::AuthRoleGetResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleGetResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthRoleGetRequest { request, client } = self;
            Box::pin(async move { client.role_get(request).await })
        }
    }
    pub struct DoAuthRoleListRequest<'a> {
        pub request: pb::AuthRoleListRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthRoleListRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthRoleListRequest {
                request: Default::default(),
                client,
            }
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthRoleListRequest<'a> {
        type Output = Result<pb::AuthRoleListResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleListResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthRoleListRequest { request, client } = self;
            Box::pin(async move { client.role_list(request).await })
        }
    }
    pub struct DoAuthRoleDeleteRequest<'a> {
        pub request: pb::AuthRoleDeleteRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthRoleDeleteRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthRoleDeleteRequest {
                request: Default::default(),
                client,
            }
        }
        pub fn with_role(mut self, role: String) -> Self {
            self.request.role = role;
            self
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthRoleDeleteRequest<'a> {
        type Output = Result<pb::AuthRoleDeleteResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleDeleteResponse>>
                    + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthRoleDeleteRequest { request, client } = self;
            Box::pin(async move { client.role_delete(request).await })
        }
    }
    pub struct DoAuthRoleGrantPermissionRequest<'a> {
        pub request: pb::AuthRoleGrantPermissionRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthRoleGrantPermissionRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthRoleGrantPermissionRequest {
                request: Default::default(),
                client,
            }
        }
        pub fn with_name(mut self, name: String) -> Self {
            self.request.name = name;
            self
        }
    }
    impl<'a> std::future::IntoFuture for DoAuthRoleGrantPermissionRequest<'a> {
        type Output = Result<pb::AuthRoleGrantPermissionResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = crate::error::Result<pb::AuthRoleGrantPermissionResponse>,
                    > + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthRoleGrantPermissionRequest { request, client } = self;
            Box::pin(async move { client.role_grant_permission(request).await })
        }
    }
    pub struct DoAuthRoleRevokePermissionRequest<'a> {
        pub request: pb::AuthRoleRevokePermissionRequest,
        pub(crate) client: &'a mut AuthClient,
    }
    impl<'a> DoAuthRoleRevokePermissionRequest<'a> {
        pub fn from_client(client: &'a mut AuthClient) -> Self {
            DoAuthRoleRevokePermissionRequest {
                request: Default::default(),
                client,
            }
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
    impl<'a> std::future::IntoFuture for DoAuthRoleRevokePermissionRequest<'a> {
        type Output = Result<pb::AuthRoleRevokePermissionResponse>;
        type IntoFuture = std::pin::Pin<
            Box<
                dyn std::future::Future<
                        Output = crate::error::Result<pb::AuthRoleRevokePermissionResponse>,
                    > + Send
                    + 'a,
            >,
        >;
        fn into_future(self) -> Self::IntoFuture {
            let DoAuthRoleRevokePermissionRequest { request, client } = self;
            Box::pin(async move { client.role_revoke_permission(request).await })
        }
    }
}
