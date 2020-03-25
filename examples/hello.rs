use etcdv3client::{pb::RangeRequest, EtcdClient, EtcdClientError};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    let key = "hello";
    let value = "world";
    let mut origin: Option<Vec<u8>> = None;

    let endpoint = "http://localhost:2379";
    let auth: Option<(String, String)> = None;

    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

    // use convenience api under EtcdClient.
    match client.get(&key).await {
        Ok(v) => {
            println!("got orignal {} => {:?}", key, String::from_utf8_lossy(&v));
            origin = Some(v.to_vec());
        }
        Err(EtcdClientError::KeyNotFound) => {
            client.put(key, value).await.unwrap();
        }
        Err(e) => {
            return Err(e);
        }
    }

    // use raw etcd grpc api.
    let req = RangeRequest {
        key: key.as_bytes().to_vec(),
        ..Default::default()
    };
    let resp = client.kv.range(req).await?;
    println!("got range resp => {:?}", resp);

    // try watch api.
    let mut watcher = client.watch(key).await?;

    // use middle level api.
    client.kv.do_put(key, &value).finish().await?;

    // read the watch event
    let mut n: i32 = 0;
    while let Some(w) = watcher.message().await? {
        println!("[{}] watch got => {:?}", n, w);

        if w.canceled {
            println!("watch cancaled, exit...");
            break;
        }

        let v = format!("{}-{}", value, n);
        client.put(key, &v).await?;
        println!("put value: {}", v);

        if n > 1 {
            watcher.cancel().await?;
            println!("sent cancel at {}", n);
        }

        n += 1;
    }

    // restore etcd status
    match origin {
        Some(ref orig) => {
            client.put(key, orig).await?;
        }
        None => {
            client.delete(key).await?;
        }
    }

    Ok(())
}
