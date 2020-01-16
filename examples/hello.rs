use etcdv3client::{pb::RangeRequest, EtcdClient, EtcdClientError};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    let key = "hello";
    let value = "world";

    let endpoint = "http://localhost:2379";
    let auth: Option<(String, String)> = None;

    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

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

    // let mut watcher = client.watch(key).await?;

    // tokio::spawn(async move {
    //     for i in 0..5u8 {
    //         let v = format!("{}-{}", value, i);
    //         client.put(key, &v).await.unwrap();

    //         println!("[{}] put {} done", i, v);
    //     }
    // });

    // let mut n: i32 = 0;
    // while let Some(w) = watcher.message().await.unwrap() {
    //     println!("[{}] watch got => {:?}", n, w);

    //     if w.canceled {
    //         println!("watch cancaled, exit...");
    //         break;
    //     }

    //     if n > 2 {
    //         watcher.cancel().await.unwrap();
    //     }
    //     n += 1;
    // }

    Ok(())
}
