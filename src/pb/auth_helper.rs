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
            dyn std::future::Future<Output = crate::error::Result<pb::AuthUserGrantRoleResponse>>
                + Send
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
            dyn std::future::Future<Output = crate::error::Result<pb::AuthUserRevokeRoleResponse>>
                + Send
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
