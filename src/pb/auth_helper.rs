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
    pub async fn finish(self) -> Result<pb::AuthEnableResponse> {
        let DoAuthEnableRequest { request, client } = self;
        client.auth_enable(request).await
    }
}
pub struct DoAuthDisableRequest<'a> {
    pub request: pb::AuthDisableRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthDisableRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthDisableResponse> {
        let DoAuthDisableRequest { request, client } = self;
        client.auth_disable(request).await
    }
}
pub struct DoAuthenticateRequest<'a> {
    pub request: pb::AuthenticateRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthenticateRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthenticateResponse> {
        let DoAuthenticateRequest { request, client } = self;
        client.authenticate(request).await
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
pub struct DoAuthUserAddRequest<'a> {
    pub request: pb::AuthUserAddRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserAddRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserAddResponse> {
        let DoAuthUserAddRequest { request, client } = self;
        client.user_add(request).await
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
pub struct DoAuthUserGetRequest<'a> {
    pub request: pb::AuthUserGetRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserGetRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserGetResponse> {
        let DoAuthUserGetRequest { request, client } = self;
        client.user_get(request).await
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
pub struct DoAuthUserListRequest<'a> {
    pub request: pb::AuthUserListRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserListRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserListResponse> {
        let DoAuthUserListRequest { request, client } = self;
        client.user_list(request).await
    }
}
pub struct DoAuthUserDeleteRequest<'a> {
    pub request: pb::AuthUserDeleteRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserDeleteRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserDeleteResponse> {
        let DoAuthUserDeleteRequest { request, client } = self;
        client.user_delete(request).await
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
pub struct DoAuthUserChangePasswordRequest<'a> {
    pub request: pb::AuthUserChangePasswordRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserChangePasswordRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserChangePasswordResponse> {
        let DoAuthUserChangePasswordRequest { request, client } = self;
        client.user_change_password(request).await
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
pub struct DoAuthUserGrantRoleRequest<'a> {
    pub request: pb::AuthUserGrantRoleRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserGrantRoleRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserGrantRoleResponse> {
        let DoAuthUserGrantRoleRequest { request, client } = self;
        client.user_grant_role(request).await
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
pub struct DoAuthUserRevokeRoleRequest<'a> {
    pub request: pb::AuthUserRevokeRoleRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthUserRevokeRoleRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthUserRevokeRoleResponse> {
        let DoAuthUserRevokeRoleRequest { request, client } = self;
        client.user_revoke_role(request).await
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
pub struct DoAuthRoleAddRequest<'a> {
    pub request: pb::AuthRoleAddRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthRoleAddRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthRoleAddResponse> {
        let DoAuthRoleAddRequest { request, client } = self;
        client.role_add(request).await
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
pub struct DoAuthRoleGetRequest<'a> {
    pub request: pb::AuthRoleGetRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthRoleGetRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthRoleGetResponse> {
        let DoAuthRoleGetRequest { request, client } = self;
        client.role_get(request).await
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
pub struct DoAuthRoleListRequest<'a> {
    pub request: pb::AuthRoleListRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthRoleListRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthRoleListResponse> {
        let DoAuthRoleListRequest { request, client } = self;
        client.role_list(request).await
    }
}
pub struct DoAuthRoleDeleteRequest<'a> {
    pub request: pb::AuthRoleDeleteRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthRoleDeleteRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthRoleDeleteResponse> {
        let DoAuthRoleDeleteRequest { request, client } = self;
        client.role_delete(request).await
    }
    pub fn with_role(mut self, role: String) -> Self {
        self.request.role = role;
        self
    }
}
pub struct DoAuthRoleGrantPermissionRequest<'a> {
    pub request: pb::AuthRoleGrantPermissionRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthRoleGrantPermissionRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthRoleGrantPermissionResponse> {
        let DoAuthRoleGrantPermissionRequest { request, client } = self;
        client.role_grant_permission(request).await
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }
}
pub struct DoAuthRoleRevokePermissionRequest<'a> {
    pub request: pb::AuthRoleRevokePermissionRequest,
    pub(crate) client: &'a mut AuthClient,
}
impl<'a> DoAuthRoleRevokePermissionRequest<'a> {
    pub async fn finish(self) -> Result<pb::AuthRoleRevokePermissionResponse> {
        let DoAuthRoleRevokePermissionRequest { request, client } = self;
        client.role_revoke_permission(request).await
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
