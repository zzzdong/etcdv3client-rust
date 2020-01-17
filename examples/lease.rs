use std::time::Duration;

use tokio::time::delay_for;

use etcdv3client::{EtcdClient, EtcdClientError};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    let key = "hello";
    let value = "world";

    let endpoint = "http://localhost:2379";
    let auth: Option<(String, String)> = None;

    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

    let start = std::time::Instant::now();
    let lease = client.grant_lease(3, 0).await?;

    let leases = client.list_leases().await?;
    println!("got all lease: {:?}", leases);

    client
        .kv
        .do_put(key, value)
        .with_lease(lease.id)
        .finish()
        .await?;

    println!("put {}-{} done at {:?}", key, value, start.elapsed());

    let mut value;
    loop {
        delay_for(Duration::from_secs(1)).await;

        let info = client.get_lease_info(lease.id, true).await?;
        println!("got lease: {:?}", info);

        value = client.get_string(key).await;
        println!("try get {}: {:?} at {:?}", key, value, start.elapsed());

        if start.elapsed() > Duration::from_secs(4) {
            assert_eq!(value.is_err(), true);
            if let Err(EtcdClientError::KeyNotFound) = value {
                println!("lease done");
                break;
            }
        }
    }

    Ok(())
}
