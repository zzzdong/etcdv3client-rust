use etcdv3client::{EtcdClient, EtcdClientError};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    let endpoint = "http://localhost:2379";
    let auth: Option<(String, String)> = None;
    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

    let key = "/hello";
    // use convenience api under EtcdClient.
    match client.get(key).await {
        Ok(v) => {
            println!("got `{}` => {:?}", key, String::from_utf8_lossy(&v));
        }
        Err(EtcdClientError::KeyNotFound) => {
            eprintln!("can not find `{}`", key);
            client.kv.do_put(key, "world").await?;
            println!("put hello done");
        }
        Err(e) => {
            eprintln!("etcd get error: `{:?}`", e);
        }
    }

    Ok(())
}
