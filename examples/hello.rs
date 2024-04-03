use std::time::Instant;

use etcdv3client::{pb::RangeRequest, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = "/hello";
    let value = "world";
    let mut origin: Option<Vec<u8>> = None;

    let endpoint = "http://localhost:2379";
    let cred = None;

    let mut client = Client::new(vec![endpoint], cred).await?;

    // use convenience api under Client.
    match client.get(key).await {
        Ok(v) => {
            println!("got orignal {} => {:?}", key, String::from_utf8_lossy(&v));
            origin = Some(v.to_vec());
        }

        Err(err) => {
            if err.is_key_not_found() {
                client.put(key, value).await.unwrap();
            } else {
                return Err(err);
            }
        }
    }

    // use raw etcd grpc api.
    let req = RangeRequest {
        key: key.as_bytes().to_vec(),
        ..Default::default()
    };
    let resp = client.kv.range(req).await?;
    println!("got range resp => {:?}", resp);

    let start = Instant::now();
    // try watch api.
    let mut watcher = client.watch(key).await?;

    // use middle level api.
    client.kv.do_put(key, value).await?;

    // read the watch event
    let mut n: i32 = 0;
    while let Some(w) = watcher.message().await? {
        println!("[{:?}] [{}] watch got => {:?}", start.elapsed(), n, w);

        if w.canceled {
            println!("[{:?}] watch cancaled, exit...", start.elapsed());
            break;
        }

        let v = format!("{}-{}", value, n);
        client.put(key, v.clone()).await?;
        println!("put value done: {}", v);

        if n > 1 {
            watcher.cancel().await?;
            println!("[{:?}] sent cancel at {}", start.elapsed(), n);
        }

        n += 1;
    }

    // restore etcd status
    match origin {
        Some(orig) => {
            client.put(key, orig).await?;
        }
        None => {
            client.delete(key).await?;
        }
    }

    Ok(())
}
