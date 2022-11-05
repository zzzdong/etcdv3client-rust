impl AuthClient {
    pub async fn auth_enable(
        &mut self,
        request: pb::AuthEnableRequest,
    ) -> Result<pb::AuthEnableResponse> {
        Ok(self.inner.auth_enable(request).await?.into_inner())
    }
    pub async fn auth_disable(
        &mut self,
        request: pb::AuthDisableRequest,
    ) -> Result<pb::AuthDisableResponse> {
        Ok(self.inner.auth_disable(request).await?.into_inner())
    }
    pub async fn auth_status(
        &mut self,
        request: pb::AuthStatusRequest,
    ) -> Result<pb::AuthStatusResponse> {
        Ok(self.inner.auth_status(request).await?.into_inner())
    }
    pub async fn authenticate(
        &mut self,
        request: pb::AuthenticateRequest,
    ) -> Result<pb::AuthenticateResponse> {
        Ok(self.inner.authenticate(request).await?.into_inner())
    }
    pub async fn user_add(
        &mut self,
        request: pb::AuthUserAddRequest,
    ) -> Result<pb::AuthUserAddResponse> {
        Ok(self.inner.user_add(request).await?.into_inner())
    }
    pub async fn user_get(
        &mut self,
        request: pb::AuthUserGetRequest,
    ) -> Result<pb::AuthUserGetResponse> {
        Ok(self.inner.user_get(request).await?.into_inner())
    }
    pub async fn user_list(
        &mut self,
        request: pb::AuthUserListRequest,
    ) -> Result<pb::AuthUserListResponse> {
        Ok(self.inner.user_list(request).await?.into_inner())
    }
    pub async fn user_delete(
        &mut self,
        request: pb::AuthUserDeleteRequest,
    ) -> Result<pb::AuthUserDeleteResponse> {
        Ok(self.inner.user_delete(request).await?.into_inner())
    }
    pub async fn user_change_password(
        &mut self,
        request: pb::AuthUserChangePasswordRequest,
    ) -> Result<pb::AuthUserChangePasswordResponse> {
        Ok(self.inner.user_change_password(request).await?.into_inner())
    }
    pub async fn user_grant_role(
        &mut self,
        request: pb::AuthUserGrantRoleRequest,
    ) -> Result<pb::AuthUserGrantRoleResponse> {
        Ok(self.inner.user_grant_role(request).await?.into_inner())
    }
    pub async fn user_revoke_role(
        &mut self,
        request: pb::AuthUserRevokeRoleRequest,
    ) -> Result<pb::AuthUserRevokeRoleResponse> {
        Ok(self.inner.user_revoke_role(request).await?.into_inner())
    }
    pub async fn role_add(
        &mut self,
        request: pb::AuthRoleAddRequest,
    ) -> Result<pb::AuthRoleAddResponse> {
        Ok(self.inner.role_add(request).await?.into_inner())
    }
    pub async fn role_get(
        &mut self,
        request: pb::AuthRoleGetRequest,
    ) -> Result<pb::AuthRoleGetResponse> {
        Ok(self.inner.role_get(request).await?.into_inner())
    }
    pub async fn role_list(
        &mut self,
        request: pb::AuthRoleListRequest,
    ) -> Result<pb::AuthRoleListResponse> {
        Ok(self.inner.role_list(request).await?.into_inner())
    }
    pub async fn role_delete(
        &mut self,
        request: pb::AuthRoleDeleteRequest,
    ) -> Result<pb::AuthRoleDeleteResponse> {
        Ok(self.inner.role_delete(request).await?.into_inner())
    }
    pub async fn role_grant_permission(
        &mut self,
        request: pb::AuthRoleGrantPermissionRequest,
    ) -> Result<pb::AuthRoleGrantPermissionResponse> {
        Ok(self
            .inner
            .role_grant_permission(request)
            .await?
            .into_inner())
    }
    pub async fn role_revoke_permission(
        &mut self,
        request: pb::AuthRoleRevokePermissionRequest,
    ) -> Result<pb::AuthRoleRevokePermissionResponse> {
        Ok(self
            .inner
            .role_revoke_permission(request)
            .await?
            .into_inner())
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthEnableRequest {
    pub request: pb::AuthEnableRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthEnableRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthEnableRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}
impl std::future::IntoFuture for DoAuthEnableRequest {
    type Output = Result<pb::AuthEnableResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthEnableResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthEnableRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.auth_enable(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthDisableRequest {
    pub request: pb::AuthDisableRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthDisableRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthDisableRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}
impl std::future::IntoFuture for DoAuthDisableRequest {
    type Output = Result<pb::AuthDisableResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthDisableResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthDisableRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.auth_disable(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthStatusRequest {
    pub request: pb::AuthStatusRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthStatusRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthStatusRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}
impl std::future::IntoFuture for DoAuthStatusRequest {
    type Output = Result<pb::AuthStatusResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthStatusResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthStatusRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.auth_status(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthenticateRequest {
    pub request: pb::AuthenticateRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthenticateRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthenticateRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoAuthenticateRequest {
    type Output = Result<pb::AuthenticateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthenticateResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthenticateRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.authenticate(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserAddRequest {
    pub request: pb::AuthUserAddRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserAddRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserAddRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoAuthUserAddRequest {
    type Output = Result<pb::AuthUserAddResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserAddResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserAddRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_add(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserGetRequest {
    pub request: pb::AuthUserGetRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserGetRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserGetRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl std::future::IntoFuture for DoAuthUserGetRequest {
    type Output = Result<pb::AuthUserGetResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserGetResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserGetRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_get(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserListRequest {
    pub request: pb::AuthUserListRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserListRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserListRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}
impl std::future::IntoFuture for DoAuthUserListRequest {
    type Output = Result<pb::AuthUserListResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserListResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserListRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_list(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserDeleteRequest {
    pub request: pb::AuthUserDeleteRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserDeleteRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserDeleteRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl std::future::IntoFuture for DoAuthUserDeleteRequest {
    type Output = Result<pb::AuthUserDeleteResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserDeleteResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserDeleteRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_delete(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserChangePasswordRequest {
    pub request: pb::AuthUserChangePasswordRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserChangePasswordRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserChangePasswordRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoAuthUserChangePasswordRequest {
    type Output = Result<pb::AuthUserChangePasswordResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = crate::error::Result<pb::AuthUserChangePasswordResponse>,
            >,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserChangePasswordRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_change_password(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserGrantRoleRequest {
    pub request: pb::AuthUserGrantRoleRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserGrantRoleRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserGrantRoleRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoAuthUserGrantRoleRequest {
    type Output = Result<pb::AuthUserGrantRoleResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserGrantRoleResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserGrantRoleRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_grant_role(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthUserRevokeRoleRequest {
    pub request: pb::AuthUserRevokeRoleRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthUserRevokeRoleRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthUserRevokeRoleRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoAuthUserRevokeRoleRequest {
    type Output = Result<pb::AuthUserRevokeRoleResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthUserRevokeRoleResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthUserRevokeRoleRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.user_revoke_role(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthRoleAddRequest {
    pub request: pb::AuthRoleAddRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthRoleAddRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthRoleAddRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl std::future::IntoFuture for DoAuthRoleAddRequest {
    type Output = Result<pb::AuthRoleAddResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleAddResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleAddRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.role_add(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthRoleGetRequest {
    pub request: pb::AuthRoleGetRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthRoleGetRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthRoleGetRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
impl std::future::IntoFuture for DoAuthRoleGetRequest {
    type Output = Result<pb::AuthRoleGetResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleGetResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleGetRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.role_get(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthRoleListRequest {
    pub request: pb::AuthRoleListRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthRoleListRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthRoleListRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}
impl std::future::IntoFuture for DoAuthRoleListRequest {
    type Output = Result<pb::AuthRoleListResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleListResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleListRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.role_list(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthRoleDeleteRequest {
    pub request: pb::AuthRoleDeleteRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthRoleDeleteRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthRoleDeleteRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
impl std::future::IntoFuture for DoAuthRoleDeleteRequest {
    type Output = Result<pb::AuthRoleDeleteResponse>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<pb::AuthRoleDeleteResponse>>>,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleDeleteRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.role_delete(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthRoleGrantPermissionRequest {
    pub request: pb::AuthRoleGrantPermissionRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthRoleGrantPermissionRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthRoleGrantPermissionRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
impl std::future::IntoFuture for DoAuthRoleGrantPermissionRequest {
    type Output = Result<pb::AuthRoleGrantPermissionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = crate::error::Result<pb::AuthRoleGrantPermissionResponse>,
            >,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleGrantPermissionRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.role_grant_permission(request).await })
    }
}
#[derive(Debug, Clone)]
pub struct DoAuthRoleRevokePermissionRequest {
    pub request: pb::AuthRoleRevokePermissionRequest,
    pub(crate) client: AuthClient,
}
impl DoAuthRoleRevokePermissionRequest {
    pub fn from_client(client: &AuthClient) -> Self {
        DoAuthRoleRevokePermissionRequest {
            request: Default::default(),
            client: client.clone(),
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
impl std::future::IntoFuture for DoAuthRoleRevokePermissionRequest {
    type Output = Result<pb::AuthRoleRevokePermissionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                Output = crate::error::Result<pb::AuthRoleRevokePermissionResponse>,
            >,
        >,
    >;
    fn into_future(self) -> Self::IntoFuture {
        let DoAuthRoleRevokePermissionRequest {
            request,
            mut client,
        } = self;
        Box::pin(async move { client.role_revoke_permission(request).await })
    }
}
