use http::header::HeaderValue;
use http::uri::Uri;
use tonic::transport::channel::Channel;
use tonic::transport::Endpoint;

use crate::error::EtcdClientError;

pub(crate) const TOKEN_ID: &str = "token";

fn new_endpoint(
    endpoint: impl AsRef<str>,
    token: Option<impl AsRef<str>>,
) -> Result<Endpoint, EtcdClientError> {
    let uri: Uri = endpoint.as_ref().parse()?;
    let mut endpoint = Channel::builder(uri);

    if let Some(token) = token {
        let token_val = HeaderValue::from_bytes(&token.as_ref().as_bytes().to_vec())?;

        endpoint.intercept_headers(move |headers| {
            headers.insert(TOKEN_ID, token_val.clone());
        });
    };

    Ok(endpoint)
}

pub(crate) async fn new_channel(
    endpoints: Vec<impl AsRef<str>>,
    token: Option<impl AsRef<str>>,
) -> Result<Channel, EtcdClientError> {
    if endpoints.is_empty() {
        return Err(EtcdClientError::EndpointError);
    }

    if endpoints.len() > 1 {
        let mut eps: Vec<Endpoint> = vec![];
        for ep in endpoints {
            let endpoint = new_endpoint(ep.as_ref(), token.as_ref())?;
            eps.push(endpoint);
        }
        return Ok(Channel::balance_list(eps.into_iter()));
    }

    let endpoint = new_endpoint(endpoints[0].as_ref(), token)?;
    Ok(endpoint.connect().await?)
}
