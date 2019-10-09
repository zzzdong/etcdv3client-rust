use http::header::HeaderValue;
use http::uri::Uri;
use tonic::transport::channel::Channel;
use tonic::transport::Endpoint;

pub(crate) const TOKEN_ID: &str = "token";

use crate::error::EtcdClientError;

fn new_endpoint(endpoint: impl AsRef<str>, token: Option<impl AsRef<str>>) -> Result<Endpoint, EtcdClientError> {
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

pub(crate) fn new_channel(
    endpoints: Vec<impl AsRef<str>>,
    token: Option<impl AsRef<str>>,
) -> Result<Channel, EtcdClientError> {
    let channel = match endpoints.len() {
        0 => {
            let endpoint = new_endpoint("", token)?;
            endpoint.channel()
        }
        1 => {
            let endpoint = new_endpoint(endpoints[0].as_ref(), token)?;
            endpoint.channel()
        }
        _ => {
            let mut eps: Vec<Endpoint> = vec![];
            for ep in endpoints {
                let endpoint = new_endpoint(ep.as_ref(), token.as_ref())?;
                eps.push(endpoint);
            }
            Channel::balance_list(eps.into_iter())
        }
    };

    Ok(channel)
}
