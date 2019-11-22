use etcdv3client::{EtcdClient, EtcdClientError, KeyValue, RangeRequest};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    env_logger::init();

    let key = "hello";
    let prefix = "hello";
    let value = "world";

    let endpoint = "http://localhost:2379";

    let name = "root";
    let password = "123456";

    let cred: Option<(String, String)> = None;
    

    let mut client = EtcdClient::new(vec![endpoint], cred).await?;

    match client.get(&key).await {
        Ok(v) => {
            println!("got {} => {:?}", key, String::from_utf8_lossy(&v));
        }
        Err(EtcdClientError::KeyNotFound) => {
            client.put(key, value).await.unwrap();
        }
        Err(e) => {
            return Err(e);
        }
    }

    let req = RangeRequest {
                key: key.as_bytes().to_vec(),
                ..Default::default()
            };
    let resp = client.kv.range(req).await?;

    println!("got resp => {:?}", resp);

    


    Ok(())
}
