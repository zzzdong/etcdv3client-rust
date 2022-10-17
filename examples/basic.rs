use etcdv3client::{Error, EtcdClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let endpoint = "http://localhost:2379";
    let cred: Option<(String, String)> = None;
    let mut client = EtcdClient::new(vec![endpoint], cred).await?;

    let key = "/hello";
    // use convenience api under EtcdClient.
    match client.get(key).await {
        Ok(v) => {
            println!("got `{}` => {:?}", key, String::from_utf8_lossy(&v));
        }
        Err(err) => {
            if err.is_key_not_found() {
                println!("can not find `{}`", key);
                client.kv.do_put(key, "world").await?;
                println!("put hello done");
            } else {
                eprintln!("etcd get failed: `{:?}`", err);
            }
        }
    }

    Ok(())
}
