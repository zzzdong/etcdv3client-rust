use etcdv3client::{Error, EtcdClient, WatchClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let endpoint = "http://127.0.0.1:2379";
    let auth: (String, String) = ("root".to_string(), "123456".to_string());
    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

    let start = std::time::Instant::now();

    let client_cloned = client.clone();

    for i in 0..100 {
        print!("round[{}] <{:?}>, ", i, start.elapsed());

        let key = "/hello";
        // use convenience api under Client.
        match client.get_string(key).await {
            Ok(v) => {
                println!("got `{}` => {:?}", key, v);
            }
            Err(e) if e.is_key_not_found() => {
                println!("can not find `{}`", key);
            }
            Err(e) => {
                println!("etcd get error: `{:?}`", e);
            }
        }

        let new_client = client_cloned.clone();
        tokio::spawn(async move {
            watch(new_client, i).await;
        });

        tokio::time::sleep(std::time::Duration::from_secs(i * 3)).await;
    }

    Ok(())
}

async fn watch(client: EtcdClient, id: u64) {
    let mut watch_client = WatchClient::new(client.service());
    match watch_client.watch_prefix("/").await {
        Ok(mut watcher) => {
            println!("[{}] etcd created wather.", id);

            while let Ok(msg) = watcher.message().await {
                println!("[{}] etcd watch message: `{:?}`", id, msg);
            }
        }
        Err(e) => {
            println!("[{}] etcd watch error: `{:?}`", id, e);
        }
    }
}
