use std::time::Duration;

use tokio::time::sleep;

use etcdv3client::{EtcdClient, EtcdClientError};

#[tokio::main]
async fn main() -> Result<(), EtcdClientError> {
    let key = "hello";
    let world = "world";

    let endpoint = "http://localhost:2379";
    let auth: Option<(String, String)> = None;

    let mut client = EtcdClient::new(vec![endpoint], auth).await?;

    let start = std::time::Instant::now();
    let lease = client.grant_lease(3, 0).await?;

    let leases = client.list_leases().await?;
    println!("got all lease: {:?}", leases);

    client
        .kv
        .do_put(key, world)
        .with_lease(lease.id)
        .finish()
        .await?;
    println!("put {}-{} done at {:?}", key, world, start.elapsed());

    sleep(Duration::from_secs(1)).await;
    // after 1 second, lease still available
    let info = client.get_lease_info(lease.id, true).await?;
    println!("got lease: {:?}", info);

    let value = client.get_string(key).await;
    println!("try get {}: {:?} at {:?}", key, value, start.elapsed());

    sleep(Duration::from_secs(3)).await;
    // after 1+3 seconds, lease unavailable
    let value = client.get_string(key).await;
    println!("try get {}: {:?} at {:?}", key, value, start.elapsed());
    assert_eq!(value.is_err(), true);

    // another case with keep_alive
    let start = std::time::Instant::now();
    let lease = client.grant_lease(3, 0).await?;

    client
        .kv
        .do_put(key, world)
        .with_lease(lease.id)
        .finish()
        .await?;

    let mut aliver = client.keep_lease_alive(lease.id).await?;
    loop {
        sleep(Duration::from_secs(1)).await;

        let elapsed = start.elapsed();
        if elapsed > Duration::from_secs(6) {
            break;
        }

        if elapsed > Duration::from_secs(4) {
            let value = client.get_string(key).await;
            println!("try get {}: {:?} at {:?}", key, value, elapsed);
            assert_eq!(value.is_ok(), true);
            println!("after {:?}, {} still alive", elapsed, key);
        }
        aliver.keep_alive().await?;
    }

    Ok(())
}
