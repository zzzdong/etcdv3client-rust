use http::uri::Uri;
use tonic::transport::channel::Channel;
use tonic::transport::Endpoint;

use crate::error::EtcdClientError;

fn new_endpoint(endpoint: impl AsRef<str>) -> Result<Endpoint, EtcdClientError> {
    let uri: Uri = endpoint.as_ref().parse()?;
    Ok(Channel::builder(uri))
}

pub(crate) async fn new_channel(
    endpoints: Vec<impl AsRef<str>>,
) -> Result<Channel, EtcdClientError> {
    if endpoints.is_empty() {
        return Err(EtcdClientError::EndpointError);
    }

    if endpoints.len() > 1 {
        let mut eps: Vec<Endpoint> = vec![];
        for ep in endpoints {
            let endpoint = new_endpoint(ep.as_ref())?;
            eps.push(endpoint);
        }
        return Ok(Channel::balance_list(eps.into_iter()));
    }

    let endpoint = new_endpoint(endpoints[0].as_ref())?;
    Ok(endpoint.connect().await?)
}
