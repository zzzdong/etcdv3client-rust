use crate::client::Transport;
use crate::error::Result;
use crate::pb::{self, auth_client::AuthClient as PbAuthClient};
use crate::Client;

use helper::*;

#[derive(Debug, Clone)]
pub struct AuthClient {
    inner: PbAuthClient<Transport>,
}

impl AuthClient {
    pub(crate) fn new(transport: Transport) -> Self {
        let inner = PbAuthClient::new(transport);

        AuthClient { inner }
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

impl DoAuthEnableRequest {
    pub fn new(client: &AuthClient) -> Self {
        DoAuthEnableRequest {
            request: Default::default(),
            client: client.clone(),
        }
    }
}

impl DoAuthDisableRequest {
    pub fn new(client: &AuthClient) -> Self {
        DoAuthDisableRequest {
            request: Default::default(),
            client: client.clone(),
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

impl DoAuthenticateRequest {
    pub fn new(name: impl AsRef<str>, password: impl AsRef<str>, client: &AuthClient) -> Self {
        DoAuthenticateRequest {
            request: pb::AuthenticateRequest::new(name, password),
            client: client.clone(),
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

impl DoAuthUserAddRequest {
    pub fn new(
        name: impl AsRef<str>,
        password: impl AsRef<str>,
        hashed_password: impl AsRef<str>,
        client: &AuthClient,
    ) -> Self {
        DoAuthUserAddRequest {
            request: pb::AuthUserAddRequest::new(name, password, hashed_password),
            client: client.clone(),
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

impl DoAuthUserGetRequest {
    pub fn new(name: impl AsRef<str>, client: &AuthClient) -> Self {
        DoAuthUserGetRequest {
            request: pb::AuthUserGetRequest::new(name),
            client: client.clone(),
        }
    }
}

mod helper {
    #![allow(dead_code)]

    use crate::auth::AuthClient;
    use crate::error::Result;
    use crate::pb;

    include!("pb/auth_helper.rs");
}
